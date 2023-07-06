use actix_web::web;

use super::{route::setup_routes, api::internet_provider::setup_provider_routes};

pub fn initialize(cfg: &mut web::ServiceConfig ) {
    setup_provider_routes(cfg);
    setup_routes(cfg);
}
