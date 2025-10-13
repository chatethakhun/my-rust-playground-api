// src/handlers/sub_assembly_handler.rs

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use sqlx::Error as SqlxError;

use crate::repository::sub_assembly::{
    create_sub_assembly, delete_sub_assembly, get_all_sub_assemblies_for_kit,
    get_sub_assembly_by_id, update_sub_assembly,
};
use crate::state::AppState;
use crate::{
    middleware::auth::AuthUser,
    model::sub_assembly::{CreateSubAssemblyPayload, SubAssembly, UpdateSubAssemblyPayload},
};

#[derive(Deserialize)]
pub struct KitIdQuery {
    kit_id: i64,
}

pub async fn create_sub_assembly_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateSubAssemblyPayload>,
) -> Result<(StatusCode, Json<SubAssembly>), (StatusCode, String)> {
    match create_sub_assembly(&state.db_pool, auth_user.user_id, payload).await {
        Ok(sa) => Ok((StatusCode::CREATED, Json(sa))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_all_sub_assemblies_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<KitIdQuery>,
) -> Result<Json<Vec<SubAssembly>>, (StatusCode, String)> {
    match get_all_sub_assemblies_for_kit(&state.db_pool, query.kit_id, auth_user.user_id).await {
        Ok(sas) => Ok(Json(sas)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_sub_assembly_by_id_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<SubAssembly>, (StatusCode, String)> {
    match get_sub_assembly_by_id(&state.db_pool, id, auth_user.user_id).await {
        Ok(sa) => Ok(Json(sa)),
        Err(SqlxError::RowNotFound) => {
            Err((StatusCode::NOT_FOUND, "Sub-assembly not found".to_string()))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn update_sub_assembly_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateSubAssemblyPayload>,
) -> Result<Json<SubAssembly>, (StatusCode, String)> {
    match update_sub_assembly(&state.db_pool, id, auth_user.user_id, payload).await {
        Ok(sa) => Ok(Json(sa)),
        Err(SqlxError::RowNotFound) => {
            Err((StatusCode::NOT_FOUND, "Sub-assembly not found".to_string()))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn delete_sub_assembly_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    match delete_sub_assembly(&state.db_pool, id, auth_user.user_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(SqlxError::RowNotFound) => {
            Err((StatusCode::NOT_FOUND, "Sub-assembly not found".to_string()))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub fn sub_assembly_router() -> Router<crate::state::AppState> {
    Router::new()
        .route(
            "/",
            post(create_sub_assembly_handler).get(get_all_sub_assemblies_handler),
        )
        .route(
            "/:id",
            get(get_sub_assembly_by_id_handler)
                .patch(update_sub_assembly_handler)
                .delete(delete_sub_assembly_handler),
        )
    // 🚀 Route พิเศษสำหรับอัปเดต status
}
