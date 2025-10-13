// use crate::repository::kit::{create_kit_handler, get_kits_handler};

// use axum::{routing::get, routing::post, Router};

// use crate::state::AppState;
//
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch, post},
    Json, Router,
};
use sqlx::Error as SqlxError;

// สมมติว่า import สิ่งที่จำเป็น

use crate::{middleware::auth::AuthUser, model::kit::KitWithRunners, state::AppState};
use crate::{
    model::kit::{CreateKitPayload, Kit, UpdateKitPayload, UpdateStatusPayload},
    repository::kit::{create, delete_kit, get_all, get_by_id, update, update_status},
};

// --- Handlers for CRUD ---

pub async fn create_kit_handler(
    State(state): State<AppState>,
    auth_user: AuthUser, // ได้จาก Auth Middleware
    Json(payload): Json<CreateKitPayload>,
) -> Result<(StatusCode, Json<KitWithRunners>), (StatusCode, String)> {
    match create(&state.db_pool, auth_user.user_id, payload).await {
        Ok(new_kit) => Ok((StatusCode::CREATED, Json(new_kit))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_all_kits_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<Kit>>, (StatusCode, String)> {
    match get_all(&state.db_pool, auth_user.user_id).await {
        Ok(kits) => Ok(Json(kits)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

// GET /kits/:id
pub async fn get_kit_by_id_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<KitWithRunners>, (StatusCode, String)> {
    // 👈 ใช้ Return Type ใหม่
    match get_by_id(&state.db_pool, id, auth_user.user_id).await {
        Ok(kit_with_runners) => Ok(Json(kit_with_runners)),
        Err(SqlxError::RowNotFound) => Err((StatusCode::NOT_FOUND, "Kit not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn update_kit_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateKitPayload>,
) -> Result<Json<KitWithRunners>, (StatusCode, String)> {
    match update(&state.db_pool, id, auth_user.user_id, payload).await {
        Ok(updated_kit) => Ok(Json(updated_kit)),
        Err(SqlxError::RowNotFound) => Err((StatusCode::NOT_FOUND, "Kit not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn update_kit_status_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateStatusPayload>,
) -> Result<Json<KitWithRunners>, (StatusCode, String)> {
    match update_status(&state.db_pool, id, auth_user.user_id, payload).await {
        Ok(updated_kit) => Ok(Json(updated_kit)),
        Err(SqlxError::RowNotFound) => Err((StatusCode::NOT_FOUND, "Kit not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn delete_kit_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    match delete_kit(&state.db_pool, id, auth_user.user_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(SqlxError::RowNotFound) => Err((StatusCode::NOT_FOUND, "Kit not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub fn kit_router() -> Router<crate::state::AppState> {
    Router::new()
        .route("/", post(create_kit_handler).get(get_all_kits_handler))
        .route(
            "/:id",
            get(get_kit_by_id_handler)
                .patch(update_kit_handler)
                .delete(delete_kit_handler),
        )
        // 🚀 Route พิเศษสำหรับอัปเดต status
        .route("/:id/status", patch(update_kit_status_handler))
}

// // ฟังก์ชันรวม Routes (Option)
// pub fn kit_router() -> Router<AppState> {
//     Router::new()
//         .route("/", get(get_kits_handler))
//         .route("/", post(create_kit_handler))
// }
