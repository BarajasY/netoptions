use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use dotenv::dotenv;

extern crate r2d2;

mod db;

#[derive(Serialize)]
struct Internet {
    name:String,
    price: i32
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::connect();


    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(return_json)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/echo")]
async fn return_json() -> web::Json<Internet> {
    web::Json(Internet {
        name: String::from("Totalplay"),
        price: 1300
    })
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
