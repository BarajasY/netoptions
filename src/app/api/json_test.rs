use actix_web::{get, web};
use serde::Serialize;

#[derive(Serialize)]
pub struct InternetProvider {
    id: i32,
    name: String
}

#[get("/json/json")]
pub async fn make_json() -> web::Json<InternetProvider> {

    web::Json(InternetProvider {
        id: 1,
        name: String::from("Totalplay")
    })
}
