// src/model/color.rs

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow; // 🚨 ต้องใช้ FromRow สำหรับ SQLx

// 🚨 Color Struct สำหรับ SQLx Mapping
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Color {
    // Primary Key (Auto-Increment ใน SQLite)
    pub id: Option<i64>,

    // ข้อมูลสี
    pub name: String, // 🚨 เพิ่ม name field ที่หายไป
    pub code: String,
    pub hex: String,

    // Boolean Fields (ใช้ bool ใน Rust)
    pub is_clear: bool,
    pub is_multi: bool,

    // 🚨 Foreign Key: ใช้ User ID (i64) แทน Struct User
    pub user_id: i64,

    // Timestamps (NaiveDateTime ถูกต้องแล้วสำหรับ SQLx/chrono)
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
#[derive(serde::Deserialize)]
pub struct CreateColorPayload {
    pub name: String,
    pub code: String,
    pub hex: String,
    pub is_clear: Option<bool>,
    pub is_multi: Option<bool>,
}
