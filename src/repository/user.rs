use crate::model::user::{User, UserResponse};
use sqlx::{Error, PgPool};

pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<User>, Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT
            id as "id?: i64",
            username,
            password_hash,
            role,
            avatar_url,
            bio,
            full_name,
            (created_at AT TIME ZONE 'UTC') as "created_at?: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at?: chrono::NaiveDateTime"
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await
}

pub async fn create_user(pool: &PgPool, new_user: User) -> Result<i64, Error> {
    // Insert a new user and return the generated id using RETURNING
    let rec = sqlx::query!(
        r#"
        INSERT INTO users (username, password_hash, role, avatar_url, bio, full_name)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id as "id!: i64"
        "#,
        new_user.username,
        new_user.password_hash,
        new_user.role,
        new_user.avatar_url,
        new_user.bio,
        new_user.full_name
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

pub async fn get_user_by_id(pool: &PgPool, id: i64) -> Result<UserResponse, Error> {
    sqlx::query_as!(
        UserResponse,
        r#"
        SELECT
            id as "id?: i64",
            username,
            role,
            avatar_url,
            bio,
            full_name
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}
