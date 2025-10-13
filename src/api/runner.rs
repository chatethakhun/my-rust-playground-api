use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch, post},
    Json, Router,
};
use sqlx::Error as SqlxError;

// สมมติว่า import สิ่งที่จำเป็น
use crate::repository::runner::{
    create_runner, delete_runner, get_all_runners, get_runner_by_id, update_runner,
    update_runner_is_used,
};
use crate::state::AppState;
use crate::{
    middleware::auth::AuthUser,
    model::runner::{CreateRunnerPayload, Runner, UpdateIsUsedPayload, UpdateRunnerPayload},
};

// POST /runners
pub async fn create_runner_handler(
    State(state): State<AppState>,
    auth_user: AuthUser, // ได้จาก Auth Middleware
    Json(payload): Json<CreateRunnerPayload>,
) -> Result<(StatusCode, Json<Runner>), (StatusCode, String)> {
    match create_runner(&state.db_pool, auth_user.user_id, payload).await {
        Ok(runner) => Ok((StatusCode::CREATED, Json(runner))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

// GET /runners
pub async fn get_all_runners_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<Runner>>, (StatusCode, String)> {
    match get_all_runners(&state.db_pool, auth_user.user_id).await {
        Ok(runners) => Ok(Json(runners)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

// GET /runners/:id
pub async fn get_runner_by_id_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<Runner>, (StatusCode, String)> {
    match get_runner_by_id(&state.db_pool, id, auth_user.user_id).await {
        Ok(runner) => Ok(Json(runner)),
        Err(SqlxError::RowNotFound) => Err((StatusCode::NOT_FOUND, "Runner not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

// PATCH /runners/:id
pub async fn update_runner_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateRunnerPayload>,
) -> Result<Json<Runner>, (StatusCode, String)> {
    match update_runner(&state.db_pool, id, auth_user.user_id, payload).await {
        Ok(runner) => Ok(Json(runner)),
        Err(SqlxError::RowNotFound) => Err((StatusCode::NOT_FOUND, "Runner not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

// PATCH /runners/:id/is_used
pub async fn update_runner_is_used_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateIsUsedPayload>,
) -> Result<Json<Runner>, (StatusCode, String)> {
    match update_runner_is_used(&state.db_pool, id, auth_user.user_id, payload).await {
        Ok(runner) => Ok(Json(runner)),
        Err(SqlxError::RowNotFound) => Err((StatusCode::NOT_FOUND, "Runner not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

// DELETE /runners/:id
pub async fn delete_runner_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    match delete_runner(&state.db_pool, id, auth_user.user_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(SqlxError::RowNotFound) => Err((StatusCode::NOT_FOUND, "Runner not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub fn runner_router() -> Router<crate::state::AppState> {
    Router::new()
        .route(
            "/",
            post(create_runner_handler).get(get_all_runners_handler),
        )
        .route(
            "/:id",
            get(get_runner_by_id_handler)
                .patch(update_runner_handler)
                .delete(delete_runner_handler),
        )
        // 🚀 Route พิเศษสำหรับอัปเดต status
        .route("/:id/status", patch(update_runner_is_used_handler))
}
