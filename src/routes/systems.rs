use actix_web::{web, HttpResponse};

use super::components::*;

pub async fn ping() -> HttpResponse {
    println!("Got ping, sending pong");
    HttpResponse::Ok().json(PongResponse::new())
}

pub async fn print_data(info: web::Json<DebugRequest>) -> HttpResponse {
    println!("Got data: {:#?}", info.message);
    HttpResponse::Ok().json(PongResponse::new())
}