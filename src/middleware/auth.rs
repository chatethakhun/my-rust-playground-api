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
    type Rejection = (StatusCode, String); // 👈 เปลี่ยนเป็น tuple เพื่อส่ง error message

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 1. ดึง Authorization Header
        let header_value = parts.headers.get("authorization").ok_or((
            StatusCode::UNAUTHORIZED,
            "Missing Authorization header".to_string(),
        ))?;

        // 2. แปลงเป็น string
        let auth_str = header_value.to_str().map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization header".to_string(),
            )
        })?;

        // 3. ตรวจสอบ Bearer prefix
        let token = auth_str.strip_prefix("Bearer ").ok_or((
            StatusCode::UNAUTHORIZED,
            "Invalid Authorization format".to_string(),
        ))?;

        // 4. Decode JWT
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_ref()),
            &validation,
        )
        .map_err(|e| {
            eprintln!("JWT Decode Error: {:?}", e);
            (
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token".to_string(),
            )
        })?;

        // 5. สร้าง AuthUser
        Ok(AuthUser {
            user_id: token_data.claims.sub,
        })
    }
}
