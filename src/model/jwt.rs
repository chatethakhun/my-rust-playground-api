// src/model/jwt.rs

use chrono::Utc;
use serde::{Deserialize, Serialize}; // นำเข้า Utc
                                     // 🚨 ต้อง derive Clone เพื่อให้สามารถใช้ใน JWT decode/encode ได้อย่างยืดหยุ่น
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // Registered Claims (มาตรฐาน JWT)
    pub sub: String, // Subject: Username หรือ User ID
    pub exp: i64,    // Expiration time: เวลาหมดอายุ (Unix Timestamp)
    pub iat: i64,    // Issued At: เวลาที่สร้าง Token (Unix Timestamp)

                     // หากมี Field อื่นๆ ที่ต้องการใส่ใน Token ก็สามารถเพิ่มได้ที่นี่
}

impl Claims {
    /// สร้าง Claims ใหม่ โดยกำหนด sub และอายุการใช้งานเป็นชั่วโมง
    pub fn new(username: String, lifetime_hours: i64) -> Self {
        let now = Utc::now();

        // คำนวณเวลาหมดอายุ (ปัจจุบัน + จำนวนชั่วโมง)
        let duration = chrono::Duration::hours(lifetime_hours);
        let exp = now + duration;

        Self {
            sub: username,
            iat: now.timestamp(),
            exp: exp.timestamp(), // แปลงเป็น Unix Timestamp (i64)
        }
    }
}
