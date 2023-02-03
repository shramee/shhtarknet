use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use secrets::{Secret, Secrets_Manager};

#[get("/secret/{contract}/{id}")]
pub async fn get_secret(data: web::Path<(String, String)>) -> impl Responder {
    let man = Secrets_Manager::new();
    HttpResponse::Ok().json(man.get(&data.0, &data.1))
}

pub async fn setup() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_secret))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
