use actix_web::HttpResponse;

use super::components::PongResponse;

pub async fn ping() -> HttpResponse {
    println!("Got ping, sending pong");
    HttpResponse::Ok().json(PongResponse::new())
}