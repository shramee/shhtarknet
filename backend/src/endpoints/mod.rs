use crate::{
    secrets::{Secret, SecretsManager},
    starknet::felt_to_str,
};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use starknet::core::types::FieldElement;

#[get("/secret/{contract}/{id}")]
pub async fn get_secret(
    db_manager: web::Data<SecretsManager>,
    data: web::Path<(String, String)>,
) -> impl Responder {
    HttpResponse::Ok().json(db_manager.get(&data.0, &data.1))
}

#[get("/secrets")]
pub async fn get_secrets(db_manager: web::Data<SecretsManager>) -> impl Responder {
    let mut response: Vec<(String, String)> = vec![];

    for key in db_manager.db.iter().keys() {
        match key {
            Ok(key) => {
                let key_str = String::from_utf8(key.to_vec()).unwrap();
                let key: Vec<&str> = key_str.split("::").collect();
                response.push((
                    key[0].into(),
                    felt_to_str(&FieldElement::from_dec_str(key[1]).ok().unwrap()),
                ));
            }
            Err(_e) => {}
        }
    }

    HttpResponse::Ok().json(response)
}

#[post("/secret")]
async fn save_secret(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("{req_body:#?}"))
}

pub async fn setup() -> std::io::Result<()> {
    let secrets_db = web::Data::new(SecretsManager::new());
    HttpServer::new(move || {
        App::new()
            .app_data(secrets_db.clone())
            .service(get_secrets)
            .service(get_secret)
            .service(save_secret)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
