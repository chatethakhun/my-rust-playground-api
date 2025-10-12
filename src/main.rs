// src/main.rs

// บอก Rust ให้รู้จักโมดูลที่เราแยกไว้
mod api;
mod model;
mod repository;
mod state;

use axum::extract::State;
use axum::Json;
use axum::{routing::get, Router};
use dotenvy;
use mongodb::Client;

use crate::model::user::Message;
use crate::state::AppState; // นำเข้า Message สำหรับ Health Check

// Handler สำหรับ Health Check (สามารถย้ายไป api/health.rs ได้)
async fn mongo_health_check(State(_state): State<AppState>) -> Json<Message> {
    // ... โค้ด Health Check
    Json(Message {
        message: "Health Check OK".to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // 1. Setup State (Client DB Name)
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let db_name = std::env::var("MONGO_DATABASE_NAME").unwrap_or_else(|_| "auth_db".to_string());

    // สร้าง MongoDB Client (ต้องใช้ .await?)
    let client = Client::with_uri_str(&database_url).await?;

    // 2. สร้าง AppState struct (ตัวแปรที่หายไป)
    let app_state = AppState {
        mongo_client: client,
        db_name,
    };

    // 1. Setup State (Client, DB_Name)
    // ... (โค้ดการสร้าง client และ AppState)

    // 2. กำหนด Router
    let app: Router = Router::new()
        .route("/", get(|| async { "Axum API Running" }))
        // 🚀 รวม Routes จากโมดูลอื่น
        .nest("/auth", api::auth::auth_router()) // เรียกใช้ router จาก auth.rs
        .route("/health/mongo", get(mongo_health_check))
        .with_state(app_state);

    // 3. รัน Server
    // ... (โค้ดการรัน server)
    //
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("🚀 Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
