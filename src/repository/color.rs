use crate::model::color::{Color, UpdateColorPayload};
use sqlx::{Error, PgPool};

pub async fn get_colors(pool: &PgPool, user_id: i64) -> Result<Vec<Color>, Error> {
    sqlx::query_as!(
        Color,
        r#"
        SELECT
            id as "id?: i64",
            name,
            code,
            hex,
            is_clear,
            is_multi,
            user_id,
            (created_at AT TIME ZONE 'UTC') as "created_at?: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at?: chrono::NaiveDateTime"
        FROM colors
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_color_by_id(pool: &PgPool, color_id: i64) -> Result<Color, Error> {
    sqlx::query_as!(
        Color,
        r#"
        SELECT
            id as "id?: i64",
            name,
            code,
            hex,
            is_clear,
            is_multi,
            user_id,
            (created_at AT TIME ZONE 'UTC') as "created_at?: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at?: chrono::NaiveDateTime"
        FROM colors
        WHERE id = $1
        "#,
        color_id
    )
    .fetch_one(pool)
    .await
}

pub async fn create_color(pool: &PgPool, color: Color) -> Result<Color, Error> {
    sqlx::query_as!(
        Color,
        r#"
        INSERT INTO colors (name, code, hex, is_clear, is_multi, user_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING
            id as "id?: i64",
            name,
            code,
            hex,
            is_clear,
            is_multi,
            user_id,
            (created_at AT TIME ZONE 'UTC') as "created_at?: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at?: chrono::NaiveDateTime"
        "#,
        color.name,
        color.code,
        color.hex,
        color.is_clear,
        color.is_multi,
        color.user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn update_color(
    pool: &PgPool,
    color_id: i64,
    user_id: i64,
    payload: UpdateColorPayload,
) -> Result<Color, Error> {
    sqlx::query_as!(
        Color,
        r#"
        UPDATE colors
        SET
            name = COALESCE($1, name),
            code = COALESCE($2, code),
            hex = COALESCE($3, hex),
            is_clear = COALESCE($4, is_clear),
            is_multi = COALESCE($5, is_multi),
            updated_at = NOW()
        WHERE id = $6 AND user_id = $7
        RETURNING
            id as "id?: i64",
            name,
            code,
            hex,
            is_clear,
            is_multi,
            user_id,
            (created_at AT TIME ZONE 'UTC') as "created_at?: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at?: chrono::NaiveDateTime"
        "#,
        payload.name,
        payload.code,
        payload.hex,
        payload.is_clear,
        payload.is_multi,
        color_id,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_color(pool: &PgPool, color_id: i64, user_id: i64) -> Result<(), Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM colors
        WHERE id = $1 AND user_id = $2
        "#,
        color_id,
        user_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }

    Ok(())
}
