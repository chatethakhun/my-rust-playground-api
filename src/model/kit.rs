// src/model/kit.rs

use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize}; // 👈 สำหรับเวลาปัจจุบัน

// 🚀 Struct Helper สำหรับค่า Default
// ฟังก์ชันนี้จะถูกเรียกเมื่อ Serde ไม่พบค่า updated_at ใน Payload

// ----------------------------------------------------
// ENUM: สำหรับ field 'grade' ที่มีค่าจำกัด
// ----------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")] // แปลงจาก Rust (SCREAMING) ไป MongoDB (EG, HG, etc.)
pub enum KitGrade {
    Eg,
    Hg,
    Rg,
    Mg,
    Pg,
    Other,
    Mgsd,
}

// ----------------------------------------------------
// STRUCT: Kit Model
// ----------------------------------------------------

// 🚀 Struct หลักสำหรับ Kit Model
#[derive(Debug, Serialize, Deserialize, Clone)]
// บอก MongoDB Driver ว่านี่คือ Struct ที่จะใช้ในการ Map ข้อมูล
pub struct Kit {
    // 1. _id (ObjectId)
    // - renames _id field จาก MongoDB เป็น id ใน Rust
    // - skip_serializing_if: ไม่ต้องส่ง id กลับไปถ้าเป็น None (ตอน insert)
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    // 2. name (String, required: true)
    pub name: String,

    // 3. grade (Enum, required: true)
    pub grade: KitGrade,

    // 4. manufacturer (String)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>, // ใช้ Option เพราะอาจเป็น null/undefined

    // 5. isFinished (Boolean, default: false)
    #[serde(default)] // ใช้ค่า default ถ้าไม่มีใน MongoDB
    pub is_finished: bool,

    // 6. user (สำหรับ KitSchema.plugin(withUser))
    // สมมติว่านี่คือ ObjectId ของ User หรือ Username String (ต้องตรงกับ Database)
    pub user: String,

    // 7. timestamps (timestamps: true)
    #[serde(default)]
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub updated_at: DateTime<Utc>,
    // 8. virtual field: runners (เทียบเท่า populate)
    // ใน Rust เราไม่สามารถทำ Virtual Field ได้โดยตรง
    // จึงใช้ Option<Vec<Runner>> เพื่อรองรับการ populate ภายหลัง
    // (ต้องสร้าง Struct Runner แยกต่างหาก)
    // #[serde(skip_serializing, default)] // ไม่ต้องเก็บ/ส่ง field นี้ไปยัง DB
    // pub runners: Option<Vec<Runner>>,
}

// **************** NOTE: Struct Runner (จำเป็นสำหรับการ populate) ****************
// คุณต้องสร้าง Struct สำหรับ Runner ด้วย (เช่น ใน src/model/runner.rs)
// เพื่อให้ Rust รู้จัก Type ของ Vec<Runner>
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Runner {
//     // กำหนด fields ของ Runner ตามความเป็นจริง
//     pub name: String,
//     pub kit: ObjectId, // Foreign Field
//                        // ...
// }
