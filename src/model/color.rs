// src/model/color.rs

use mongodb::bson::{
    oid::ObjectId, // สำหรับ _id
    DateTime,      // สำหรับ timestamps
};
use serde::{Deserialize, Serialize};

// 🚀 Struct หลักสำหรับ Color Model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Color {
    // 1. _id
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    // 2. name (String, required: true)
    pub name: String,

    // 3. code (String)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>, // ใช้ Option เพราะอาจเป็น null/undefined

    // 4. hex (String)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>, // ใช้ Option เพราะอาจเป็น null/undefined

    // 5. multiple (Boolean, default: false)
    // #[serde(default)] จะทำให้ค่า default เป็น false หาก Field ไม่มีใน DB
    #[serde(default)]
    pub multiple: bool,

    // 6. clearColor (Boolean, default: false)
    #[serde(default)]
    pub clear_color: bool, // ใช้ snake_case ตามหลัก Rust

    // 7. user (สำหรับ ColorSchema.plugin(withUser))
    // สมมติว่าเก็บ User ID/Username เป็น String
    pub user: String,

    // 8. timestamps (timestamps: true)
    // #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime>,

    // #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime>,
}
