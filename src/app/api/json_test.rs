use actix_web::{get, web};

#[derive(serde::Serialize)]
pub struct InternetProvider {
    name: String,
    price: i32,
}

#[get("/json")]
pub async fn make_json() -> web::Json<InternetProvider> {
    web::Json(InternetProvider {
        name: String::from("Totalplay"),
        price: 1300,
    })
}
