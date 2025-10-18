use crate::middleware::auth::AuthUser;
use crate::model::kit_part::KitPartRequirement;
use crate::model::requirement::{
    BulkCreateRequirementsPayload, BulkDeleteRequirementsPayload, BulkSyncRequirementsPayload,
    BulkUpdateRequirementsPayload, CompareSyncRequirementsPayload, CreateKitPartRequirementPayload,
};
use crate::repository::requirement::{
    bulk_create_requirements, bulk_delete_requirements, bulk_sync_requirements,
    bulk_update_requirements, compare_sync_requirements, create_kit_part_requirement,
};

use sqlx::Error as SqlxError;

use crate::state::AppState;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{patch, post},
    Json, Router,
};

pub async fn compare_sync_requirements_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CompareSyncRequirementsPayload>,
) -> Result<Json<Vec<KitPartRequirement>>, (StatusCode, String)> {
    match compare_sync_requirements(&state.db_pool, auth_user.user_id, payload).await {
        Ok(reqs) => Ok(Json(reqs)),
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            "Some requirements not found".to_string(),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn bulk_delete_requirements_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<BulkDeleteRequirementsPayload>,
) -> Result<Json<u64>, (StatusCode, String)> {
    match bulk_delete_requirements(&state.db_pool, auth_user.user_id, payload.ids).await {
        Ok(count) => Ok(Json(count)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn bulk_create_requirements_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<BulkCreateRequirementsPayload>,
) -> Result<Json<Vec<KitPartRequirement>>, (StatusCode, String)> {
    match bulk_create_requirements(&state.db_pool, auth_user.user_id, payload).await {
        Ok(reqs) => Ok(Json(reqs)),
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            "Kit part not found for some items".to_string(),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn bulk_update_requirements_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<BulkUpdateRequirementsPayload>,
) -> Result<Json<Vec<KitPartRequirement>>, (StatusCode, String)> {
    match bulk_update_requirements(&state.db_pool, auth_user.user_id, payload).await {
        Ok(reqs) => Ok(Json(reqs)),
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            "Some requirements not found".to_string(),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn create_kit_part_requirement_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateKitPartRequirementPayload>,
) -> Result<(StatusCode, Json<KitPartRequirement>), (StatusCode, String)> {
    // We should probably verify ownership of the parent kit_part_id here in a real app
    match create_kit_part_requirement(&state.db_pool, auth_user.user_id, payload).await {
        Ok(req) => Ok((StatusCode::CREATED, Json(req))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

// pub async fn delete_kit_part_requirement_handler(
//     State(state): State<AppState>,
//     auth_user: AuthUser,
//     Path(id): Path<i64>,
// ) -> Result<StatusCode, (StatusCode, String)> {
//     match delete_kit_part_requirement(&state.db_pool, id, auth_user.user_id).await {
//         Ok(_) => Ok(StatusCode::NO_CONTENT),
//         Err(SqlxError::RowNotFound) => {
//             Err((StatusCode::NOT_FOUND, "Requirement not found".to_string()))
//         }
//         Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
//     }
// }

// pub async fn update_kit_part_requirement_handler(
//     State(state): State<AppState>,
//     auth_user: AuthUser,
//     Path(id): Path<i64>,
//     Json(payload): Json<UpdateKitPartRequirementPayload>,
// ) -> Result<Json<KitPartRequirement>, (StatusCode, String)> {
//     match update_kit_part_requirement(&state.db_pool, id, auth_user.user_id, payload).await {
//         Ok(req) => Ok(Json(req)),
//         Err(SqlxError::RowNotFound) => {
//             Err((StatusCode::NOT_FOUND, "Requirement not found".to_string()))
//         }
//         Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
//     }
// }

// --- KitPartRequirement Handlers ---

pub async fn bulk_sync_requirements_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<BulkSyncRequirementsPayload>,
) -> Result<Json<Vec<KitPartRequirement>>, (StatusCode, String)> {
    match bulk_sync_requirements(&state.db_pool, auth_user.user_id, payload).await {
        Ok(reqs) => Ok(Json(reqs)),
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            "Some requirements not found".to_string(),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub fn requirement_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_kit_part_requirement_handler))
        .route(
            "/bulk",
            post(bulk_create_requirements_handler).patch(bulk_update_requirements_handler),
        )
        .route("/sync", patch(bulk_sync_requirements_handler))
        .route("/compare_sync", patch(compare_sync_requirements_handler))
        .route("/bulk_delete", post(bulk_delete_requirements_handler))
    // .route(
    //     "/requirements/:id",
    //     delete(delete_kit_part_requirement_handler).patch(update_kit_part_requirement_handler),
    // )
}
