use axum::{extract::State, routing::get, Json, Router};
use dotenvy;
use mongodb::Client; // สำหรับสร้าง MongoDB Client
use serde::Serialize; // สำหรับโหลดค่าจาก .env

// Struct สำหรับ Response ทั่วไป
#[derive(Debug, Serialize, Clone)]
struct Message {
    message: String,
}

// Struct สำหรับ Application State ที่เก็บ MongoDB Client
#[derive(Clone)]
struct AppState {
    mongo_client: Client, // MongoDB Client
}

// ----------------------------------------------------
// HANDLERS
// ----------------------------------------------------

// Handler 1: Hello World
async fn root() -> Json<Message> {
    let response = Message {
        message: "Hello, World!".to_string(),
    };
    Json(response)
}

// Handler 2: ตรวจสอบสถานะการเชื่อมต่อ MongoDB
async fn mongo_health_check(
    State(state): State<AppState>, // ดึง AppState ออกมา
) -> Json<Message> {
    // ลองเชื่อมต่อกับ Admin Database เพื่อตรวจสอบสถานะ
    match state.mongo_client.list_database_names(None, None).await {
        Ok(_) => Json(Message {
            message: "MongoDB Atlas Connection: OK".to_string(),
        }),
        Err(e) => {
            eprintln!("MongoDB Health Check Failed: {}", e);
            // ตอบกลับด้วยสถานะ error
            Json(Message {
                message: format!("MongoDB Atlas Connection FAILED: {}", e),
            })
        }
    }
}

// ----------------------------------------------------
// MAIN FUNCTION
// ----------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 0. โหลด Environment Variables
    dotenvy::dotenv().ok();

    // 1. สร้าง MongoDB Client
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file (mongodb+srv://...)");

    // 🚀 แก้ไขแล้ว: ใช้ with_uri_str และ await อย่างถูกต้อง
    let client = Client::with_uri_str(&database_url).await?;

    // 2. สร้าง Application State
    let app_state = AppState {
        mongo_client: client,
    };

    // 3. กำหนด Router และฝัง State
    let app = Router::new()
        .route("/", get(root))
        .route("/health/mongo", get(mongo_health_check))
        .with_state(app_state); // ส่ง AppState เข้าไป

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("🚀 Server running on http://127.0.0.1:3000");
    println!("   - Base: http://127.0.0.1:3000/");
    println!("   - Health Check: http://127.0.0.1:3000/health/mongo");

    axum::serve(listener, app).await?;

    Ok(())
}
