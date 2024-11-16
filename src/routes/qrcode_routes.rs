use actix_web::{get, post, web::{Data, Json}, Responder};
use crate::services::qrcode_service;
use crate::AppState;
use crate::models::qrcode_models::CreateQrCodeBody;

#[get("/qrcode")]
pub async fn fetch_qrcodes(state: Data<AppState>) -> impl Responder {
    qrcode_service::fetch_qrcodes_logic(state).await
}

#[post("/qrcode")]
pub async fn create_qrcode(state: Data<AppState>, req_body: Json<CreateQrCodeBody>) -> impl Responder {
    qrcode_service::create_qrcode_logic(state, req_body).await
}