
use crate::model::jwt::Claims;
use crate::model::user::{AuthResponse, LoginPayload, Message, User};
use crate::repository::user::create_user;
use crate::repository::user::find_by_username;
use crate::state::AppState;
use argon2::PasswordVerifier;
use jsonwebtoken::{encode, EncodingKey, Header};
// นำเข้า AppState
use axum::{extract::State, http::StatusCode, routing::post, Json, Router}; // 👈 นำเข้า Repository Function
                                                                           // สำหรับ Hashing
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher,
        // 🚨 ตัวที่ขาด: นำเข้า Salt เพื่อแปลงค่า
        SaltString, // 👈 ต้องนำเข้า SaltString
    },
    Argon2,
};

use mongodb::Database; // ... (นำเข้า argon2 และ mongodb ที่จำเป็น)

// Handler Login (ยกโค้ดจาก main.rs มาที่นี่)
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<(StatusCode, Json<AuthResponse>), StatusCode> {
    // ... โค้ด Login Logic ...
    let db = state.mongo_client.database(&state.db_name);

    // 2. ค้นหาผู้ใช้ผ่าน Repository (โค้ดสะอาดขึ้นมาก!)
    let user_doc = find_by_username(&db, &payload.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // หาก DB Error

    let user = match user_doc {
        Some(u) => u,
        None => return Err(StatusCode::UNAUTHORIZED), // ผู้ใช้ไม่พบ
    };
    // 3. เปรียบเทียบรหัสผ่าน (Password Verification)
    let is_valid = match argon2::password_hash::PasswordHash::new(&user.password) {
        // ... โค้ด Verify เดิม ...
        Ok(parsed_hash) => argon2::Argon2::default()
            .verify_password(payload.password.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    };

    if is_valid {
        // Login สำเร็จ
        let claims = Claims::new(user.username.clone(), 24);

        // 🚀 2. สร้าง JWT Token
        let token = encode(
            &Header::new(jsonwebtoken::Algorithm::HS256), // ใช้ HS256 เป็น Algorithm มาตรฐาน
            &claims,
            &EncodingKey::from_secret(state.jwt_secret.as_ref()),
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok((
            StatusCode::OK,
            Json(AuthResponse {
                token,
                message: format!("Login successful for user: {}", user.username),
            }),
        ))
    } else {
        // รหัสผ่านไม่ถูกต้อง
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>, // ใช้ LoginPayload ร่วมกัน
) -> Result<(StatusCode, Json<Message>), StatusCode> {
    let db: Database = state.mongo_client.database(&state.db_name);

    // 1. ตรวจสอบว่า Username ซ้ำหรือไม่
    let existing_user = find_by_username(&db, &payload.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_user.is_some() {
        // Username ถูกใช้งานแล้ว
        return Err(StatusCode::CONFLICT); // HTTP 409 Conflict
    }

    // 💡 แก้ไข: สร้าง SaltString จาก OsRng ก่อน
    let salt = SaltString::generate(&mut OsRng); // 👈 สร้าง Salt ด้วย OsRng

    // 2. Hashing รหัสผ่าน
    let password = payload.password.as_bytes();
    let password_hash = Argon2::default()
        .hash_password(password, &salt) // 👈 ส่ง salt.as_ref() แทน OsRng
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    // 3. สร้าง Struct User ใหม่
    let new_user = User {
        id: None, // ให้ MongoDB สร้าง ObjectId
        username: payload.username,
        password: password_hash,
        role: "user".to_string(),
        name: None,
        email: None,
        avatar: None,
        bio: None,
        full_name: None,
    };

    // 4. บันทึก User ผ่าน Repository
    match create_user(&db, new_user).await {
        Ok(_) => Ok((
            StatusCode::CREATED, // HTTP 201 Created
            Json(Message {
                message: "User registration successful.".to_string(),
            }),
        )),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// ฟังก์ชันรวม Routes (Option)
pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/register", post(register_handler))
}
