// src/models/runner.rs

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::model::color::RunnerColor;

// --- Main Model: Runner ---
#[derive(Debug, Serialize, Clone, FromRow)]
pub struct Runner {
    pub id: i64,
    pub name: String,
    pub kit_id: i64, // 👈 เพิ่มที่นี่
    pub color_id: i64,
    pub amount: i32,
    pub user_id: i64,
    pub is_used: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// --- Payload for Creating a new Runner ---
#[derive(Debug, Deserialize)]
pub struct CreateRunnerPayload {
    pub name: String,
    pub kit_id: i64, // 👈 เพิ่มที่นี่
    pub color_id: i64,
    pub amount: i32,
}

// --- Payload for Updating a Runner ---
#[derive(Debug, Deserialize)]
pub struct UpdateRunnerPayload {
    pub name: Option<String>,
    pub kit_id: Option<i64>, // 👈 เพิ่มที่นี่
    pub color_id: Option<i64>,
    pub amount: Option<i64>,
}

// --- Payload for Updating is_used status ---
#[derive(Debug, Deserialize)]
pub struct UpdateIsUsedPayload {
    pub is_used: bool,
}

// Struct สำหรับ Runner ที่มี Color object ซ้อนอยู่ข้างใน
#[derive(Debug, Serialize, Deserialize)]
pub struct RunnerWithColor {
    pub id: i64,
    pub name: String,
    pub kit_id: i64, // 👈 เพิ่มที่นี่
    pub amount: i32,
    pub user_id: i64,
    pub is_used: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub color: RunnerColor, // 👈 nested color object
}
