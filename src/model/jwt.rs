// src/model/jwt.rs

use chrono::Utc;
use serde::{Deserialize, Serialize}; // นำเข้า Utc
                                     // 🚨 ต้อง derive Clone เพื่อให้สามารถใช้ใน JWT decode/encode ได้อย่างยืดหยุ่น
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // Registered Claims (มาตรฐาน JWT)
    pub sub: i64, // Subject: Username หรือ User ID
    pub exp: i64, // Expiration time: เวลาหมดอายุ (Unix Timestamp)
    pub iat: i64, // Issued At: เวลาที่สร้าง Token (Unix Timestamp)

                  // หากมี Field อื่นๆ ที่ต้องการใส่ใน Token ก็สามารถเพิ่มได้ที่นี่
}

impl Claims {
    pub fn new(user_id: i64, lifetime_hours: i64) -> Self {
        let now = Utc::now();
        let duration = chrono::Duration::hours(lifetime_hours);
        let exp = now + duration;
        Self {
            sub: user_id,
            iat: now.timestamp(),
            exp: exp.timestamp(),
        }
    }

    /// ตรวจสอบว่า token หมดอายุหรือยัง
    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        self.exp < now
    }

    /// ตรวจสอบว่า token ยังใช้งานได้ไหม (ไม่หมดอายุ และไม่ออกในอนาคต)
    pub fn is_valid(&self) -> bool {
        let now = Utc::now().timestamp();
        self.iat <= now && self.exp > now
    }
}
