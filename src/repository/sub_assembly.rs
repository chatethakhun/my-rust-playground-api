use crate::model::sub_assembly::{CreateSubAssemblyPayload, SubAssembly, UpdateSubAssemblyPayload};
use sqlx::{Error, PgPool};

pub async fn create_sub_assembly(
    pool: &PgPool,
    user_id: i64,
    payload: CreateSubAssemblyPayload,
) -> Result<SubAssembly, Error> {
    sqlx::query_as!(
        SubAssembly,
        r#"
        INSERT INTO sub_assemblies (name, kit_id, user_id, created_at, updated_at)
        VALUES ($1, $2, $3, NOW(), NOW())
        RETURNING
            id as "id!: i64",
            name,
            kit_id as "kit_id!: i64",
            user_id as "user_id!: i64",
            (created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime"
        "#,
        payload.name,
        payload.kit_id,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_all_sub_assemblies_for_kit(
    pool: &PgPool,
    kit_id: i64,
    user_id: i64,
) -> Result<Vec<SubAssembly>, Error> {
    sqlx::query_as!(
        SubAssembly,
        r#"
        SELECT
            id as "id!: i64",
            name,
            kit_id as "kit_id!: i64",
            user_id as "user_id!: i64",
            (created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime"
        FROM sub_assemblies
        WHERE kit_id = $1 AND user_id = $2
        "#,
        kit_id,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_sub_assembly_by_id(
    pool: &PgPool,
    id: i64,
    user_id: i64,
) -> Result<SubAssembly, Error> {
    sqlx::query_as!(
        SubAssembly,
        r#"
        SELECT
            id as "id!: i64",
            name,
            kit_id as "kit_id!: i64",
            user_id as "user_id!: i64",
            (created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime"
        FROM sub_assemblies
        WHERE id = $1 AND user_id = $2
        "#,
        id,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn update_sub_assembly(
    pool: &PgPool,
    id: i64,
    user_id: i64,
    payload: UpdateSubAssemblyPayload,
) -> Result<SubAssembly, Error> {
    sqlx::query_as!(
        SubAssembly,
        r#"
        UPDATE sub_assemblies
        SET
            name = COALESCE($1, name),
            kit_id = COALESCE($2, kit_id),
            updated_at = NOW()
        WHERE id = $3 AND user_id = $4
        RETURNING
            id as "id!: i64",
            name,
            kit_id as "kit_id!: i64",
            user_id as "user_id!: i64",
            (created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime"
        "#,
        payload.name,
        payload.kit_id,
        id,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_sub_assembly(pool: &PgPool, id: i64, user_id: i64) -> Result<(), Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM sub_assemblies
        WHERE id = $1 AND user_id = $2
        "#,
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
