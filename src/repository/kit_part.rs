use crate::model::kit_part::{
    CreateKitPartPayload, CreateKitPartRequirementPayload, KitPart, KitPartRequirement,
    KitPartWithRequirements, KitPartWithSubAssemblyAndRequirements,
    UpdateKitPartRequirementPayload,
};
use sqlx::{Error, PgPool, Row};

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

pub async fn create_kit_part_requirement(
    pool: &PgPool,
    user_id: i64,
    payload: CreateKitPartRequirementPayload,
) -> Result<KitPartRequirement, Error> {
    let row = sqlx::query!(
        r#"
        INSERT INTO kit_part_requirements (gate, qty, is_cut, runner_id, kit_part_id, user_id)
        VALUES ($1::JSONB, $2, $3, $4, $5, $6)
        RETURNING
            id as "id!: i64",
            gate as "gate: sqlx::types::Json<Vec<String>>",
            (qty)::BIGINT as "qty!: i64",
            is_cut,
            runner_id as "runner_id!: i64",
            kit_part_id as "kit_part_id!: i64",
            user_id as "user_id!: i64"
        "#,
        serde_json::json!(payload.gate),
        payload.qty,
        false,
        payload.runner_id,
        payload.kit_part_id,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(KitPartRequirement {
        id: row.id,
        gate: row.gate.0,
        qty: row.qty,
        is_cut: row.is_cut,
        runner_id: row.runner_id,
        kit_part_id: row.kit_part_id,
        user_id: row.user_id,
    })
}

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

pub async fn update_kit_part_requirement(
    pool: &PgPool,
    id: i64,
    user_id: i64,
    payload: UpdateKitPartRequirementPayload,
) -> Result<KitPartRequirement, Error> {
    // Prepare optional JSON value for gate
    let gate_json_opt = payload.gate.map(|g| serde_json::json!(g));

    let row = sqlx::query!(
        r#"
        UPDATE kit_part_requirements
        SET
            gate = COALESCE($1::JSONB, gate),
            qty = COALESCE($2, qty),
            is_cut = COALESCE($3, is_cut),
            runner_id = COALESCE($4, runner_id)
        WHERE id = $5 AND user_id = $6
        RETURNING
            id as "id!: i64",
            gate as "gate: sqlx::types::Json<Vec<String>>",
            (qty)::BIGINT as "qty!: i64",
            is_cut,
            runner_id as "runner_id!: i64",
            kit_part_id as "kit_part_id!: i64",
            user_id as "user_id!: i64"
        "#,
        gate_json_opt,
        payload.qty,
        payload.is_cut,
        payload.runner_id,
        id,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(KitPartRequirement {
        id: row.id,
        gate: row.gate.0,
        qty: row.qty,
        is_cut: row.is_cut,
        runner_id: row.runner_id,
        kit_part_id: row.kit_part_id,
        user_id: row.user_id,
    })
}

pub async fn update_kit_part_requirement_is_cut(
    pool: &PgPool,
    id: i64,
    user_id: i64,
    is_cut: bool,
) -> Result<KitPartRequirement, Error> {
    let row = sqlx::query!(
        r#"
        UPDATE kit_part_requirements
        SET is_cut = $1
        WHERE id = $2 AND user_id = $3
        RETURNING
            id as "id!: i64",
            gate as "gate: sqlx::types::Json<Vec<String>>",
            (qty)::BIGINT as "qty!: i64",
            is_cut,
            runner_id as "runner_id!: i64",
            kit_part_id as "kit_part_id!: i64",
            user_id as "user_id!: i64"
        "#,
        is_cut,
        id,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(KitPartRequirement {
        id: row.id,
        gate: row.gate.0,
        qty: row.qty,
        is_cut: row.is_cut,
        runner_id: row.runner_id,
        kit_part_id: row.kit_part_id,
        user_id: row.user_id,
    })
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

pub async fn bulk_create_requirements(
    pool: &PgPool,
    user_id: i64,
    payload: crate::model::kit_part::BulkCreateRequirementsPayload,
) -> Result<Vec<KitPartRequirement>, Error> {
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;
    let mut out = Vec::with_capacity(payload.items.len());

    for item in payload.items {
        let row = sqlx::query(
            r#"
            INSERT INTO kit_part_requirements (gate, qty, is_cut, runner_id, kit_part_id, user_id)
            VALUES ($1::JSONB, $2, $3, $4, $5, $6)
            RETURNING
                id,
                (gate)::TEXT AS gate_text,
                (qty)::BIGINT AS qty,
                is_cut,
                runner_id,
                kit_part_id,
                user_id
            "#,
        )
        .bind(serde_json::json!(item.gate))
        .bind(item.qty)
        .bind(item.is_cut.unwrap_or(false))
        .bind(item.runner_id)
        .bind(payload.kit_part_id)
        .bind(user_id)
        .fetch_one(&mut *tx)
        .await?;

        let id: i64 = row.try_get("id")?;
        let gate_text: String = row.try_get("gate_text")?;
        let qty: i64 = row.try_get("qty")?;
        let is_cut_val: bool = row.try_get("is_cut")?;
        let runner_id_val: i64 = row.try_get("runner_id")?;
        let kit_part_id_val: i64 = row.try_get("kit_part_id")?;
        let user_id_val: i64 = row.try_get("user_id")?;
        let gate_vec: Vec<String> = serde_json::from_str(&gate_text).unwrap_or_default();

        out.push(KitPartRequirement {
            id,
            gate: gate_vec,
            qty,
            is_cut: is_cut_val,
            runner_id: runner_id_val,
            kit_part_id: kit_part_id_val,
            user_id: user_id_val,
        });
    }

    tx.commit().await?;
    Ok(out)
}

pub async fn bulk_update_requirements(
    pool: &PgPool,
    user_id: i64,
    payload: crate::model::kit_part::BulkUpdateRequirementsPayload,
) -> Result<Vec<KitPartRequirement>, Error> {
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;
    let mut out = Vec::with_capacity(payload.items.len());

    for item in payload.items {
        let gate_json_opt = item.gate.map(|g| serde_json::json!(g));

        let row = sqlx::query(
            r#"
            UPDATE kit_part_requirements
            SET
                gate = COALESCE($1::JSONB, gate),
                qty = COALESCE($2, qty),
                is_cut = COALESCE($3, is_cut),
                runner_id = COALESCE($4, runner_id)
            WHERE id = $5 AND user_id = $6
            RETURNING
                id,
                (gate)::TEXT AS gate_text,
                (qty)::BIGINT AS qty,
                is_cut,
                runner_id,
                kit_part_id,
                user_id
            "#,
        )
        .bind(gate_json_opt)
        .bind(item.qty)
        .bind(item.is_cut)
        .bind(item.runner_id)
        .bind(item.id)
        .bind(user_id)
        .fetch_one(&mut *tx)
        .await?;

        let id: i64 = row.try_get("id")?;
        let gate_text: String = row.try_get("gate_text")?;
        let qty: i64 = row.try_get("qty")?;
        let is_cut_val: bool = row.try_get("is_cut")?;
        let runner_id_val: i64 = row.try_get("runner_id")?;
        let kit_part_id_val: i64 = row.try_get("kit_part_id")?;
        let user_id_val: i64 = row.try_get("user_id")?;
        let gate_vec: Vec<String> = serde_json::from_str(&gate_text).unwrap_or_default();

        out.push(KitPartRequirement {
            id,
            gate: gate_vec,
            qty,
            is_cut: is_cut_val,
            runner_id: runner_id_val,
            kit_part_id: kit_part_id_val,
            user_id: user_id_val,
        });
    }

    tx.commit().await?;
    Ok(out)
}
