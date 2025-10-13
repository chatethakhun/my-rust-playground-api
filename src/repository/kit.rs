// // src/api/kit.rs (ตัวอย่าง)

// use axum::{
//     extract::{Query, State},
//     http::StatusCode,
//     Json,
// };
// use chrono::Utc;
// use mongodb::{bson::doc, Collection, Database};
// use serde::Deserialize;

// use crate::model::kit::Kit; // Struct สำหรับ Kit
// use crate::state::AppState;
// use crate::{middleware::auth::AuthUser, model::kit::KitGrade}; // Extractor สำหรับ req.user.id

// use futures_util::TryStreamExt;

// // 1. Struct สำหรับรับ Query Parameters (req.query)
// #[derive(Debug, Deserialize)]
// pub struct KitQuery {
//     // Rust จะแปลงค่าจาก URL (?isFinished=true/false) เป็น Boolean ให้อัตโนมัติ
//     pub is_finished: Option<bool>,
// }

// #[derive(Debug, Deserialize)]
// pub struct CreateKitPayload {
//     pub name: String,
//     pub grade: KitGrade, // ใช้ String ชั่วคราว หรือใช้ KitGrade Enum ที่กำหนดไว้
//     pub manufacturer: Option<String>,
//     pub is_finished: Option<bool>,
//     // ไม่ต้องมี user field เพราะจะถูกเพิ่มโดย Handler
// }

// // 2. Handler Function
// // รับ State, AuthUser (แทน req.user.id), และ Query Params

// pub async fn get_kits_handler(
//     State(state): State<AppState>,

//     // 💡 แก้ไขตรงนี้: ใช้ชื่อตัวแปรโดยตรง
//     auth_user: AuthUser, // ✅ ถูกต้อง: ประกาศชื่อตัวแปรที่รับค่า AuthUser

//     Query(params): Query<KitQuery>,
// ) -> Result<Json<Vec<Kit>>, StatusCode> {
//     // ใช้งาน Database
//     let db: Database = state.mongo_client.database(&state.db_name);
//     let collection = db.collection::<Kit>("kits");

//     // 1. สร้าง Filter (เทียบเท่า Kit.find({ isFinished: ... }))
//     let mut filter = doc! {};

//     // ตรวจสอบ isFinished (req.query.isFinished)
//     if let Some(is_finished) = params.is_finished {
//         filter.insert("isFinished", is_finished);
//     }

//     // 2. เพิ่มเงื่อนไข .forUser(req.user.id)
//     // สมมติว่าใน MongoDB มี field ชื่อ userId เก็บ Owner ID ไว้
//     filter.insert("user", auth_user.username); // ใช้ username เป็น ID

//     // 3. กำหนด Sort (เทียบเท่า .sort({ updatedAt: -1 }))
//     let options = mongodb::options::FindOptions::builder()
//         .sort(doc! { "updatedAt": -1 }) // -1 คือ Descending
//         .build();

//     // 4. ดำเนินการค้นหา
//     let cursor = collection
//         .find(filter, options)
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     // 5. รวบรวมผลลัพธ์ (เทียบเท่า .lean() และ res.json(kits))
//     let kits: Vec<Kit> = cursor
//         .try_collect()
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     // 6. ส่ง Response
//     // การทำ .populate() จะต้องถูกทำแยกใน Rust ถ้าคุณไม่ใช้ ORM
//     // โดยทั่วไปจะทำใน Repository/Service Layer ก่อนส่งกลับ
//     Ok(Json(kits))
// }

// // 🚀 Handler Function: POST /kits
// pub async fn create_kit_handler(
//     State(state): State<AppState>,
//     auth_user: AuthUser,
//     Json(params): Json<CreateKitPayload>,
// ) -> Result<Json<Kit>, StatusCode> {
//     // ใช้งาน Database และ Collection
//     let db: Database = state.mongo_client.database(&state.db_pool);
//     let collection: Collection<Kit> = db.collection("kits");

//     // 1. สร้าง Kit Struct จาก Payload และ AuthUser
//     //
//     // // 1. สร้าง Kit Struct จาก Payload และ AuthUser

//     // 🚨 แก้ไข: ใช้ from_chrono() เพื่อแปลงค่า
//     let new_kit = Kit {
//         id: None, // ให้ MongoDB สร้าง
//         name: params.name,
//         grade: params.grade,
//         manufacturer: params.manufacturer,
//         is_finished: params.is_finished.unwrap_or(false),
//         // 🚨 แก้ไข: ใช้ username จาก AuthUser (ที่มาจาก JWT sub)]
//         updated_at: Utc::now(),
//         created_at: Utc::now(),
//         user: auth_user.username,
//         // runners: None,
//     };

//     // 2. ส่ง Kit กลับไปถึง MongoDB และจัดการ Error
//     let inserted_result = collection.insert_one(new_kit.clone(), None).await;

//     println!("New document inserted with ID: {:?}", inserted_result);

//     // handle error if insert_one() failed if duplicate

//     if inserted_result.is_err() {
//         return Err(StatusCode::INTERNAL_SERVER_ERROR);
//     }

//     Ok(Json(new_kit))

//     // 3. ส่ง Kit กลับไปถึง API และจัดการ Error
// }
