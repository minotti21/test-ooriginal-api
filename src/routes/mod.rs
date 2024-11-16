mod qrcode_routes;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(qrcode_routes::fetch_qrcodes);
    cfg.service(qrcode_routes::create_qrcode);
}
