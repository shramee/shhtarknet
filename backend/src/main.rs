mod endpoints;
mod secrets;
mod starknet;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    endpoints::setup().await
}
