use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SteamPriceResponse {
    pub app_id: u32,
    pub name: String,
    pub price: Option<f64>,
    pub currency: Option<String>,
    pub discount: Option<i32>,
    pub image: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct SteamApiPriceOverview {
    pub currency: Option<String>,
    #[serde(rename = "final")]
    pub final_: Option<i32>,
    pub discount_percent: Option<i32>,
}
#[derive(serde::Deserialize)]
pub struct SteamApiData {
    pub name: String,
    pub header_image: Option<String>,
    #[serde(rename = "price_overview")]
    pub price: Option<SteamApiPriceOverview>,
}
#[derive(serde::Deserialize)]
pub struct SteamApiResponse {
    pub success: bool,
    pub data: Option<SteamApiData>,
}

#[derive(serde::Deserialize)]
pub struct CreateSteamAppGamePayload {
    pub app_id: u32,
    pub name: String,
    pub steam_db_url: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct UpdateSteamAppGamePayload {
    pub name: Option<String>,
    pub steam_db_url: Option<String>,
    pub is_buy: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct SteamGame {
    pub id: i64,
    pub app_id: i64,
    pub name: String,
    pub steam_db_url: String,
    pub is_buy: bool,
    pub user_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
