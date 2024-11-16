use ooriginal_qrcode_api::setup_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_server().await
}
