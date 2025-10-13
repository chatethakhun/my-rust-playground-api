// src/middleware/auth.rs

use crate::model::jwt::Claims;
use crate::state::AppState;
use async_trait::async_trait; // 👈 ต้องมี Dependency นี้ใน Cargo.toml
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;

// Struct ที่จะใช้เป็น Extractor ใน Handler
#[derive(Debug, Deserialize, Clone)]
pub struct AuthUser {
    pub user_id: i64, // 👈 เก็บ ID ที่ดึงมาจาก JWT Claims
}

// ----------------------------------------------------
// 1. นำ Token ออกจาก Header
// ----------------------------------------------------

// Implement Trait FromRequestParts เพื่อให้ Struct นี้เป็น Extractor
// #[async_trait] ทำให้เราสามารถใช้ async fn ใน Trait ได้
#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    // Type ที่จะถูกส่งกลับเมื่อเกิด Error (401 Unauthorized)
    type Rejection = StatusCode;

    // ฟังก์ชันที่รันก่อน Handler
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 1. ดึง Authorization Header ออกมา
        let header_value = parts.headers.get("authorization");

        let token = match header_value {
            Some(value) => {
                let s = value.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;
                // ตรวจสอบรูปแบบ "Bearer <token>" และดึงเฉพาะส่วน <token>
                if s.starts_with("Bearer ") {
                    &s[7..]
                } else {
                    return Err(StatusCode::UNAUTHORIZED); // 401: รูปแบบ Header ผิด
                }
            }
            None => return Err(StatusCode::UNAUTHORIZED), // 401: ไม่พบ Header
        };

        // 2. ถอดรหัส (Decode) Token
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_ref()),
            &validation,
        )
        .map_err(|e| {
            eprintln!("JWT Decode Error: {}", e); // 👈 เพิ่ม Debugging Log
            StatusCode::UNAUTHORIZED
        })?;

        // 3. สร้าง AuthUser จาก Claims
        Ok(AuthUser {
            user_id: token_data.claims.sub, // ดึง 'sub' (username) จาก Payload
        })
    }
}
