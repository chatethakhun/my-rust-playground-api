// src/handlers/kit_part_handler.rs
// src/handlers/kit_part.rs

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, patch, post},
    Json, Router,
};
use serde::Deserialize;
use sqlx::Error as SqlxError;

use crate::state::AppState;
use crate::{
    middleware::auth::AuthUser,
    model::{
        kit_part::{CreateKitPartPayload, KitPart, KitPartRequirement, UpdateKitPartIsCutPayload},
        requirement::KitPartWithRequirements,
    },
};
use crate::{
    model::requirement::KitPartRequirementWithRunner,
    repository::kit_part::{
        create_kit_part, delete_kit_part, get_all_kit_parts_for_sub_assembly,
        get_all_requirements_for_kit_part, get_all_requirements_with_join_runner_for_kit_part,
        get_kit_part_by_id, get_kit_part_by_id_with_requirements, update_kit_part_is_cut,
    },
};

#[derive(Deserialize)]
pub struct SubAssemblyIdQuery {
    sub_assembly_id: i64,
}

// --- KitPart Handlers ---

pub async fn create_kit_part_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateKitPartPayload>,
) -> Result<(StatusCode, Json<KitPart>), (StatusCode, String)> {
    match create_kit_part(&state.db_pool, auth_user.user_id, payload).await {
        Ok(part) => Ok((StatusCode::CREATED, Json(part))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_kit_parts_by_sub_assembly_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<SubAssemblyIdQuery>,
) -> Result<Json<Vec<KitPart>>, (StatusCode, String)> {
    match get_all_kit_parts_for_sub_assembly(
        &state.db_pool,
        query.sub_assembly_id,
        auth_user.user_id,
    )
    .await
    {
        Ok(parts) => Ok(Json(parts)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn delete_kit_part_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    match delete_kit_part(&state.db_pool, id, auth_user.user_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(SqlxError::RowNotFound) => {
            Err((StatusCode::NOT_FOUND, "Kit part not found".to_string()))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn update_kit_part_is_cut_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateKitPartIsCutPayload>,
) -> Result<Json<KitPart>, (StatusCode, String)> {
    match update_kit_part_is_cut(&state.db_pool, id, auth_user.user_id, payload.is_cut).await {
        Ok(part) => Ok(Json(part)),
        Err(SqlxError::RowNotFound) => {
            Err((StatusCode::NOT_FOUND, "Kit part not found".to_string()))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
pub async fn get_kit_part_by_id_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<KitPart>, (StatusCode, String)> {
    match get_kit_part_by_id(&state.db_pool, id, auth_user.user_id).await {
        Ok(part) => Ok(Json(part)),
        Err(SqlxError::RowNotFound) => {
            Err((StatusCode::NOT_FOUND, "Kit part not found".to_string()))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_kit_part_by_id_with_requirements_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<KitPartWithRequirements>, (StatusCode, String)> {
    match get_kit_part_by_id_with_requirements(&state.db_pool, id, auth_user.user_id).await {
        Ok(part) => Ok(Json(part)),
        Err(SqlxError::RowNotFound) => {
            Err((StatusCode::NOT_FOUND, "Kit part not found".to_string()))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_requirements_by_kit_part_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(kit_part_id): Path<i64>,
) -> Result<Json<Vec<KitPartRequirement>>, (StatusCode, String)> {
    match get_all_requirements_for_kit_part(&state.db_pool, kit_part_id, auth_user.user_id).await {
        Ok(reqs) => Ok(Json(reqs)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_all_requirements_with_join_runner_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(kit_part_id): Path<i64>,
) -> Result<Json<Vec<KitPartRequirementWithRunner>>, (StatusCode, String)> {
    match get_all_requirements_with_join_runner_for_kit_part(
        &state.db_pool,
        kit_part_id,
        auth_user.user_id,
    )
    .await
    {
        Ok(reqs) => Ok(Json(reqs)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub fn kit_part_router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(create_kit_part_handler).get(get_kit_parts_by_sub_assembly_handler),
        )
        .route(
            "/:id",
            get(get_kit_part_by_id_handler).delete(delete_kit_part_handler),
        )
        .route("/:id/is_cut", patch(update_kit_part_is_cut_handler))
        .route(
            "/:id/with_requirements",
            get(get_kit_part_by_id_with_requirements_handler),
        )
        // Route สำหรับดึง requirements ทั้งหมดของ part นั้นๆ
        .route(
            "/:id/requirements",
            get(get_requirements_by_kit_part_handler),
        )
        .route(
            "/:id/requirements_with_runners",
            get(get_all_requirements_with_join_runner_handler),
        )
}
