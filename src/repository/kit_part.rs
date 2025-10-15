use crate::model::kit_part::{
    CreateKitPartPayload, CreateKitPartRequirementPayload, KitPart, KitPartRequirement,
    UpdateKitPartPayload,
};
use sqlx::{Error, PgPool};

// --- KitPart Functions ---

pub async fn create_kit_part(
    pool: &PgPool,
    user_id: i64,
    payload: CreateKitPartPayload,
) -> Result<KitPart, Error> {
    sqlx::query_as!(
        KitPart,
        r#"
        INSERT INTO kit_parts (code, is_cut, kit_id, sub_assembly_id, user_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING
            id as "id!: i64",
            code,
            is_cut,
            kit_id as "kit_id!: i64",
            sub_assembly_id as "sub_assembly_id!: i64",
            user_id as "user_id!: i64",
            (created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime"
        "#,
        payload.code,
        false, // default value
        payload.kit_id,
        payload.sub_assembly_id,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_all_kit_parts_for_sub_assembly(
    pool: &PgPool,
    sub_assembly_id: i64,
    user_id: i64,
) -> Result<Vec<KitPart>, Error> {
    sqlx::query_as!(
        KitPart,
        r#"
        SELECT
            id as "id!: i64",
            code,
            is_cut,
            kit_id as "kit_id!: i64",
            sub_assembly_id as "sub_assembly_id!: i64",
            user_id as "user_id!: i64",
            (created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime"
        FROM kit_parts
        WHERE sub_assembly_id = $1 AND user_id = $2
        "#,
        sub_assembly_id,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_all_kit_parts_for_kit(
    pool: &PgPool,
    kit_id: i64,
    user_id: i64,
) -> Result<Vec<KitPart>, Error> {
    sqlx::query_as!(
        KitPart,
        r#"
        SELECT
            id as "id!: i64",
            code,
            is_cut,
            kit_id as "kit_id!: i64",
            sub_assembly_id as "sub_assembly_id!: i64",
            user_id as "user_id!: i64",
            (created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime"
        FROM kit_parts
        WHERE kit_id = $1 AND user_id = $2
        "#,
        kit_id,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_kit_part_by_id(pool: &PgPool, id: i64, user_id: i64) -> Result<KitPart, Error> {
    sqlx::query_as!(
        KitPart,
        r#"
        SELECT
            id as "id!: i64",
            code,
            is_cut,
            kit_id as "kit_id!: i64",
            sub_assembly_id as "sub_assembly_id!: i64",
            user_id as "user_id!: i64",
            (created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime"
        FROM kit_parts
        WHERE id = $1 AND user_id = $2
        "#,
        id,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn update_kit_part(
    pool: &PgPool,
    id: i64,
    user_id: i64,
    payload: UpdateKitPartPayload,
) -> Result<KitPart, Error> {
    sqlx::query_as!(
        KitPart,
        r#"
        UPDATE kit_parts
        SET
            code = COALESCE($1, code),
            is_cut = COALESCE($2, is_cut),
            updated_at = NOW()
        WHERE id = $3 AND user_id = $4
        RETURNING
            id as "id!: i64",
            code,
            is_cut,
            kit_id as "kit_id!: i64",
            sub_assembly_id as "sub_assembly_id!: i64",
            user_id as "user_id!: i64",
            (created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
            (updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime"
        "#,
        payload.code,
        payload.is_cut,
        id,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_kit_part(pool: &PgPool, id: i64, user_id: i64) -> Result<(), Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM kit_parts
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

// --- KitPartRequirement Functions ---

pub async fn create_kit_part_requirement(
    pool: &PgPool,
    user_id: i64,
    payload: CreateKitPartRequirementPayload,
) -> Result<KitPartRequirement, Error> {
    sqlx::query_as!(
        KitPartRequirement,
        r#"
        INSERT INTO kit_part_requirements (gate, qty, is_cut, runner_id, kit_part_id, user_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING
            id as "id!: i64",
            gate,
            (qty)::BIGINT as "qty!: i64",
            is_cut,
            runner_id as "runner_id!: i64",
            kit_part_id as "kit_part_id!: i64",
            user_id as "user_id!: i64"
        "#,
        payload.gate,
        payload.qty, // INTEGER in DB; struct expects i64 via cast on return
        false,       // default
        payload.runner_id,
        payload.kit_part_id,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_all_requirements_for_kit_part(
    pool: &PgPool,
    kit_part_id: i64,
    user_id: i64,
) -> Result<Vec<KitPartRequirement>, Error> {
    sqlx::query_as!(
        KitPartRequirement,
        r#"
        SELECT
            id as "id!: i64",
            gate,
            (qty)::BIGINT as "qty!: i64",
            is_cut,
            runner_id as "runner_id!: i64",
            kit_part_id as "kit_part_id!: i64",
            user_id as "user_id!: i64"
        FROM kit_part_requirements
        WHERE kit_part_id = $1 AND user_id = $2
        "#,
        kit_part_id,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn delete_kit_part_requirement(
    pool: &PgPool,
    id: i64,
    user_id: i64,
) -> Result<(), Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM kit_part_requirements
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
