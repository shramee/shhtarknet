#[actix_web::main]
async fn main() -> std::io::Result<()> {
    shhtarknet::endpoints::setup().await
}
