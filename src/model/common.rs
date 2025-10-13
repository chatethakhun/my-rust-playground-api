use serde::Serialize;

// Struct สำหรับ Response ทั่วไป
#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub message: String,
}
