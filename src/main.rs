// src/main.rs

// บอก Rust ให้รู้จักโมดูลที่เราแยกไว้
mod api;
mod middleware;
mod model;
mod repository;
mod state;
use crate::api::i18n::serve_i18n_file;
use crate::model::user::Message;
use crate::state::AppState;
use axum::extract::State;
use axum::http::{self, header};
use axum::Json;
use axum::{routing::get, Router};
use dotenvy;
use mongodb::Client;
use std::time::Duration; // Optional: for max_age
use tokio::net::TcpListener;
use tower_http::cors::AllowOrigin; // 👈 For flexible origin control
use tower_http::cors::CorsLayer; // 👈 Import CorsLayer // 👈 ต้องนำเข้า TcpListener ด้วย // นำเข้า Message สำหรับ Health Check
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
    // 🚀 1. Configure the CORS policy
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list([
            "http://localhost:3000".parse().unwrap(),
            "https://playground-fe-xi.vercel.app".parse().unwrap(),
        ]))
        // 🚨 Set allowed HTTP methods (GET, POST, PUT, DELETE, OPTIONS, etc.)
        .allow_methods([http::Method::GET, http::Method::POST, http::Method::OPTIONS])
        // 🚨 Set allowed headers (Content-Type, Authorization, etc.)
        .allow_headers(vec![
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
        ])
        // อนุญาตให้ Credentials (Cookies, Auth Headers) ถูกส่งข้าม Domain
        .allow_credentials(true)
        // กำหนดระยะเวลาที่ Browser ควร Cache CORS policy (optional)
        .max_age(Duration::from_secs(60 * 60));
    // 1. Setup State (Client DB Name)
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let db_name = std::env::var("MONGO_DATABASE_NAME").unwrap_or_else(|_| "auth_db".to_string());
    // 🚀 ส่วนที่แก้ไข: การดึงค่า PORT

    // สร้าง MongoDB Client (ต้องใช้ .await?)
    let client = Client::with_uri_str(&database_url).await?;
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file");
    // 2. สร้าง AppState struct (ตัวแปรที่หายไป)
    let app_state = AppState {
        mongo_client: client,
        db_name,
        jwt_secret,
    };

    // 1. Setup State (Client, DB_Name)
    // ... (โค้ดการสร้าง client และ AppState)

    // 2. กำหนด Router
    let app: Router = Router::new()
        .route("/", get(|| async { "Axum API Running" }))
        // 🚀 รวม Routes จากโมดูลอื่น
        .route("/health/mongo", get(mongo_health_check))
        .route("/i18n/:lng/:ns", get(serve_i18n_file))
        .nest(
            "/v2/api",
            Router::new()
                .nest("/auth", api::auth::auth_router()) // URL: /v2/api/auth/...
                .nest("/kits", api::kit::kit_router()), // URL: /v2/api/kits/...
        )
        .layer(cors)
        .with_state(app_state.clone());

    // 3. รัน Server
    // ... (โค้ดการรัน server)
    //
    let port: u16 = std::env::var("PORT")
        // พยายามแปลงค่าจาก String เป็น u16
        .unwrap_or_else(|_| "3000".to_string()) // หากไม่พบ PORT ใน env ให้ใช้ "3000" เป็นค่าเริ่มต้น
        .parse()
        .expect("PORT must be a valid number (u16)"); // หากแปลงไม่ได้ (ไม่ใช่ตัวเลข) ให้ panic

    // 💡 Bind Address: ใช้ "0.0.0.0" เพื่อรับฟังทุก Network Interface
    let addr = format!("0.0.0.0:{}", port);

    // 1. กำหนด Address และ Port ที่ต้องการ Bind
    let listener = TcpListener::bind(&addr)
        .await
        .expect(&format!("Failed to bind TCP listener to {}", addr));

    println!("Listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
