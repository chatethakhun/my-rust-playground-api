use serde::{Deserialize, Serialize};

// Struct สำหรับรับข้อมูล Login
#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub message: String,
    pub token: String, // 👈 ส่ง JWT Token กลับไป
}
