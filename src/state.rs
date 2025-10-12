// src/state.rs

use mongodb::Client;

#[derive(Clone)]
pub struct AppState {
    pub mongo_client: Client,
    pub db_name: String,
}
