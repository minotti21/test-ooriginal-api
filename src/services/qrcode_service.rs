use actix_web::{http::Error, web::{self, Data}, HttpResponse};
use image::{DynamicImage, ImageOutputFormat, Luma};
use qrcode::QrCode;
use sqlx::Error as SqlxError;
use crate::{models::qrcode_models::{CreateQrCodeBody, GetQrCodeResponse, QrCodeDbResponse}, AppState};
use base64::{Engine as _, engine::general_purpose};

pub async fn fetch_qrcodes_logic(state: Data<AppState>) -> Result<HttpResponse, Error> {
    match sqlx::query_as::<_, QrCodeDbResponse>(
        "SELECT id, name, qr_code FROM qrcode_details"
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(qrcodes) => {
            let qrcodes_base64: Vec<GetQrCodeResponse> = qrcodes.into_iter().map(|qrcode| {
                GetQrCodeResponse {
                    id: qrcode.id,
                    name: qrcode.name,
                    qr_code_base64: general_purpose::STANDARD.encode(qrcode.qr_code),
                }
            }).collect();

            Ok(HttpResponse::Ok().json(qrcodes_base64))
        },
        Err(e) => {
            match e {
                SqlxError::RowNotFound => Ok(HttpResponse::NotFound().finish()),
                _ => Ok(HttpResponse::InternalServerError().json("Database error")),
            }
        },
    }
}

pub async fn create_qrcode_logic(
    state: Data<AppState>,
    body: web::Json<CreateQrCodeBody>,
) -> HttpResponse {
    let qr_code = match QrCode::new(&body.url) {
        Ok(code) => code,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to generate QR code"),
    };

    let image = qr_code.render::<Luma<u8>>().build();
    let dynamic_image = DynamicImage::ImageLuma8(image);

    let mut qr_code_bytes = Vec::new();
    match dynamic_image.write_to(&mut qr_code_bytes, ImageOutputFormat::Png) {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().body("Failed to convert QR code to bytes"),
    };

    match sqlx::query!(
        "INSERT INTO qrcode_details (name, description, url, additional_data, qr_code) VALUES ($1, $2, $3, $4, $5)",
        body.name,
        body.description,
        body.url,
        body.additional_data,
        qr_code_bytes
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            eprintln!("Failed to insert QR code into database: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to insert QR code into database")
        },
    }
}