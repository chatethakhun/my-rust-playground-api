use crate::model::color::{Color, UpdateColorPayload};
use chrono::Utc;
use sqlx::{Error, SqlitePool};

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

pub async fn get_color_by_id(pool: &SqlitePool, color_id: i64) -> Result<Color, Error> {
    let color = sqlx::query_as!(
        Color,
        "SELECT id, name, code, hex, is_clear, is_multi, user_id, created_at, updated_at FROM colors WHERE id = ?",
        color_id
    )
    .fetch_one(pool)
    .await?;

    Ok(color)
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

// 🚀 Handler สำหรับอัปเดต Color

// เพื่อให้โค้ดสมบูรณ์ ควรกำหนด struct ของ payload ที่จะรับเข้ามา
// สมมติว่าหน้าตาเป็นแบบนี้ และอาจจะมาจาก JSON body

pub async fn update_color(
    pool: &SqlitePool,
    color_id: i64,
    user_id: i64,
    payload: UpdateColorPayload,
) -> Result<Color, Error> {
    // 👈 1. เปลี่ยน Return Type เป็น Result<Color, Error>

    // --- ส่วนที่ 1: UPDATE ข้อมูล ---
    let now = Utc::now().naive_utc();
    let result = sqlx::query!(
        r#"
        UPDATE colors
        SET
            name = COALESCE(?, name),
            code = COALESCE(?, code),
            hex = COALESCE(?, hex),
            is_clear = COALESCE(?, is_clear),
            is_multi = COALESCE(?, is_multi),
            updated_at = ?
        WHERE id = ? AND user_id = ?
        "#,
        payload.name,
        payload.code,
        payload.hex,
        payload.is_clear,
        payload.is_multi,
        now,
        color_id,
        user_id
    )
    .execute(pool)
    .await?;

    // 👈 2. ตรวจสอบว่ามีแถวถูกแก้ไขจริงหรือไม่
    if result.rows_affected() == 0 {
        // ถ้าไม่มีแถวไหนถูกแก้ไขเลย (อาจเพราะ id หรือ user_id ไม่ตรง)
        // ให้คืนค่า Error::RowNotFound เพื่อให้ handler แปลงเป็น 404 Not Found
        return Err(Error::RowNotFound);
    }

    // --- ส่วนที่ 2: SELECT ข้อมูลที่เพิ่งอัปเดตกลับมา ---
    // ใช้ sqlx::query_as! เพื่อ map ผลลัพธ์เข้า struct `Color` โดยอัตโนมัติ
    let updated_color = sqlx::query_as!(
        Color,
        "SELECT id, name, code, hex, is_clear, is_multi, user_id, created_at, updated_at FROM colors WHERE id = ?",
        color_id
    )
    .fetch_one(pool) // ดึงข้อมูลมาแค่ 1 แถวเท่านั้น
    .await?;

    // 👈 3. คืนค่า struct Color ที่สมบูรณ์
    Ok(updated_color)
}

pub async fn delete_color(pool: &SqlitePool, color_id: i64, user_id: i64) -> Result<(), Error> {
    // 👈 คืนค่าเป็น Result<(), Error> เพราะถ้าสำเร็จก็ไม่ต้องการข้อมูลใดๆ กลับมา

    let result = sqlx::query!(
        "DELETE FROM colors WHERE id = ? AND user_id = ?",
        color_id,
        user_id // 🛡️ ตรวจสอบความเป็นเจ้าของใน WHERE clause
    )
    .execute(pool)
    .await?;

    // ตรวจสอบว่ามีแถวถูกลบจริงหรือไม่
    if result.rows_affected() == 0 {
        // ถ้าไม่มีแถวไหนถูกลบเลย แสดงว่าไม่เจอข้อมูล (อาจเพราะ id หรือ user_id ไม่ตรง)
        // เราจะคืนค่าเป็น Error::RowNotFound เพื่อให้ handler นำไปใช้ต่อได้
        return Err(Error::RowNotFound);
    }

    // ถ้าลบสำเร็จ คืนค่า Ok ที่มี unit type `()` ซึ่งหมายถึง "สำเร็จแบบไม่มีข้อมูลจะส่งกลับ"
    Ok(())
}
