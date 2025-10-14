// src/models/kit.rs

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::model::runner::Runner;

// --- Enums ---
// 🚨 เพิ่ม derive macros ที่จำเป็นสำหรับ sqlx และ serde
// sqlx::Type บอกให้ sqlx รู้จัก enum นี้และ map กับ TEXT ใน DB
// Serialize/Deserialize บอกให้ serde แปลงเป็น JSON string ได้
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
// บอก sqlx ว่าจะเก็บ enum นี้เป็น TEXT ในฐานข้อมูล
#[sqlx(rename_all = "snake_case")] // ✅ เพิ่มบรรทัดนี้สำหรับ sqlx
#[serde(rename_all = "snake_case")] // บอก serde ให้ใช้ snake_case (เช่น "in_progress") ใน JSON
pub enum KitStatus {
    Pending,
    InProgress,
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
pub enum KitGrade {
    Eg,
    Hg,
    Rg,
    Mg,
    Mgsd,
    Pg,
    Other,
}

// --- Main Model: Kit ---
// โครงสร้างหลักที่ใช้ map กับตาราง `kits` และใช้ส่งข้อมูลกลับไปให้ client
#[derive(Debug, Serialize, Clone, FromRow)]
pub struct Kit {
    pub id: i64, // 👈 เมื่อดึงจาก DB จะมีค่าเสมอ
    pub name: String,
    pub grade: KitGrade,
    pub status: KitStatus,
    pub user_id: i64,
    pub created_at: NaiveDateTime, // 👈 เมื่อดึงจาก DB จะมีค่าเสมอ
    pub updated_at: NaiveDateTime,
}

// ✨ สร้าง Struct ใหม่สำหรับ Response โดยเฉพาะ
//
#[derive(Debug, Serialize)]
pub struct KitWithRunners {
    // ใช้ flatten attribute เพื่อให้ฟิลด์ทั้งหมดของ Kit ถูกใส่เข้ามาในระดับเดียวกัน
    #[serde(flatten)]
    pub kit: Kit,
    // เพิ่มฟิลด์ runners ที่เป็น Vec<Runner>
    pub runners: Vec<Runner>,
}

// --- Payload Structs ---
// Struct สำหรับรับข้อมูลจาก JSON request body เท่านั้น

// ใช้สำหรับสร้าง Kit ใหม่ (POST /kits)
#[derive(Debug, Deserialize)]
pub struct CreateKitPayload {
    pub name: String,
    pub grade: KitGrade,
}

// ใช้สำหรับอัปเดตข้อมูล Kit (PATCH /kits/:id)
// ทุกฟิลด์เป็น Option เพื่อรองรับการอัปเดตแค่บางส่วน
#[derive(Debug, Deserialize)]
pub struct UpdateKitPayload {
    pub name: Option<String>,
    pub grade: Option<KitGrade>,
}

// ใช้สำหรับอัปเดตเฉพาะ status (เช่น PATCH /kits/:id/status)
#[derive(Debug, Deserialize)]
pub struct UpdateStatusPayload {
    pub status: KitStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KitQuery {
    pub status: Option<KitStatus>,
}
