use crate::middleware::auth::AuthUser;
use crate::model::auth::AuthResponse;
use crate::model::jwt::Claims;
use crate::model::user::{User, UserResponse};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::repository::user::{create_user, find_by_username, get_user_by_id};

use crate::{model::auth::LoginPayload, state::AppState};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
}; // 👈 นำเข้า Repository Function
   //                                                                            // สำหรับ Hashing
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher,
        PasswordVerifier,
        // 🚨 ตัวที่ขาด: นำเข้า Salt เพื่อแปลงค่า
        SaltString, // 👈 ต้องนำเข้า SaltString
    },
    Argon2,
};

// Helper Type สำหรับ Result ที่ถูกต้อง
type HandlerResult<T> = Result<T, StatusCode>;

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> HandlerResult<Json<AuthResponse>> {
    // 1. ตรวจสอบว่า Username ซ้ำหรือไม่
    let jwt_secret: &str = &state.jwt_secret;
    let existing_user = find_by_username(&state.db_pool, &payload.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_user.is_some() {
        // 2. ค้นหาผู้ใช้ผ่าน Repository (โค้ดสะอาดขึ้นมาก!)
        let user = existing_user.unwrap();

        // 3. เปรียบเทียบรหัสผ่าน (Password Verification)
        let is_valid = match argon2::password_hash::PasswordHash::new(&user.password_hash) {
            // ... โค้ด Verify เดิม ...
            Ok(parsed_hash) => argon2::Argon2::default()
                .verify_password(payload.password.as_bytes(), &parsed_hash)
                .is_ok(),
            Err(_) => false,
        };

        if is_valid {
            // Login สำเร็จ
            // 4. สร้าง Claims: ใช้ .clone() เพื่อสร้างสำเนาของ username
            //    และ Move สำเนาเข้าไปใน Claims::new()

            let claims = Claims::new(user.id.unwrap(), 24);

            let token = encode(
                &Header::new(jsonwebtoken::Algorithm::HS256),
                &claims,
                &EncodingKey::from_secret(jwt_secret.as_ref()),
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // 5. ส่ง Response พร้อม Token
            Ok(Json(AuthResponse {
                message: "Login successful for user.".to_string(),
                token,
            }))
        } else {
            // รหัสผ่านไม่ถูกต้อง
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        // ผู้ใช้ไม่พบ
        Err(StatusCode::UNAUTHORIZED)
    }
}

// // Handler Login (ยกโค้ดจาก main.rs มาที่นี่)
// pub async fn login_handler(
//     State(state): State<AppState>,
//     Json(payload): Json<LoginPayload>,
// ) -> Result<(StatusCode, Json<AuthResponse>), StatusCode> {
//     // ... โค้ด Login Logic ...
//     let db = state.mongo_client.database(&state.db_name);

//     // 2. ค้นหาผู้ใช้ผ่าน Repository (โค้ดสะอาดขึ้นมาก!)
//     let user_doc = find_by_username(&db, &payload.username)
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // หาก DB Error

//     let user = match user_doc {
//         Some(u) => u,
//         None => return Err(StatusCode::UNAUTHORIZED), // ผู้ใช้ไม่พบ
//     };
//     // 3. เปรียบเทียบรหัสผ่าน (Password Verification)
//     let is_valid = match argon2::password_hash::PasswordHash::new(&user.password) {
//         // ... โค้ด Verify เดิม ...
//         Ok(parsed_hash) => argon2::Argon2::default()
//             .verify_password(payload.password.as_bytes(), &parsed_hash)
//             .is_ok(),
//         Err(_) => false,
//     };

//     if is_valid {
//         // Login สำเร็จ
//         let claims = Claims::new(user.username.clone(), 24);

//         // 🚀 2. สร้าง JWT Token
//         let token = encode(
//             &Header::new(jsonwebtoken::Algorithm::HS256), // ใช้ HS256 เป็น Algorithm มาตรฐาน
//             &claims,
//             &EncodingKey::from_secret(state.jwt_secret.as_ref()),
//         )
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
//         Ok((
//             StatusCode::OK,
//             Json(AuthResponse {
//                 token,
//                 message: format!("Login successful for user: {}", user.username),
//             }),
//         ))
//     } else {
//         // รหัสผ่านไม่ถูกต้อง
//         Err(StatusCode::UNAUTHORIZED)
//     }
// }

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>, // ใช้ LoginPayload ร่วมกัน
) -> HandlerResult<Json<AuthResponse>> {
    // 1. ตรวจสอบว่า Username ซ้ำหรือไม่
    let jwt_secret: &str = &state.jwt_secret;
    let existing_user = find_by_username(&state.db_pool, &payload.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_user.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let salt = SaltString::generate(&mut OsRng);

    let password = payload.password.as_bytes();
    let password_hash = Argon2::default()
        .hash_password(password, &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let new_user = User {
        id: None,
        username: payload.username.clone(),
        password_hash: password_hash,
        role: "user".to_string(),
        avatar_url: None,
        bio: None,
        full_name: None,
        created_at: None,
        updated_at: None,
    };

    match create_user(&state.db_pool, new_user).await {
        Ok(user_id) => {
            // 1. สร้าง Claims: ใช้ .clone() เพื่อสร้างสำเนาของ username
            //    และ Move สำเนาเข้าไปใน Claims::new()

            let claims = Claims::new(user_id, 24);

            let token = encode(
                &Header::new(jsonwebtoken::Algorithm::HS256),
                &claims,
                &EncodingKey::from_secret(jwt_secret.as_ref()),
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // 6. ส่ง Response พร้อม Token
            Ok(Json(AuthResponse {
                message: "User registration successful.".to_string(),
                token,
            }))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/v2/auth/me - ดึงข้อมูล user ที่ login อยู่
pub async fn get_auth_user_handler(
    State(state): State<AppState>,
    auth_user: AuthUser, // ✅ ดึง user_id จาก JWT token
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    match get_user_by_id(&state.db_pool, auth_user.user_id).await {
        Ok(user) => Ok(Json(user)),
        Err(sqlx::Error::RowNotFound) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ))
        }
    }
}

// ฟังก์ชันรวม Routes (Option)
pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/register", post(register_handler))
        .route("/me", get(get_auth_user_handler))
}
