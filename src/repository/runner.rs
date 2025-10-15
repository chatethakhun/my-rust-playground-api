use crate::model::{
    color::RunnerColor,
    runner::{
        CreateRunnerPayload, Runner, RunnerWithColor, UpdateIsUsedPayload, UpdateRunnerPayload,
    },
};
use sqlx::{Error, PgPool};

// --- CREATE ---
pub async fn create_runner(
    pool: &PgPool,
    user_id: i64,
    payload: CreateRunnerPayload,
) -> Result<Runner, Error> {
    // created_at/updated_at handled by DB defaults; return the inserted row
    sqlx::query_as!(
        Runner,
        r#"
        INSERT INTO runners (name, kit_id, color_id, amount, user_id, is_used)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING
            id as "id!: i64",
            name,
            kit_id as "kit_id!: i64",
            color_id as "color_id!: i64",
            amount as "amount!: i32",
            user_id as "user_id!: i64",
            is_used,
            (created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime"
        "#,
        payload.name,
        payload.kit_id,
        payload.color_id,
        payload.amount,
        user_id,
        false
    )
    .fetch_one(pool)
    .await
}

// --- READ ---
pub async fn get_all_runners(pool: &PgPool, user_id: i64) -> Result<Vec<Runner>, Error> {
    sqlx::query_as!(
        Runner,
        r#"
        SELECT
            id as "id!: i64",
            name,
            kit_id as "kit_id!: i64",
            color_id as "color_id!: i64",
            amount as "amount!: i32",
            user_id as "user_id!: i64",
            is_used,
            (created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime"
        FROM runners
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}

// Function สำหรับดึงข้อมูล Runner พร้อมข้อมูลสีที่ซ้อนอยู่ข้างใน
pub async fn get_all_runners_with_color_for_kit(
    pool: &PgPool,
    kit_id: i64,
    user_id: i64,
) -> Result<Vec<RunnerWithColor>, Error> {
    // Query แบบปกติ แล้วแปลงเป็น nested structure
    let rows = sqlx::query!(
        r#"
        SELECT
            r.id as "runner_id!: i64",
            r.name as runner_name,
            r.kit_id as "kit_id!: i64",
            r.amount as "amount!: i32",
            r.user_id as "user_id!: i64",
            r.is_used,
            (r.created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
            (r.updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime",
            c.id as "color_id!: i64",
            c.code as color_code,
            c.name as color_name,
            c.hex as color_hex,
            c.is_multi as color_is_multi,
            c.is_clear as color_is_clear
        FROM runners r
        INNER JOIN colors c ON r.color_id = c.id
        WHERE r.user_id = $1 AND r.kit_id = $2
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
            is_used: row.is_used,
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
    pool: &PgPool,
    runner_id: i64,
    user_id: i64,
) -> Result<Runner, Error> {
    sqlx::query_as!(
        Runner,
        r#"
        SELECT
            id as "id!: i64",
            name,
            kit_id as "kit_id!: i64",
            color_id as "color_id!: i64",
            amount as "amount!: i32",
            user_id as "user_id!: i64",
            is_used,
            (created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime"
        FROM runners
        WHERE id = $1 AND user_id = $2
        "#,
        runner_id,
        user_id
    )
    .fetch_one(pool)
    .await
}

// --- UPDATE ---
pub async fn update_runner(
    pool: &PgPool,
    runner_id: i64,
    user_id: i64,
    payload: UpdateRunnerPayload,
) -> Result<Runner, Error> {
    // amount ใน payload เป็น Option<i64> แต่ในตารางเป็น INTEGER (i32) — แปลงเพื่อความชัดเจน
    let amount_i32: Option<i32> = payload.amount.map(|v| v as i32);

    sqlx::query_as!(
        Runner,
        r#"
        UPDATE runners
        SET
            name = COALESCE($1, name),
            kit_id = COALESCE($2, kit_id),
            color_id = COALESCE($3, color_id),
            amount = COALESCE($4, amount),
            updated_at = NOW()
        WHERE id = $5 AND user_id = $6
        RETURNING
            id as "id!: i64",
            name,
            kit_id as "kit_id!: i64",
            color_id as "color_id!: i64",
            amount as "amount!: i32",
            user_id as "user_id!: i64",
            is_used,
            (created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime"
        "#,
        payload.name,
        payload.kit_id,
        payload.color_id,
        amount_i32,
        runner_id,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn update_runner_is_used(
    pool: &PgPool,
    runner_id: i64,
    user_id: i64,
    payload: UpdateIsUsedPayload,
) -> Result<Runner, Error> {
    sqlx::query_as!(
        Runner,
        r#"
        UPDATE runners
        SET is_used = $1, updated_at = NOW()
        WHERE id = $2 AND user_id = $3
        RETURNING
            id as "id!: i64",
            name,
            kit_id as "kit_id!: i64",
            color_id as "color_id!: i64",
            amount as "amount!: i32",
            user_id as "user_id!: i64",
            is_used,
            (created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime"
        "#,
        payload.is_used,
        runner_id,
        user_id
    )
    .fetch_one(pool)
    .await
}

// --- DELETE ---
pub async fn delete_runner(pool: &PgPool, runner_id: i64, user_id: i64) -> Result<(), Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM runners WHERE id = $1 AND user_id = $2
        "#,
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
    pool: &PgPool,
    kit_id: i64,
    user_id: i64,
) -> Result<Vec<Runner>, Error> {
    sqlx::query_as!(
        Runner,
        r#"
        SELECT
            id as "id!: i64",
            name,
            kit_id as "kit_id!: i64",
            color_id as "color_id!: i64",
            amount as "amount!: i32",
            user_id as "user_id!: i64",
            is_used,
            (created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime"
        FROM runners
        WHERE kit_id = $1 AND user_id = $2
        "#,
        kit_id,
        user_id
    )
    .fetch_all(pool)
    .await
}
