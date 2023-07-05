use actix_web::{web::{Data}, App, HttpServer, middleware};
use app::{init::initialize, db::connection_pool};
use dotenv::dotenv;
use serde::Serialize;

extern crate r2d2;

pub mod app;

#[derive(Serialize)]
struct Internet {
    name: String,
    price: i32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pg_pool = Data::new(connection_pool());

    HttpServer::new(move || {
        App::new()
        .app_data(pg_pool.clone())
            .configure(initialize)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
