pub mod components;
mod systems;

use actix_web::web;
use systems::*;

pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ping").route(web::get().to(ping)));
    cfg.service(web::resource("/debug").route(web::post().to(print_data)));
    cfg.service(web::resource("/ip").route(web::get().to(get_ip)));
}
