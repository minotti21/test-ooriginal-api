use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct QrCodeRecord {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub url: String,
    pub additional_data: String,
    pub qr_code: Vec<u8>,
}

#[derive(Deserialize)]
pub struct CreateQrCodeBody {
    pub name: String,
    pub description: String,
    pub url: String,
    pub additional_data: String,
}

#[derive(Serialize, FromRow)]
pub struct QrCodeDbResponse {
    pub id: i32,
    pub name: String,
    pub qr_code: Vec<u8>,
}

#[derive(Serialize)]
pub struct GetQrCodeResponse {
    pub id: i32,
    pub name: String,
    pub qr_code_base64: String,
}