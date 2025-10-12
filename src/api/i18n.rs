// src/api/i18n.rs (สร้างไฟล์ใหม่)

use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::IntoResponse,
};

// Struct เพื่อดึงค่า Dynamic จาก Path: /i18n/:lng/:ns.json
#[derive(Debug, serde::Deserialize)]
pub struct I18nParams {
    lng: String,
    ns: String,
}

// ----------------------------------------------------
// Handler: Serve ไฟล์แปลภาษา
// ----------------------------------------------------

pub async fn serve_i18n_file(
    Path(params): Path<I18nParams>,
) -> Result<impl IntoResponse, StatusCode> {
    // 1. สร้าง Path ของไฟล์ที่คาดหวัง
    // เช่น "i18n/th/common.json"
    let file_path = format!("i18n/{}/{}.json", params.lng, params.ns);

    // 2. อ่านไฟล์ JSON แบบ Asynchronously
    let content = tokio::fs::read_to_string(&file_path).await.map_err(|e| {
        // หากไม่พบไฟล์หรือเข้าถึงไม่ได้ ให้คืนค่า 404
        eprintln!("i18n file error for {}: {}", file_path, e);
        StatusCode::NOT_FOUND
    })?;

    // 3. ส่ง Response พร้อม Header ที่ถูกต้อง
    let response = (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        content,
    );

    Ok(response)
}
