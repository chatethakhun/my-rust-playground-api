// src/repositories/kit_part_repository.rs

use crate::model::kit_part::{
    CreateKitPartPayload, CreateKitPartRequirementPayload, KitPart, KitPartRequirement,
    UpdateKitPartPayload,
};
use chrono::Utc;
use sqlx::{Error, SqlitePool};

// --- KitPart Functions ---

pub async fn create_kit_part(
    pool: &SqlitePool,
    user_id: i64,
    payload: CreateKitPartPayload,
) -> Result<KitPart, Error> {
    let now = Utc::now().naive_utc();
    let new_id = sqlx::query!(
        r#"
        INSERT INTO kit_parts (code, is_cut, kit_id, sub_assembly_id, user_id, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        payload.code,
        false, // Default value
        payload.kit_id,
        payload.sub_assembly_id,
        user_id,
        now,
        now
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    get_kit_part_by_id(pool, new_id, user_id).await
}

pub async fn get_all_kit_parts_for_sub_assembly(
    pool: &SqlitePool,
    sub_assembly_id: i64,
    user_id: i64,
) -> Result<Vec<KitPart>, Error> {
    sqlx::query_as!(
        KitPart,
        "SELECT * FROM kit_parts WHERE sub_assembly_id = ? AND user_id = ?",
        sub_assembly_id,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_all_kit_parts_for_kit(
    pool: &SqlitePool,
    kit_id: i64,
    user_id: i64,
) -> Result<Vec<KitPart>, Error> {
    sqlx::query_as!(
        KitPart,
        "SELECT * FROM kit_parts WHERE kit_id = ? AND user_id = ?",
        kit_id,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_kit_part_by_id(
    pool: &SqlitePool,
    id: i64,
    user_id: i64,
) -> Result<KitPart, Error> {
    sqlx::query_as!(
        KitPart,
        "SELECT * FROM kit_parts WHERE id = ? AND user_id = ?",
        id,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn update_kit_part(
    pool: &SqlitePool,
    id: i64,
    user_id: i64,
    payload: UpdateKitPartPayload,
) -> Result<KitPart, Error> {
    let now = Utc::now().naive_utc();
    let result = sqlx::query!(
        "UPDATE kit_parts SET code = COALESCE(?, code), is_cut = COALESCE(?, is_cut), updated_at = ? WHERE id = ? AND user_id = ?",
        payload.code,
        payload.is_cut,
        now,
        id,
        user_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }
    get_kit_part_by_id(pool, id, user_id).await
}

pub async fn delete_kit_part(pool: &SqlitePool, id: i64, user_id: i64) -> Result<(), Error> {
    let result = sqlx::query!(
        "DELETE FROM kit_parts WHERE id = ? AND user_id = ?",
        id,
        user_id
    )
    .execute(pool)
    .await?;
    if result.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }
    Ok(())
}

// --- KitPartRequirement Functions ---

pub async fn create_kit_part_requirement(
    pool: &SqlitePool,
    user_id: i64,
    payload: CreateKitPartRequirementPayload,
) -> Result<KitPartRequirement, Error> {
    sqlx::query_as!(
        KitPartRequirement,
        "INSERT INTO kit_part_requirements (gate, qty, is_cut, runner_id, kit_part_id, user_id) VALUES (?, ?, ?, ?, ?, ?) RETURNING *",
        payload.gate,
        payload.qty,
        false, // Default value
        payload.runner_id,
        payload.kit_part_id,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_all_requirements_for_kit_part(
    pool: &SqlitePool,
    kit_part_id: i64,
    user_id: i64,
) -> Result<Vec<KitPartRequirement>, Error> {
    sqlx::query_as!(
        KitPartRequirement,
        "SELECT * FROM kit_part_requirements WHERE kit_part_id = ? AND user_id = ?",
        kit_part_id,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn delete_kit_part_requirement(
    pool: &SqlitePool,
    id: i64,
    user_id: i64,
) -> Result<(), Error> {
    let result = sqlx::query!(
        "DELETE FROM kit_part_requirements WHERE id = ? AND user_id = ?",
        id,
        user_id
    )
    .execute(pool)
    .await?;
    if result.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }
    Ok(())
}
