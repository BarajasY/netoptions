use actix_web::web;

use super::route::setup_routes;

pub fn initialize(cfg: &mut web::ServiceConfig ) {
    setup_routes(cfg);
}
