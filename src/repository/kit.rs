use chrono::Utc;
use sqlx::{Error, SqlitePool};

// สมมติว่า import model จาก path นี้
use crate::model::kit::{
    CreateKitPayload, Kit, KitGrade, Status, UpdateKitPayload, UpdateStatusPayload,
};

// --- CREATE ---
pub async fn create(
    pool: &SqlitePool,
    user_id: i64,
    payload: CreateKitPayload,
) -> Result<Kit, Error> {
    let now = Utc::now().naive_utc();
    let new_kit_id = sqlx::query!(
        r#"
        INSERT INTO kits (name, grade, status, user_id, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        payload.name,
        payload.grade,
        Status::Pending, // 👈 สถานะเริ่มต้น
        user_id,
        now,
        now
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    get_by_id(pool, new_kit_id, user_id).await
}

pub async fn get_all(pool: &SqlitePool, user_id: i64) -> Result<Vec<Kit>, Error> {
    let kits = sqlx::query_as!(
        Kit,
        r#"
        SELECT
            id as "id!", -- 👈 เพิ่ม !
            name,
            grade as "grade: KitGrade",
            status as "status: Status",
            user_id as "user_id!", -- 👈 เพิ่ม !
            created_at as "created_at!", -- 👈 เพิ่ม !
            updated_at as "updated_at!"  -- 👈 เพิ่ม !
        FROM kits
        WHERE user_id = ?
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;
    Ok(kits)
}

pub async fn get_by_id(pool: &SqlitePool, kit_id: i64, user_id: i64) -> Result<Kit, Error> {
    let kit = sqlx::query_as!(
        Kit,
        r#"
        SELECT
            id, name,
            grade as "grade: KitGrade",
            status as "status: Status",
            user_id, created_at, updated_at
        FROM kits
        WHERE id = ? AND user_id = ?
        "#,
        kit_id,
        user_id
    )
    .fetch_one(pool)
    .await?;
    Ok(kit)
}

// --- UPDATE ---
pub async fn update(
    pool: &SqlitePool,
    kit_id: i64,
    user_id: i64,
    payload: UpdateKitPayload,
) -> Result<Kit, Error> {
    let now = Utc::now().naive_utc();
    let result = sqlx::query!(
        r#"
        UPDATE kits
        SET name = COALESCE(?, name), grade = COALESCE(?, grade), updated_at = ?
        WHERE id = ? AND user_id = ?
        "#,
        payload.name,
        payload.grade,
        now,
        kit_id,
        user_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }
    get_by_id(pool, kit_id, user_id).await
}

// --- UPDATE STATUS (Specific Update) ---
pub async fn update_status(
    pool: &SqlitePool,
    kit_id: i64,
    user_id: i64,
    payload: UpdateStatusPayload,
) -> Result<Kit, Error> {
    let now = Utc::now().naive_utc();
    let result = sqlx::query!(
        "UPDATE kits SET status = ?, updated_at = ? WHERE id = ? AND user_id = ?",
        payload.status,
        now,
        kit_id,
        user_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }
    get_by_id(pool, kit_id, user_id).await
}

// --- DELETE ---
pub async fn delete_kit(pool: &SqlitePool, kit_id: i64, user_id: i64) -> Result<(), Error> {
    let result = sqlx::query!(
        "DELETE FROM kits WHERE id = ? AND user_id = ?",
        kit_id,
        user_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }
    Ok(())
}
