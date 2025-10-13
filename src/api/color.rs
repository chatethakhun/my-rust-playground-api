use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use sqlx::Error as SqlxError; // 💡 Alias SQLx Error เพื่อใช้ใน map_err

// Type Alias ที่คุณกำหนดไว้ (ใช้เหมือนเดิม)

use crate::{
    middleware::auth::AuthUser,
    model::{
        color::{Color, CreateColorPayload},
        common::Message,
    },
    repository::color::{create_color, get_colors},
    state::AppState,
};

pub async fn get_colors_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Json<Vec<Color>> {
    match get_colors(&state.db_pool, auth_user.user_id).await {
        Ok(colors) => Json(colors),
        Err(err) => {
            println!("Error: {}", err);
            Json(vec![])
        }
    }
}
#[axum::debug_handler]
async fn create_color_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateColorPayload>,
) -> Result<(StatusCode, Json<Color>), (StatusCode, Json<Message>)> {
    // 1. สร้าง new_color (Ownership/Move ถูกต้องแล้วด้วย Struct Update Syntax)
    let new_color = Color {
        id: None,
        name: payload.name,                          // Move
        code: payload.code,                          // Move
        hex: payload.hex,                            // Move
        is_clear: payload.is_clear.unwrap_or(false), // Option Handled
        is_multi: payload.is_multi.unwrap_or(false), // Option Handled
        user_id: auth_user.user_id,                  // User ID Handled
        created_at: None,
        updated_at: None,
    };

    // 2. เรียก Repository และจัดการ Error (Type Conversion ถูกต้อง)
    let created_color = create_color(&state.db_pool, new_color)
        .await
        .map_err(|e: SqlxError| {
            // ... Logic จัดการ 409 Conflict และ 500 Internal Error ...
            let status = if let Some(db_err) = e.as_database_error() {
                if db_err.message().contains("UNIQUE constraint failed") {
                    StatusCode::CONFLICT
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (
                status,
                Json(Message {
                    message: e.to_string(),
                }),
            )
        })?;

    // 3. ส่ง Response 201 Created
    Ok((StatusCode::CREATED, Json(created_color)))
}

// pub async fn get_color_by_id_handler(
//     State(state): State<AppState>,
//     Path(id): Path<i64>,
// ) -> Result<(StatusCode, Json<Color>), (StatusCode, Json<Color>)> {
//     match get_color_by_id(id).await {
//         Ok(color) => Ok((StatusCode::OK, Json(color))),

pub fn color_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_colors_handler))
        .route("/", post(create_color_handler))
}
