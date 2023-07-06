use actix_web::{web::{Data, self}, App, HttpServer, middleware, http};
use app::{init::initialize, db::connection_pool};
use dotenv::dotenv;
use std::{env,io::Result};
use actix_cors::Cors;
use anyhow::Error;

extern crate r2d2;
pub mod app;

#[actix_web::main]
async fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    dotenv().ok();

    let pg_pool = Data::new(connection_pool());

    HttpServer::new(move || {
        let cors = Cors::default()
              .allow_any_origin()
              .allowed_methods(vec!["GET", "POST"]);

        App::new()
        .wrap(middleware::Logger::default())
        .wrap(cors)
        .app_data(pg_pool.clone())
            .configure(initialize)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
