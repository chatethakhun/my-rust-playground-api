//
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, patch, post},
    Json, Router,
};
use sqlx::Error as SqlxError;

// สมมติว่า import สิ่งที่จำเป็น

use crate::{
    middleware::auth::AuthUser,
    model::{
        kit::{KitQuery, KitWithRunners},
        kit_part::KitPartWithSubAssembly,
        runner::{Runner, RunnerWithColor},
        sub_assembly::SubAssembly,
    },
    repository::{
        kit_part::get_all_kit_parts_for_kit,
        runner::{get_all_runners_for_kit, get_all_runners_with_color_for_kit},
        sub_assembly::get_all_sub_assemblies_for_kit,
    },
    state::AppState,
};
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
    Query(params): Query<KitQuery>,
) -> Result<Json<Vec<Kit>>, (StatusCode, String)> {
    match get_all(&state.db_pool, auth_user.user_id, params.status).await {
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

// GET /kits/:kit_id/runners
pub async fn get_runners_by_kit_id_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(kit_id): Path<i64>, // 👈 รับ kit_id จาก Path
) -> Result<Json<Vec<Runner>>, (StatusCode, String)> {
    match get_all_runners_for_kit(&state.db_pool, kit_id, auth_user.user_id).await {
        Ok(runners) => Ok(Json(runners)),
        // คืนค่าเป็น array ว่างถ้าไม่เจอข้อมูล, ไม่ใช่ error
        Err(SqlxError::RowNotFound) => Ok(Json(vec![])),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_sub_assemblies_by_it_id_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(kit_id): Path<i64>, // 👈 รับ kit_id จาก Path
) -> Result<Json<Vec<SubAssembly>>, (StatusCode, String)> {
    match get_all_sub_assemblies_for_kit(&state.db_pool, kit_id, auth_user.user_id).await {
        Ok(sub_assemblies) => Ok(Json(sub_assemblies)),
        Err(SqlxError::RowNotFound) => Ok(Json(vec![])), // คืนค่า array ว่างถ้าไม่เจอ
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_kit_part_by_kit_id_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(kit_id): Path<i64>, // 👈 รับ kit_id จาก Path
) -> Result<Json<Vec<KitPartWithSubAssembly>>, (StatusCode, String)> {
    match get_all_kit_parts_for_kit(&state.db_pool, kit_id, auth_user.user_id).await {
        Ok(kit_parts) => Ok(Json(kit_parts)),
        Err(SqlxError::RowNotFound) => Ok(Json(vec![])), // คืนค่า array ว่างถ้าไม่เจอ
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
#[axum::debug_handler]
async fn get_all_runners_with_color_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(kit_id): Path<i64>, // 👈 รับ kit_id จาก Path
) -> Result<Json<Vec<RunnerWithColor>>, (StatusCode, String)> {
    match get_all_runners_with_color_for_kit(&state.db_pool, kit_id, auth_user.user_id).await {
        Ok(runners) => Ok(Json(runners)),
        Err(SqlxError::RowNotFound) => Ok(Json(vec![])), // คืนค่า array ว่างถ้าไม่เจอ
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
        .route("/:id/runners", get(get_runners_by_kit_id_handler))
        .route(
            "/:id/runner_colors",
            get(get_all_runners_with_color_handler),
        )
        .route(
            "/:id/sub_assemblies",
            get(get_sub_assemblies_by_it_id_handler),
        )
        .route("/:id/kit_parts", get(get_kit_part_by_kit_id_handler))
}

// // ฟังก์ชันรวม Routes (Option)
// pub fn kit_router() -> Router<AppState> {
//     Router::new()
//         .route("/", get(get_kits_handler))
//         .route("/", post(create_kit_handler))
// }
