// src/repositories/sub_assembly_repository.rs

use crate::model::sub_assembly::{CreateSubAssemblyPayload, SubAssembly, UpdateSubAssemblyPayload};
use chrono::Utc;
use sqlx::{Error, SqlitePool};

pub async fn create_sub_assembly(
    pool: &SqlitePool,
    user_id: i64,
    payload: CreateSubAssemblyPayload,
) -> Result<SubAssembly, Error> {
    let now = Utc::now().naive_utc();
    let new_id = sqlx::query!(
        "INSERT INTO sub_assemblies (name, kit_id, user_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        payload.name,
        payload.kit_id,
        user_id,
        now,
        now
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    get_sub_assembly_by_id(pool, new_id, user_id).await
}

pub async fn get_all_sub_assemblies_for_kit(
    pool: &SqlitePool,
    kit_id: i64,
    user_id: i64,
) -> Result<Vec<SubAssembly>, Error> {
    sqlx::query_as!(
        SubAssembly,
        "SELECT * FROM sub_assemblies WHERE kit_id = ? AND user_id = ?",
        kit_id,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_sub_assembly_by_id(
    pool: &SqlitePool,
    id: i64,
    user_id: i64,
) -> Result<SubAssembly, Error> {
    sqlx::query_as!(
        SubAssembly,
        "SELECT * FROM sub_assemblies WHERE id = ? AND user_id = ?",
        id,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn update_sub_assembly(
    pool: &SqlitePool,
    id: i64,
    user_id: i64,
    payload: UpdateSubAssemblyPayload,
) -> Result<SubAssembly, Error> {
    let now = Utc::now().naive_utc();
    let result = sqlx::query!(
        "UPDATE sub_assemblies SET name = COALESCE(?, name), kit_id = COALESCE(?, kit_id), updated_at = ? WHERE id = ? AND user_id = ?",
        payload.name,
        payload.kit_id,
        now,
        id,
        user_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }
    get_sub_assembly_by_id(pool, id, user_id).await
}

pub async fn delete_sub_assembly(pool: &SqlitePool, id: i64, user_id: i64) -> Result<(), Error> {
    let result = sqlx::query!(
        "DELETE FROM sub_assemblies WHERE id = ? AND user_id = ?",
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
