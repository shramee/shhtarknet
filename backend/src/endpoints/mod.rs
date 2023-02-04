use crate::secrets::{Secret, SecretsManager};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/secret/{contract}/{id}")]
pub async fn get_secret(db_manager: web::Data<SecretsManager>,data: web::Path<(String, String)>) -> impl Responder {
	HttpResponse::Ok().json(db_manager.get(&data.0, &data.1))
}
}

pub async fn setup() -> std::io::Result<()> {
	let secrets_db = web::Data::new(SecretsManager::new());
    HttpServer::new(move || {
        App::new()
            .app_data(secrets_db.clone())
            .service(get_secret)
            .service(save_secret)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
