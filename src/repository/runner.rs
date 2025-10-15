use crate::model::{
    color::RunnerColor,
    runner::{
        CreateRunnerPayload, Runner, RunnerWithColor, UpdateIsUsedPayload, UpdateRunnerPayload,
    },
};
use chrono::Utc;
use sqlx::{Error, SqlitePool};

// --- CREATE ---
pub async fn create_runner(
    pool: &SqlitePool,
    user_id: i64,
    payload: CreateRunnerPayload,
) -> Result<Runner, Error> {
    let now = Utc::now().naive_utc();
    let new_runner_id = sqlx::query!(
        r#"
        INSERT INTO runners (name, kit_id, color_id, amount, user_id, is_used, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        payload.name,
        payload.kit_id,
        payload.color_id,
        payload.amount,
        user_id,
        false, // ค่าเริ่มต้น
        now,
        now
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    // เรียกใช้ get_runner_by_id ที่มีการ SELECT แบบสมบูรณ์
    get_runner_by_id(pool, new_runner_id, user_id).await
}

// --- READ ---
pub async fn get_all_runners(pool: &SqlitePool, user_id: i64) -> Result<Vec<Runner>, Error> {
    sqlx::query_as!(
        Runner,
        r#"
        SELECT
            id as "id!",
            name,
            kit_id as "kit_id!",
            color_id as "color_id!",
            amount as "amount!: i32", -- 👈 แก้ไขที่นี่
            user_id as "user_id!",
            is_used,
            created_at as "created_at!",
            updated_at as "updated_at!"
        FROM runners
        WHERE user_id = ?
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}

// Function สำหรับดึงข้อมูล
pub async fn get_all_runners_with_color_for_kit(
    pool: &SqlitePool,
    kit_id: i64,
    user_id: i64,
) -> Result<Vec<RunnerWithColor>, Error> {
    // Query แบบปกติ แล้วแปลงเป็น nested structure
    let rows = sqlx::query!(
        r#"
        SELECT
            r.id as runner_id,
            r.name as runner_name,
            r.kit_id,
            r.amount as "amount!: i32",
            r.user_id,
            r.is_used,
            r.created_at,
            r.updated_at,
            c.id as color_id,
            c.code as color_code,
            c.name as color_name,
            c.hex as color_hex,
            c.is_multi as color_is_multi,
            c.is_clear as color_is_clear
        FROM runners r
        INNER JOIN colors c ON r.color_id = c.id
        WHERE r.user_id = ? AND r.kit_id = ?
        "#,
        user_id,
        kit_id
    )
    .fetch_all(pool)
    .await?;

    // แปลง flat rows เป็น nested structure
    let runners = rows
        .into_iter()
        .map(|row| RunnerWithColor {
            id: row.runner_id,
            name: row.runner_name,
            kit_id: row.kit_id,
            amount: row.amount,
            user_id: row.user_id,
            is_used: row.is_used, // SQLite boolean handling
            created_at: row.created_at,
            updated_at: row.updated_at,
            color: RunnerColor {
                id: row.color_id,
                name: row.color_name,
                code: row.color_code,
                hex: row.color_hex,
                is_clear: row.color_is_clear,
                is_multi: row.color_is_multi,
            },
        })
        .collect();

    Ok(runners)
}

pub async fn get_runner_by_id(
    pool: &SqlitePool,
    runner_id: i64,
    user_id: i64,
) -> Result<Runner, Error> {
    sqlx::query_as!(
        Runner,
        r#"
        SELECT
            id as "id!",
            name,
            kit_id as "kit_id!",
            color_id as "color_id!",
            amount as "amount!: i32", -- 👈 แก้ไขที่นี่
            user_id as "user_id!",
            is_used,
            created_at as "created_at!",
            updated_at as "updated_at!"
        FROM runners
        WHERE id = ? AND user_id = ?
        "#,
        runner_id,
        user_id
    )
    .fetch_one(pool)
    .await
}

// --- UPDATE ---
pub async fn update_runner(
    pool: &SqlitePool,
    runner_id: i64,
    user_id: i64,
    payload: UpdateRunnerPayload,
) -> Result<Runner, Error> {
    let now = Utc::now().naive_utc();
    let result = sqlx::query!(
        r#"
        UPDATE runners
        SET name = COALESCE(?, name),
            kit_id = COALESCE(?, kit_id),
            color_id = COALESCE(?, color_id),
            amount = COALESCE(?, amount),
            updated_at = ?
        WHERE id = ? AND user_id = ?
        "#,
        payload.name,
        payload.kit_id,
        payload.color_id,
        payload.amount,
        now,
        runner_id,
        user_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }
    get_runner_by_id(pool, runner_id, user_id).await
}

pub async fn update_runner_is_used(
    pool: &SqlitePool,
    runner_id: i64,
    user_id: i64,
    payload: UpdateIsUsedPayload,
) -> Result<Runner, Error> {
    let now = Utc::now().naive_utc();
    let result = sqlx::query!(
        "UPDATE runners SET is_used = ?, updated_at = ? WHERE id = ? AND user_id = ?",
        payload.is_used,
        now,
        runner_id,
        user_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }
    get_runner_by_id(pool, runner_id, user_id).await
}

// --- DELETE ---
pub async fn delete_runner(pool: &SqlitePool, runner_id: i64, user_id: i64) -> Result<(), Error> {
    let result = sqlx::query!(
        "DELETE FROM runners WHERE id = ? AND user_id = ?",
        runner_id,
        user_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }
    Ok(())
}

pub async fn get_all_runners_for_kit(
    pool: &SqlitePool,
    kit_id: i64,
    user_id: i64,
) -> Result<Vec<Runner>, Error> {
    sqlx::query_as!(
        Runner,
        r#"
        SELECT
            id as "id!",
            name,
            kit_id as "kit_id!",
            color_id as "color_id!",
            amount as "amount!: i32",
            user_id as "user_id!",
            is_used,
            created_at as "created_at!",
            updated_at as "updated_at!"
        FROM runners
        WHERE kit_id = ? AND user_id = ? -- 🛡️ เช็ค user_id เพื่อความปลอดภัย
        "#,
        kit_id,
        user_id
    )
    .fetch_all(pool)
    .await
}
