// src/models/sub_assembly.rs

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// --- Main Model: SubAssembly ---
#[derive(Debug, Serialize, Clone, FromRow)]
pub struct SubAssembly {
    pub id: i64,
    pub name: String,
    pub kit_id: i64,
    pub user_id: i64,
    pub created_at: NaiveDateTime, // 👈 เพิ่ม Type
    pub updated_at: NaiveDateTime, // 👈 เพิ่ม Type
}

// --- Payloads ---
#[derive(Debug, Deserialize)]
pub struct CreateSubAssemblyPayload {
    pub name: String,
    pub kit_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubAssemblyPayload {
    pub name: Option<String>,
    pub kit_id: Option<i64>,
}
