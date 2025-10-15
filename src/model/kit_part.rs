// src/models/kit_part.rs

use crate::model::sub_assembly::SubAssembly;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// --- Main Model: KitPart ---
#[derive(Debug, Serialize, Clone, FromRow)]
pub struct KitPart {
    pub id: i64,
    pub code: Option<String>,
    pub is_cut: bool,
    pub kit_id: i64,
    pub sub_assembly_id: i64,
    pub user_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// --- KitPart with SubAssembly (joined result) ---
#[derive(Debug, Serialize, Clone)]
pub struct KitPartWithSubAssembly {
    #[serde(flatten)]
    pub kit_part: KitPart,
    pub sub_assembly: SubAssembly,
}

// --- Related Model: KitPartRequirement ---
// ใน SQL, Nested Schema จะกลายเป็นตารางของตัวเองที่เชื่อมโยงกลับมา
#[derive(Debug, Serialize, Clone, FromRow)]
pub struct KitPartRequirement {
    pub id: i64,
    pub gate: String,
    pub qty: i64,
    pub is_cut: bool,
    pub runner_id: i64,   // Foreign key to runners table
    pub kit_part_id: i64, // Foreign key to kit_parts table
    pub user_id: i64,     // Foreign key to users table
}

// --- Payloads for KitPart ---
#[derive(Debug, Deserialize)]
pub struct CreateKitPartPayload {
    pub code: Option<String>,
    pub kit_id: i64,
    pub sub_assembly_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateKitPartPayload {
    pub code: Option<String>,
    pub is_cut: Option<bool>,
}

// --- Payload for KitPartRequirement ---
#[derive(Debug, Deserialize)]
pub struct CreateKitPartRequirementPayload {
    pub gate: String,
    pub qty: i32,
    pub runner_id: i64,
    pub kit_part_id: i64,
}
