use sqlx::{Error, PgPool, Row};

use crate::model::{
    kit_part::KitPartRequirement,
    requirement::{
        BulkCreateRequirementsPayload, BulkSyncRequirementsPayload, BulkUpdateRequirementsPayload,
        CompareSyncRequirementsPayload, CreateKitPartRequirementPayload,
    },
};

// pub async fn update_kit_part_requirement(
//     pool: &PgPool,
//     id: i64,
//     user_id: i64,
//     payload: UpdateKitPartRequirementPayload,
// ) -> Result<KitPartRequirement, Error> {
//     // Prepare optional JSON value for gate
//     let gate_json_opt = payload.gate.map(|g| serde_json::json!(g));

//     let row = sqlx::query!(
//         r#"
//         UPDATE kit_part_requirements
//         SET
//             gate = COALESCE($1::JSONB, gate),
//             qty = COALESCE($2, qty),
//             is_cut = COALESCE($3, is_cut),
//             runner_id = COALESCE($4, runner_id)
//         WHERE id = $5 AND user_id = $6
//         RETURNING
//             id as "id!: i64",
//             gate as "gate: sqlx::types::Json<Vec<String>>",
//             (qty)::BIGINT as "qty!: i64",
//             is_cut,
//             runner_id as "runner_id!: i64",
//             kit_part_id as "kit_part_id!: i64",
//             user_id as "user_id!: i64"
//         "#,
//         gate_json_opt,
//         payload.qty,
//         payload.is_cut,
//         payload.runner_id,
//         id,
//         user_id
//     )
//     .fetch_one(pool)
//     .await?;

//     Ok(KitPartRequirement {
//         id: row.id,
//         gate: row.gate.0,
//         qty: row.qty,
//         is_cut: row.is_cut,
//         runner_id: row.runner_id,
//         kit_part_id: row.kit_part_id,
//         user_id: row.user_id,
//     })
// }

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

pub async fn bulk_create_requirements(
    pool: &PgPool,
    user_id: i64,
    payload: BulkCreateRequirementsPayload,
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
    payload: BulkUpdateRequirementsPayload,
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

pub async fn bulk_sync_requirements(
    pool: &PgPool,
    user_id: i64,
    payload: BulkSyncRequirementsPayload,
) -> Result<Vec<KitPartRequirement>, Error> {
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;

    // 1) Delete
    for del_id in &payload.delete_ids {
        sqlx::query(
            r#"
            DELETE FROM kit_part_requirements
            WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(del_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;
    }

    // 2) Update
    for item in &payload.update {
        let gate_json_opt = item.gate.as_ref().map(|g| serde_json::json!(g));
        sqlx::query(
            r#"
            UPDATE kit_part_requirements
            SET
                gate = COALESCE($1::JSONB, gate),
                qty = COALESCE($2, qty),
                is_cut = COALESCE($3, is_cut),
                runner_id = COALESCE($4, runner_id)
            WHERE id = $5 AND user_id = $6
            "#,
        )
        .bind(gate_json_opt)
        .bind(item.qty)
        .bind(item.is_cut)
        .bind(item.runner_id)
        .bind(item.id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;
    }

    // 3) Create
    for item in &payload.create {
        sqlx::query(
            r#"
            INSERT INTO kit_part_requirements (gate, qty, is_cut, runner_id, kit_part_id, user_id)
            VALUES ($1::JSONB, $2, COALESCE($3, false), $4, $5, $6)
            "#,
        )
        .bind(serde_json::json!(item.gate))
        .bind(item.qty)
        .bind(item.is_cut)
        .bind(item.runner_id)
        .bind(payload.kit_part_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;
    }

    // 4) Read back all requirements for this kit_part_id
    let rows = sqlx::query(
        r#"
        SELECT
            id,
            (gate)::TEXT AS gate_text,
            (qty)::BIGINT AS qty,
            is_cut,
            runner_id,
            kit_part_id,
            user_id
        FROM kit_part_requirements
        WHERE kit_part_id = $1 AND user_id = $2
        ORDER BY id
        "#,
    )
    .bind(payload.kit_part_id)
    .bind(user_id)
    .fetch_all(&mut *tx)
    .await?;

    let mut out: Vec<KitPartRequirement> = Vec::with_capacity(rows.len());
    for row in rows {
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

pub async fn bulk_delete_requirements(
    pool: &PgPool,
    user_id: i64,
    ids: Vec<i64>,
) -> Result<u64, Error> {
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;
    let mut total: u64 = 0;

    for id in ids {
        let res = sqlx::query(
            r#"
            DELETE FROM kit_part_requirements
            WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

        total += res.rows_affected();
    }

    tx.commit().await?;
    Ok(total)
}

pub async fn compare_sync_requirements(
    pool: &PgPool,
    user_id: i64,
    payload: CompareSyncRequirementsPayload,
) -> Result<Vec<KitPartRequirement>, Error> {
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;

    // 1) Read existing IDs for this kit_part_id
    let existing_rows = sqlx::query(
        r#"
        SELECT id
        FROM kit_part_requirements
        WHERE kit_part_id = $1 AND user_id = $2
        "#,
    )
    .bind(payload.kit_part_id)
    .bind(user_id)
    .fetch_all(&mut *tx)
    .await?;

    let mut existing_ids: Vec<i64> = Vec::with_capacity(existing_rows.len());
    for row in existing_rows {
        let id: i64 = row.try_get("id")?;
        existing_ids.push(id);
    }

    let provided_ids: Vec<i64> = payload.items.iter().filter_map(|i| i.id).collect();

    // 2) Delete records that are missing from provided list
    for id in existing_ids {
        if !provided_ids.contains(&id) {
            sqlx::query(
                r#"
                DELETE FROM kit_part_requirements
                WHERE id = $1 AND user_id = $2
                "#,
            )
            .bind(id)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
        }
    }

    // 3) Update existing ones (items that include an id)
    for item in payload.items.iter().filter(|i| i.id.is_some()) {
        sqlx::query(
            r#"
            UPDATE kit_part_requirements
            SET
                gate = $1::JSONB,
                qty = $2,
                is_cut = COALESCE($3, is_cut),
                runner_id = $4
            WHERE id = $5 AND user_id = $6
            "#,
        )
        .bind(serde_json::json!(&item.gate))
        .bind(item.qty)
        .bind(item.is_cut)
        .bind(item.runner_id)
        .bind(item.id.unwrap())
        .bind(user_id)
        .execute(&mut *tx)
        .await?;
    }

    // 4) Create new ones (items without id)
    for item in payload.items.into_iter().filter(|i| i.id.is_none()) {
        sqlx::query(
            r#"
            INSERT INTO kit_part_requirements (gate, qty, is_cut, runner_id, kit_part_id, user_id)
            VALUES ($1::JSONB, $2, COALESCE($3, false), $4, $5, $6)
            "#,
        )
        .bind(serde_json::json!(item.gate))
        .bind(item.qty)
        .bind(item.is_cut)
        .bind(item.runner_id)
        .bind(payload.kit_part_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;
    }

    // 5) Read back all requirements for this kit_part_id
    let rows = sqlx::query(
        r#"
        SELECT
            id,
            (gate)::TEXT AS gate_text,
            (qty)::BIGINT AS qty,
            is_cut,
            runner_id,
            kit_part_id,
            user_id
        FROM kit_part_requirements
        WHERE kit_part_id = $1 AND user_id = $2
        ORDER BY id
        "#,
    )
    .bind(payload.kit_part_id)
    .bind(user_id)
    .fetch_all(&mut *tx)
    .await?;

    let mut out: Vec<KitPartRequirement> = Vec::with_capacity(rows.len());
    for row in rows {
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

// pub async fn _update_kit_part_requirement_is_cut(
//     pool: &PgPool,
//     id: i64,
//     user_id: i64,
//     is_cut: bool,
// ) -> Result<KitPartRequirement, Error> {
//     let row = sqlx::query!(
//         r#"
//         UPDATE kit_part_requirements
//         SET is_cut = $1
//         WHERE id = $2 AND user_id = $3
//         RETURNING
//             id as "id!: i64",
//             gate as "gate: sqlx::types::Json<Vec<String>>",
//             (qty)::BIGINT as "qty!: i64",
//             is_cut,
//             runner_id as "runner_id!: i64",
//             kit_part_id as "kit_part_id!: i64",
//             user_id as "user_id!: i64"
//         "#,
//         is_cut,
//         id,
//         user_id
//     )
//     .fetch_one(pool)
//     .await?;

//     Ok(KitPartRequirement {
//         id: row.id,
//         gate: row.gate.0,
//         qty: row.qty,
//         is_cut: row.is_cut,
//         runner_id: row.runner_id,
//         kit_part_id: row.kit_part_id,
//         user_id: row.user_id,
//     })
// }
//
