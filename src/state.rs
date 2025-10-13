// src/state.rs

// 🚨 ต้องใช้ MySqlPool แทน Client

use sqlx::SqlitePool;
#[derive(Clone)]
pub struct AppState {
    pub db_pool: SqlitePool, // 🚨 เปลี่ยน Type
    pub jwt_secret: String,
}
