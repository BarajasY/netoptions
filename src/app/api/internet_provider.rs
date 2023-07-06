use actix_web::{
    get, post, web,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use crate::app::db::DbPool;

#[derive(Serialize, Deserialize)]
pub struct InternetProvider {
    name: String,
}

#[get("")]
pub async fn iprovider_index() -> impl Responder {
    HttpResponse::Ok().body(
        "Hello, this is an index for internetProviders, follow on to /add, /remove, /update...",
    )
}

#[post("/add")]
pub async fn add_provider(pool: Data<DbPool>, provider: Json<InternetProvider>) -> HttpResponse {
    let mut client = pool.get().unwrap();

    /*     HttpResponse::Ok().json(provider) */

    let adder =
        web::block(move || client.execute("INSERT INTO wifi_companies (name) VALUES ($1);", &[&provider.name])).await;

    match adder {
        Ok(adder) => match adder {
            Ok(adder) => HttpResponse::Ok().json(adder),
            Err(error) => HttpResponse::Conflict().body(error.to_string())
        },
        Err(error) => {
            println!("{}", error.to_string());
            HttpResponse::Conflict().body(error.to_string())
        }
    }
}

pub fn setup_provider_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    cfg.service(
        web::scope("/iprovider")
            .service(iprovider_index)
            .service(add_provider),
    )
}
