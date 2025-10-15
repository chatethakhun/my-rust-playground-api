use sqlx::{Error, PgPool};

use crate::model::{
    kit::{
        CreateKitPayload, Kit, KitGrade, KitStatus, KitWithRunners, UpdateKitPayload,
        UpdateStatusPayload,
    },
    runner::Runner,
};

// --- CREATE ---
pub async fn create(
    pool: &PgPool,
    user_id: i64,
    payload: CreateKitPayload,
) -> Result<KitWithRunners, Error> {
    let grade_str = match payload.grade {
        KitGrade::Eg => "eg",
        KitGrade::Hg => "hg",
        KitGrade::Rg => "rg",
        KitGrade::Mg => "mg",
        KitGrade::Mgsd => "mgsd",
        KitGrade::Pg => "pg",
        KitGrade::Other => "other",
    };
    let status_str = "pending";

    let rec = sqlx::query!(
        r#"
        INSERT INTO kits (name, grade, status, user_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, NOW(), NOW())
        RETURNING id as "id!: i64"
        "#,
        payload.name,
        grade_str,
        status_str,
        user_id
    )
    .fetch_one(pool)
    .await?;

    let new_kit_id = rec.id;
    get_by_id(pool, new_kit_id, user_id).await
}

pub async fn get_all(
    pool: &PgPool,
    user_id: i64,
    kit_status: Option<KitStatus>,
) -> Result<Vec<Kit>, Error> {
    // map enum -> &str for filtering
    let status_str: Option<&str> = match kit_status {
        Some(KitStatus::Pending) => Some("pending"),
        Some(KitStatus::InProgress) => Some("in_progress"),
        Some(KitStatus::Done) => Some("done"),
        None => None,
    };

    let kits = sqlx::query_as!(
        Kit,
        r#"
        SELECT
            id as "id!",
            name,
            grade as "grade: KitGrade",
            status as "status: KitStatus",
            user_id as "user_id!",
            (created_at AT TIME ZONE 'UTC') as "created_at!",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!"
        FROM kits
        WHERE user_id = $1 AND ($2::TEXT IS NULL OR status = $2)
        "#,
        user_id,
        status_str
    )
    .fetch_all(pool)
    .await?;

    Ok(kits)
}

// --- READ BY ID (พร้อม Runners) ---
pub async fn get_by_id(pool: &PgPool, kit_id: i64, user_id: i64) -> Result<KitWithRunners, Error> {
    // 1. ดึงข้อมูล Kit ที่ต้องการ
    let kit = sqlx::query_as!(
        Kit,
        r#"
        SELECT
            id as "id!",
            name,
            grade as "grade: KitGrade",
            status as "status: KitStatus",
            user_id as "user_id!",
            (created_at AT TIME ZONE 'UTC') as "created_at!",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!"
        FROM kits
        WHERE id = $1 AND user_id = $2
        "#,
        kit_id,
        user_id
    )
    .fetch_one(pool)
    .await?;

    // 2. ดึง Runners ทั้งหมดที่เกี่ยวข้องกับ Kit นี้
    let runners = sqlx::query_as!(
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
            (created_at AT TIME ZONE 'UTC') as "created_at!",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!"
        FROM runners
        WHERE kit_id = $1 AND user_id = $2
        "#,
        kit_id,
        user_id
    )
    .fetch_all(pool)
    .await?;

    // 3. ประกอบร่างเป็น KitWithRunners แล้วส่งกลับ
    Ok(KitWithRunners { kit, runners })
}

// --- UPDATE ---
pub async fn update(
    pool: &PgPool,
    kit_id: i64,
    user_id: i64,
    payload: UpdateKitPayload,
) -> Result<KitWithRunners, Error> {
    let grade_str: Option<&str> = payload.grade.map(|g| match g {
        KitGrade::Eg => "eg",
        KitGrade::Hg => "hg",
        KitGrade::Rg => "rg",
        KitGrade::Mg => "mg",
        KitGrade::Mgsd => "mgsd",
        KitGrade::Pg => "pg",
        KitGrade::Other => "other",
    });

    let result = sqlx::query!(
        r#"
        UPDATE kits
        SET
            name = COALESCE($1, name),
            grade = COALESCE($2, grade),
            updated_at = NOW()
        WHERE id = $3 AND user_id = $4
        "#,
        payload.name,
        grade_str,
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
    pool: &PgPool,
    kit_id: i64,
    user_id: i64,
    payload: UpdateStatusPayload,
) -> Result<KitWithRunners, Error> {
    let status_str = match payload.status {
        KitStatus::Pending => "pending",
        KitStatus::InProgress => "in_progress",
        KitStatus::Done => "done",
    };

    let result = sqlx::query!(
        r#"
        UPDATE kits
        SET status = $1, updated_at = NOW()
        WHERE id = $2 AND user_id = $3
        "#,
        status_str,
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
pub async fn delete_kit(pool: &PgPool, kit_id: i64, user_id: i64) -> Result<(), Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM kits WHERE id = $1 AND user_id = $2
        "#,
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
