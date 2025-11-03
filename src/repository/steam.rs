use crate::model::steam::{
    CreateSteamAppGamePayload, SteamApiResponse, SteamGame, SteamPriceResponse,
};
use axum::{extract::Path, response::IntoResponse, Json};
use sqlx::postgres::PgRow;
use sqlx::{query, Error, PgPool, Row};

pub async fn get_steam_price_handler(Path(appid): Path<u32>) -> impl IntoResponse {
    // Local typed structs to deserialize Steam API

    let url = format!(
        "https://store.steampowered.com/api/appdetails?appids={}&cc=th&l=th",
        appid
    );

    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(_) => {
            return (
                axum::http::StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({ "error": "Failed to fetch from Steam" })),
            )
        }
    };

    if !resp.status().is_success() {
        return (
            axum::http::StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({ "error": "Steam returned bad status" })),
        );
    }

    let map: std::collections::HashMap<String, SteamApiResponse> = match resp.json().await {
        Ok(m) => m,
        Err(_) => {
            return (
                axum::http::StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({ "error": "Invalid response from Steam" })),
            )
        }
    };

    let key = appid.to_string();
    let Some(entry) = map.get(&key) else {
        return (
            axum::http::StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({ "error": "Steam response missing app data" })),
        );
    };

    if !entry.success {
        return (
            axum::http::StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Game not found" })),
        );
    }

    let Some(data) = entry.data.as_ref() else {
        return (
            axum::http::StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Game not found" })),
        );
    };

    let name = data.name.clone();
    let image = data.header_image.clone();
    let (price, currency, discount) = if let Some(p) = &data.price {
        let price_opt = p.final_.map(|v| (v as f64) / 100.0);
        (price_opt, p.currency.clone(), p.discount_percent)
    } else {
        (None, None, None)
    };

    let out = SteamPriceResponse {
        app_id: appid,
        name,
        price,
        currency,
        discount,
        image,
    };

    (axum::http::StatusCode::OK, Json(serde_json::json!(out)))
}

pub async fn create_steam_game_handler(
    pool: &PgPool,
    user_id: i64,
    payload: CreateSteamAppGamePayload,
) -> Result<SteamGame, Error> {
    let row: PgRow = query(
        r#"
        INSERT INTO steam_app_games (app_id, name, steam_db_url, is_buy, user_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        RETURNING id
        "#
    )
    .bind(payload.app_id as i64)
    .bind(&payload.name)
    .bind(&payload.steam_db_url)
    .bind(false)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    let new_steam_id: i64 = row.try_get("id")?;
    get_steam_game(pool, new_steam_id, user_id).await
}

pub async fn get_steam_game(
    pool: &PgPool,
    id: i64,
    user_id: i64,
) -> Result<SteamGame, sqlx::Error> {
    let row: PgRow = query(
        r#"
        SELECT
            id,
            app_id,
            steam_db_url,
            name,
            is_buy,
            user_id,
            (created_at AT TIME ZONE 'UTC') as created_at,
            (updated_at AT TIME ZONE 'UTC') as updated_at
        FROM steam_app_games
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    let steam = SteamGame {
        id: row.try_get("id")?,
        app_id: row.try_get("app_id")?,
        name: row.try_get("name")?,
        steam_db_url: row.try_get("steam_db_url")?,
        is_buy: row.try_get("is_buy")?,
        user_id: row.try_get("user_id")?,
        created_at: row.try_get::<chrono::NaiveDateTime, _>("created_at")?,
        updated_at: row.try_get::<chrono::NaiveDateTime, _>("updated_at")?,
    };

    Ok(steam)
}

pub async fn list_steam_games(pool: &PgPool, user_id: i64) -> Result<Vec<SteamGame>, Error> {
    let rows = query(
        r#"
        SELECT
            id,
            app_id,
            name,
            steam_db_url,
            is_buy,
            user_id,
            (created_at AT TIME ZONE 'UTC') as created_at,
            (updated_at AT TIME ZONE 'UTC') as updated_at
        FROM steam_app_games
        WHERE user_id = $1
        ORDER BY id DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let items = rows
        .into_iter()
        .map(|row: PgRow| SteamGame {
            id: row.try_get("id").unwrap_or_default(),
            app_id: row.try_get("app_id").unwrap_or_default(),
            name: row.try_get("name").unwrap_or_default(),
            steam_db_url: row.try_get("steam_db_url").unwrap_or_default(),
            is_buy: row.try_get("is_buy").unwrap_or(false),
            user_id: row.try_get("user_id").unwrap_or_default(),
            created_at: row.try_get("created_at").unwrap(),
            updated_at: row.try_get("updated_at").unwrap(),
        })
        .collect();

    Ok(items)
}

pub async fn update_steam_game(
    pool: &PgPool,
    id: i64,
    user_id: i64,
    payload: crate::model::steam::UpdateSteamAppGamePayload,
) -> Result<SteamGame, Error> {
    let row: PgRow = query(
        r#"
        UPDATE steam_app_games
        SET
            name = COALESCE($1, name),
            steam_db_url = COALESCE($2, steam_db_url),
            is_buy = COALESCE($3, is_buy),
            updated_at = NOW()
        WHERE id = $4 AND user_id = $5
        RETURNING
            id, app_id, name, steam_db_url, is_buy, user_id,
            (created_at AT TIME ZONE 'UTC') as created_at,
            (updated_at AT TIME ZONE 'UTC') as updated_at
        "#,
    )
    .bind(payload.name.as_ref())
    .bind(payload.steam_db_url.as_ref())
    .bind(payload.is_buy)
    .bind(id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    let item = SteamGame {
        id: row.try_get("id")?,
        app_id: row.try_get("app_id")?,
        name: row.try_get("name")?,
        steam_db_url: row.try_get("steam_db_url")?,
        is_buy: row.try_get("is_buy")?,
        user_id: row.try_get("user_id")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    };

    Ok(item)
}

pub async fn delete_steam_game(pool: &PgPool, id: i64, user_id: i64) -> Result<(), Error> {
    let res = query(
        r#"
        DELETE FROM steam_app_games
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?;

    if res.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }
    Ok(())
}
