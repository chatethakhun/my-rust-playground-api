use sqlx::{Error, SqlitePool};

use crate::model::color::Color;

pub async fn get_colors(pool: &SqlitePool, user_id: i64) -> Result<Vec<Color>, Error> {
    let colors_by_username =
        sqlx::query_as!(Color, "SELECT * FROM colors WHERE user_id = ?", user_id)
            .fetch_all(pool)
            .await?;

    let mut colors = vec![];
    for color in colors_by_username {
        colors.push(color);
    }

    Ok(colors)
}

pub async fn create_color(pool: &SqlitePool, color: Color) -> Result<Color, Error> {
    // 1. 🚀 INSERT: สร้างแถวใหม่และดึง ID
    // 💡 Note: เราใส่ user_id ใน color struct แต่ SQLx ต้องการมันใน query
    let result = sqlx::query!(
        "INSERT INTO colors (name, code, hex, is_clear, is_multi, user_id)
         VALUES (?, ?, ?, ?, ?, ?)",
        color.name,
        color.code,
        color.hex,
        color.is_clear,
        color.is_multi,
        color.user_id
    )
    .execute(pool)
    .await?;

    // 2. ดึง Last Insert ID
    let last_insert_id = result.last_insert_rowid();

    // 3. 🚀 SELECT: ดึง Object ที่สมบูรณ์กลับมา (รวม Timestamps)
    // ใช้ last_insert_id เพื่อค้นหาแถวที่เพิ่งสร้าง
    let created_color = sqlx::query_as!(
        Color,
        "SELECT id, name, code, hex, is_clear, is_multi, user_id, created_at, updated_at
         FROM colors WHERE id = ?",
        last_insert_id
    )
    .fetch_one(pool) // ต้อง fetch_one เพราะเราคาดหวังผลลัพธ์เดียว
    .await?;

    // 4. คืนค่า Color ที่สมบูรณ์
    Ok(created_color) // ✅ ถูกต้อง
}

// pub async fn get_color_by_id(id: i64) -> Result<Color, sqlx::Error> {
//     let color = sqlx::query_as!(Color, "SELECT * FROM colors WHERE id = ?", id)
//         .fetch_one(&mut *DB_POOL)
//         .await?;

//     Ok(color)
// }
