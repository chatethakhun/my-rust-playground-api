// src/repository/user.rs

use crate::model::user::User;
use mongodb::{bson::doc, Collection, Database};

// ฟังก์ชันนี้จะจัดการการติดต่อกับ MongoDB โดยเฉพาะ
pub async fn find_by_username(
    db: &Database,
    username: &str,
) -> Result<Option<User>, mongodb::error::Error> {
    let collection: Collection<User> = db.collection("users");

    // ค้นหาเอกสารเดียวที่ตรงกับ username
    collection
        .find_one(doc! { "username": username }, None)
        .await
}

// 2. 🚀 ฟังก์ชันบันทึกผู้ใช้ใหม่ (New Function)
pub async fn create_user(
    db: &Database,
    new_user: User,
) -> Result<mongodb::results::InsertOneResult, mongodb::error::Error> {
    let collection: Collection<User> = db.collection("users");

    // บันทึก User (Struct) ลงใน MongoDB
    // เนื่องจาก User struct มี #[serde(skip_serializing_if = "Option::is_none")]
    // เราจึงส่ง None ใน id เพื่อให้ MongoDB สร้าง ObjectId ให้
    collection.insert_one(new_user, None).await
}
