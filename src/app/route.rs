use actix_web::web;
use crate::app::api::non_dynamic;

use super::api::json_test;

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    cfg.service(
        web::scope("")
        .service(non_dynamic::greet)
        .service(non_dynamic::post_testing)
        .service(non_dynamic::get_name)
        .service(json_test::make_json)
        .route("/hey", web::get().to(non_dynamic::no_path_hello))
    )
}
