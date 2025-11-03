use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::model::steam::{CreateSteamAppGamePayload, UpdateSteamAppGamePayload};
use crate::repository::steam::{
    create_steam_game_handler, delete_steam_game, get_steam_game, get_steam_price_handler,
    list_steam_games, update_steam_game,
};
use crate::state::AppState;
use crate::{middleware::auth::AuthUser, model::steam::SteamGame};
use sqlx::Error as SqlxError;

pub async fn create_steam_app_game_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateSteamAppGamePayload>,
) -> Result<(StatusCode, Json<SteamGame>), (StatusCode, String)> {
    match create_steam_game_handler(&state.db_pool, auth_user.user_id, payload).await {
        Ok(new_steam_game) => Ok((StatusCode::CREATED, Json(new_steam_game))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn list_steam_app_games_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<SteamGame>>, (StatusCode, String)> {
    match list_steam_games(&state.db_pool, auth_user.user_id).await {
        Ok(items) => Ok(Json(items)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_steam_app_game_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<SteamGame>, (StatusCode, String)> {
    match get_steam_game(&state.db_pool, id, auth_user.user_id).await {
        Ok(item) => Ok(Json(item)),
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            "Steam app game not found".to_string(),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn update_steam_app_game_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateSteamAppGamePayload>,
) -> Result<Json<SteamGame>, (StatusCode, String)> {
    match update_steam_game(&state.db_pool, id, auth_user.user_id, payload).await {
        Ok(item) => Ok(Json(item)),
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            "Steam app game not found".to_string(),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn delete_steam_app_game_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    match delete_steam_game(&state.db_pool, id, auth_user.user_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            "Steam app game not found".to_string(),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub fn steam_router() -> axum::Router<AppState> {
    Router::new()
        .route("/:appid", get(get_steam_price_handler))
        .route(
            "/",
            post(create_steam_app_game_handler).get(list_steam_app_games_handler),
        )
        .route(
            "/games/:id",
            get(get_steam_app_game_handler)
                .patch(update_steam_app_game_handler)
                .delete(delete_steam_app_game_handler),
        )
}
