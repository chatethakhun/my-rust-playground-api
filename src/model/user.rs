// src/model/user.rs

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

// User Struct (Database Schema)
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub full_name: Option<String>,
    pub created_at: Option<NaiveDateTime>, // Stores creation time in UTC,
    pub updated_at: Option<NaiveDateTime>, // Stores creation time in UTC,
}

impl User {
    fn new(username: String, password_hash: String) -> Self {
        // 🚀 1. สร้างค่า DateTime<Utc> ปัจจุบัน
        let now = Utc::now();

        // 🚀 2. แปลงเป็น NaiveDateTime (โดยการตัด Timezone ออก)
        let naive_now = now.naive_utc();

        Self {
            id: None,
            username,
            password_hash,
            avatar_url: None,
            bio: None,
            role: "user".to_string(),
            full_name: None,

            // 3. กำหนดค่า: ห่อด้วย Some()
            created_at: Some(naive_now), // ✅ ถูกต้อง
            updated_at: Some(naive_now), // ✅ ถูกต้อง
        }
    }
}
