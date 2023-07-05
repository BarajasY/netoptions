use actix_web::{Responder, HttpResponse, get, web, post};

#[get("/")]
pub async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello there!")
}

#[get("/{name}")]
pub async fn get_name(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    HttpResponse::Ok().body(name)
}

#[post("/car")]
pub async fn post_testing(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn no_path_hello() -> impl Responder {
    HttpResponse::Ok().body("This is a manual hello world with no path")
}
