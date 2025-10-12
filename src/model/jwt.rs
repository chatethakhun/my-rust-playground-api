// src/model/jwt.rs

use chrono::Utc;
use serde::{Deserialize, Serialize}; // นำเข้า Utc

// Payload ที่จะใส่ใน JWT Token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (โดยทั่วไปคือ User ID หรือ Username)
    pub exp: i64,    // Expiration time (เวลาหมดอายุ)
    pub iat: i64,    // Issued At (เวลาที่สร้าง Token)
}

impl Claims {
    // ฟังก์ชันสำหรับสร้าง Claims ใหม่
    pub fn new(username: String, lifetime_hours: i64) -> Self {
        let now = Utc::now();
        let exp = now + chrono::Duration::hours(lifetime_hours);

        Self {
            sub: username,
            iat: now.timestamp(),
            exp: exp.timestamp(),
        }
    }
}
