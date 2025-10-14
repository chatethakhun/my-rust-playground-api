use crate::model::user::{User, UserResponse};
use sqlx::{Error, SqlitePool}; // 🚨 ใช้ Error จาก sqlx

// 🚨 เปลี่ยน Return Type: ใช้ sqlx::Error แทน mongodb::error::Error
pub async fn find_by_username(pool: &SqlitePool, username: &str) -> Result<Option<User>, Error> {
    // ✅ ใช้ sqlx::Error

    // 1. 🚨 ใช้ SQL Query และ FromRow Macro
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = ?", username)
        .fetch_optional(pool) // ค้นหา 0 หรือ 1 แถวจาก Pool
        .await?; // ✅ จัดการ Error ของ SQLx

    // 2.return ข้อมูลที่พบหรือ None
    Ok(user)
}

pub async fn create_user(pool: &SqlitePool, new_user: User) -> Result<i64, Error> {
    // ✅ ใช้ sqlx::Error
    // 1. 🚨 ใช้ SQL Query และ FromRow Macro
    let result = sqlx::query!(
        "INSERT INTO users (username, password_hash, role, avatar_url, bio, full_name)
             VALUES (?, ?, ?, ?, ?, ?)",
        new_user.username,
        new_user.password_hash,
        new_user.role,
        new_user.avatar_url,
        new_user.bio,
        new_user.full_name,
    )
    .execute(pool) // 🚨 ใช้ execute() แทน fetch_optional()
    .await?;

    // 3. 🚀 ดึง Last Insert ID (Primary Key ที่ถูกสร้างอัตโนมัติ)
    let last_insert_id = result.last_insert_rowid();

    // 4. กำหนด ID กลับเข้าสู่ Struct User
    // ใน SQLite, ID จะเป็น i64

    // 2. คืนค่า Result
    Ok(last_insert_id) // ✅ คืนค่า SqliteQueryResult
}

// 2. 🚀 ฟังก์ชันบันทึกผู้ใช้ใหม่ (New Function)
// pub async fn create_user(
//     db: &SqlitePool,
//     new_user: User,
// ) -> Result<mongodb::results::InsertOneResult, mongodb::error::Error> {
//     // let collection: Collection<User> = db.collection("users");

//     // // บันทึก User (Struct) ลงใน MongoDB
//     // // เนื่องจาก User struct มี #[serde(skip_serializing_if = "Option::is_none")]
//     // // เราจึงส่ง None ใน id เพื่อให้ MongoDB สร้าง ObjectId ให้
//     // collection.insert_one(new_user, None).await
// }
pub async fn get_user_by_id(pool: &SqlitePool, id: i64) -> Result<UserResponse, Error> {
    // ✅ ใช้ sqlx::Error
    // 1. 🚨 ใช้ SQL Query และ FromRow Macro
    let user = sqlx::query_as!(
        UserResponse,
        "SELECT id, username, role, avatar_url, bio, full_name FROM users WHERE id = ?",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}
