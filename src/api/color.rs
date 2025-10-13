use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
    Json, Router,
};
use sqlx::Error as SqlxError; // 💡 Alias SQLx Error เพื่อใช้ใน map_err

// Type Alias ที่คุณกำหนดไว้ (ใช้เหมือนเดิม)

use crate::{
    middleware::auth::AuthUser,
    model::{
        color::{Color, CreateColorPayload, UpdateColorPayload},
        common::Message,
    },
    repository::color::{create_color, get_colors, update_color},
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

pub async fn update_color_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    auth_user: AuthUser, // <-- ได้มาจาก Auth Middleware เช่น JWT
    Json(payload): Json<UpdateColorPayload>,
) -> Result<(StatusCode, Json<Color>), (StatusCode, Json<Message>)> {
    // 2. เรียก Repository เพื่ออัปเดตข้อมูล
    // เราส่ง id, user_id (สำหรับเช็ค ownership), และ payload เข้าไป
    let update_result = update_color(&state.db_pool, id, auth_user.user_id, payload).await;

    // 3. จัดการผลลัพธ์ (Ok หรือ Err)
    match update_result {
        Ok(updated_color) => {
            // สำเร็จ: คืนค่า Status 200 OK พร้อมข้อมูล Color ที่อัปเดตแล้ว
            Ok((StatusCode::OK, Json(updated_color)))
        }
        Err(e) => {
            // ไม่สำเร็จ: แปลง Error จาก database เป็น HTTP Status Code ที่เหมาะสม
            let status_code = match e {
                // 💡 ถ้าไม่เจอแถวที่จะอัปเดต (อาจเพราะ ID ผิด หรือไม่ใช่เจ้าของ)
                // ให้คืน 404 Not Found ซึ่งปลอดภัยและสื่อความหมายได้ดี
                SqlxError::RowNotFound => StatusCode::NOT_FOUND,
                // Error อื่นๆ ที่ไม่คาดคิด ถือเป็น Internal Server Error
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            let message = Message {
                message: format!("Failed to update color: {}", e),
            };

            Err((status_code, Json(message)))
        }
    }
}

// pub async fn get_color_by_id_handler(
//     State(state): State<AppState>,
//     Path(id): Path<i64>,
// ) -> Result<(StatusCode, Json<Color>), (StatusCode, Json<Color>)> {
//     match get_color_by_id(id).await {
//         Ok(color) => Ok((StatusCode::OK, Json(color))),

pub fn color_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_colors_handler).post(create_color_handler))
        .route("/:id", patch(update_color_handler))
}
