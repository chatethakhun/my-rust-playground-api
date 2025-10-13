// src/models/paint.rs

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Clone, FromRow)]
pub struct Paint {
    pub id: i64,
    pub name: String,
    pub brand: Option<String>,
    pub code: Option<String>,
    pub user_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreatePaintPayload {
    pub name: String,
    pub brand: Option<String>,
    pub code: Option<String>,
}
