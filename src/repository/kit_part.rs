use crate::model::requirement::KitPartRequirementWithRunner;
use crate::model::{
    kit_part::{
        CreateKitPartPayload, KitPart, KitPartRequirement, KitPartWithSubAssemblyAndRequirements,
    },
    requirement::KitPartWithRequirements,
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

// pub async fn get_all_kit_parts_for_kit(
//     pool: &PgPool,
//     kit_id: i64,
//     user_id: i64,
// ) -> Result<Vec<KitPartWithSubAssembly>, Error> {
//     let rows = sqlx::query!(
//         r#"
//         SELECT
//             kp.id as "kp_id!: i64",
//             kp.code as kp_code,
//             kp.is_cut as kp_is_cut,
//             kp.kit_id as "kp_kit_id!: i64",
//             kp.sub_assembly_id as "kp_sub_assembly_id!: i64",
//             kp.user_id as "kp_user_id!: i64",
//             (kp.created_at AT TIME ZONE 'UTC') as "kp_created_at!: chrono::NaiveDateTime",
//             (kp.updated_at AT TIME ZONE 'UTC') as "kp_updated_at!: chrono::NaiveDateTime",
//             sa.id as "sa_id!: i64",
//             sa.name as sa_name,
//             sa.kit_id as "sa_kit_id!: i64",
//             sa.user_id as "sa_user_id!: i64",
//             (sa.created_at AT TIME ZONE 'UTC') as "sa_created_at!: chrono::NaiveDateTime",
//             (sa.updated_at AT TIME ZONE 'UTC') as "sa_updated_at!: chrono::NaiveDateTime"
//         FROM kit_parts kp
//         JOIN sub_assemblies sa ON sa.id = kp.sub_assembly_id
//         WHERE kp.kit_id = $1 AND kp.user_id = $2
//         "#,
//         kit_id,
//         user_id
//     )
//     .fetch_all(pool)
//     .await?;

//     let out = rows
//         .into_iter()
//         .map(|row| KitPartWithSubAssembly {
//             kit_part: KitPart {
//                 id: row.kp_id,
//                 code: row.kp_code,
//                 is_cut: row.kp_is_cut,
//                 kit_id: row.kp_kit_id,
//                 sub_assembly_id: row.kp_sub_assembly_id,
//                 user_id: row.kp_user_id,
//                 created_at: row.kp_created_at,
//                 updated_at: row.kp_updated_at,
//             },
//             sub_assembly: crate::model::sub_assembly::SubAssembly {
//                 id: row.sa_id,
//                 name: row.sa_name,
//                 kit_id: row.sa_kit_id,
//                 user_id: row.sa_user_id,
//                 created_at: row.sa_created_at,
//                 updated_at: row.sa_updated_at,
//             },
//         })
//         .collect();

//     Ok(out)
// }

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

pub async fn get_kit_part_by_id_with_requirements(
    pool: &PgPool,
    id: i64,
    user_id: i64,
) -> Result<KitPartWithRequirements, Error> {
    let kit_part = get_kit_part_by_id(pool, id, user_id).await?;
    let requirements = get_all_requirements_for_kit_part(pool, id, user_id).await?;
    Ok(KitPartWithRequirements {
        kit_part,
        requirements,
    })
}

// pub async fn update_kit_part(
//     pool: &PgPool,
//     id: i64,
//     user_id: i64,
//     payload: UpdateKitPartPayload,
// ) -> Result<KitPart, Error> {
//     sqlx::query_as!(
//         KitPart,
//         r#"
//         UPDATE kit_parts
//         SET
//             code = COALESCE($1, code),
//             is_cut = COALESCE($2, is_cut),
//             updated_at = NOW()
//         WHERE id = $3 AND user_id = $4
//         RETURNING
//             id as "id!: i64",
//             code,
//             is_cut,
//             kit_id as "kit_id!: i64",
//             sub_assembly_id as "sub_assembly_id!: i64",
//             user_id as "user_id!: i64",
//             (created_at AT TIME ZONE 'UTC') as "created_at!: chrono::NaiveDateTime",
//             (updated_at AT TIME ZONE 'UTC') as "updated_at!: chrono::NaiveDateTime"
//         "#,
//         payload.code,
//         payload.is_cut,
//         id,
//         user_id
//     )
//     .fetch_one(pool)
//     .await
// }

pub async fn update_kit_part_is_cut(
    pool: &PgPool,
    id: i64,
    user_id: i64,
    is_cut: bool,
) -> Result<KitPart, Error> {
    sqlx::query_as!(
        KitPart,
        r#"
        UPDATE kit_parts
        SET is_cut = $1, updated_at = NOW()
        WHERE id = $2 AND user_id = $3
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
        is_cut,
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

pub async fn get_all_requirements_for_kit_part(
    pool: &PgPool,
    kit_part_id: i64,
    user_id: i64,
) -> Result<Vec<KitPartRequirement>, Error> {
    let rows = sqlx::query!(
        r#"
        SELECT
            id as "id!: i64",
            gate as "gate: sqlx::types::Json<Vec<String>>",
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
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| KitPartRequirement {
            id: row.id,
            gate: row.gate.0,
            qty: row.qty,
            is_cut: row.is_cut,
            runner_id: row.runner_id,
            kit_part_id: row.kit_part_id,
            user_id: row.user_id,
        })
        .collect())
}

pub async fn get_all_requirements_with_join_runner_for_kit_part(
    pool: &PgPool,
    kit_part_id: i64,
    user_id: i64,
) -> Result<Vec<KitPartRequirementWithRunner>, Error> {
    let rows = sqlx::query!(
        r#"
        SELECT
            kpr.id as "id!: i64",
            kpr.gate as "gate: sqlx::types::Json<Vec<String>>",
            (kpr.qty)::BIGINT as "qty!: i64",
            kpr.is_cut,
            kpr.runner_id as "runner_id!: i64",
            kpr.kit_part_id as "kit_part_id!: i64",
            kpr.user_id as "user_id!: i64",
            runner.id as "r_id!: i64",
            (runner.name)::TEXT as "r_name!: String",
            runner.kit_id as "r_kit_id!: i64",
            runner.color_id as "r_color_id!: i64",
            (runner.amount)::INT as "r_amount!: i32",
            runner.user_id as "r_user_id!: i64",
            (runner.is_used)::BOOLEAN as "r_is_used!: bool",
            (runner.created_at AT TIME ZONE 'UTC') as "r_created_at!: chrono::NaiveDateTime",
            (runner.updated_at AT TIME ZONE 'UTC') as "r_updated_at!: chrono::NaiveDateTime"
        FROM kit_part_requirements kpr
        JOIN runners runner ON runner.id = kpr.runner_id
        WHERE kpr.kit_part_id = $1 AND kpr.user_id = $2
        "#,
        kit_part_id,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| KitPartRequirementWithRunner {
            id: row.id,
            gate: row.gate.0,
            qty: row.qty,
            is_cut: row.is_cut,
            runner_id: row.runner_id,
            kit_part_id: row.kit_part_id,
            user_id: row.user_id,
            runner: crate::model::runner::Runner {
                id: row.r_id,
                name: row.r_name,
                kit_id: row.r_kit_id,
                color_id: row.r_color_id,
                amount: row.r_amount,
                user_id: row.r_user_id,
                is_used: row.r_is_used,
                created_at: row.r_created_at,
                updated_at: row.r_updated_at,
            },
        })
        .collect())
}

pub async fn get_all_kit_parts_for_kit_with_requirements(
    pool: &PgPool,
    kit_id: i64,
    user_id: i64,
) -> Result<Vec<KitPartWithSubAssemblyAndRequirements>, Error> {
    let rows = sqlx::query!(
        r#"
        SELECT
            kp.id as "kp_id!: i64",
            kp.code as kp_code,
            kp.is_cut as kp_is_cut,
            kp.kit_id as "kp_kit_id!: i64",
            kp.sub_assembly_id as "kp_sub_assembly_id!: i64",
            kp.user_id as "kp_user_id!: i64",
            (kp.created_at AT TIME ZONE 'UTC') as "kp_created_at!: chrono::NaiveDateTime",
            (kp.updated_at AT TIME ZONE 'UTC') as "kp_updated_at!: chrono::NaiveDateTime",
            sa.id as "sa_id!: i64",
            sa.name as sa_name,
            sa.kit_id as "sa_kit_id!: i64",
            sa.user_id as "sa_user_id!: i64",
            (sa.created_at AT TIME ZONE 'UTC') as "sa_created_at!: chrono::NaiveDateTime",
            (sa.updated_at AT TIME ZONE 'UTC') as "sa_updated_at!: chrono::NaiveDateTime",
            COALESCE(
                json_agg(
                    json_build_object(
                        'id', kpr.id,
                        'gate', kpr.gate,
                        'qty', kpr.qty,
                        'is_cut', kpr.is_cut,
                        'runner_id', kpr.runner_id,
                        'kit_part_id', kpr.kit_part_id,
                        'user_id', kpr.user_id
                    )
                    ORDER BY kpr.id
                ) FILTER (WHERE kpr.id IS NOT NULL),
                '[]'::json
            ) as "reqs!: serde_json::Value"
        FROM kit_parts kp
        JOIN sub_assemblies sa ON sa.id = kp.sub_assembly_id
        LEFT JOIN kit_part_requirements kpr
            ON kpr.kit_part_id = kp.id AND kpr.user_id = kp.user_id
        WHERE kp.kit_id = $1 AND kp.user_id = $2
        GROUP BY kp.id, sa.id
        "#,
        kit_id,
        user_id
    )
    .fetch_all(pool)
    .await?;

    let out = rows
        .into_iter()
        .map(|row| KitPartWithSubAssemblyAndRequirements {
            kit_part: KitPart {
                id: row.kp_id,
                code: row.kp_code,
                is_cut: row.kp_is_cut,
                kit_id: row.kp_kit_id,
                sub_assembly_id: row.kp_sub_assembly_id,
                user_id: row.kp_user_id,
                created_at: row.kp_created_at,
                updated_at: row.kp_updated_at,
            },
            sub_assembly: crate::model::sub_assembly::SubAssembly {
                id: row.sa_id,
                name: row.sa_name,
                kit_id: row.sa_kit_id,
                user_id: row.sa_user_id,
                created_at: row.sa_created_at,
                updated_at: row.sa_updated_at,
            },
            requirements: match serde_json::from_value(row.reqs) {
                Ok(v) => v,
                Err(_) => Vec::new(),
            },
        })
        .collect();

    Ok(out)
}
