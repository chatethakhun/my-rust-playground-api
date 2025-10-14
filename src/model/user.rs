// src/model/user.rs

use chrono::NaiveDateTime;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Option<i64>,
    pub username: String,
    pub role: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub full_name: Option<String>,
}
