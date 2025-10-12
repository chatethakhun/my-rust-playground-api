// src/model/user.rs

use serde::{Deserialize, Serialize};

// User Struct (Database Schema)
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub username: String,
    pub password: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: String,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub full_name: Option<String>,
}

// Struct สำหรับรับข้อมูล Login
#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

// Struct สำหรับ Response ทั่วไป
#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub message: String,
}
