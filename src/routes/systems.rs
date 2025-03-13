use actix_web::{web, HttpRequest, HttpResponse};

use super::components::*;

pub async fn ping() -> HttpResponse {
    println!("Got ping, sending pong");
    HttpResponse::Ok().json(PongResponse::new())
}

pub async fn print_data(info: web::Json<DebugRequest>) -> HttpResponse {
    println!("Got data: {:#?}", info.message);
    HttpResponse::Ok().json(PongResponse::new())
}

pub async fn get_ip(req: HttpRequest) -> HttpResponse {
    let request = req;
    let ip_str_from_header = request
        .headers()
        .get("X-Forwarded-For")
        .and_then(|value| value.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim());

    let ip_str = match ip_str_from_header {
        Some(ip) => ip.to_string(), // Clone the string from the header
        None => {
            let connection_info = request.connection_info();
            connection_info
                .realip_remote_addr()
                .map(|s| s.to_string()) // Clone the string from connection info
                .unwrap_or_else(|| "0.0.0.0".to_string())
        }
    };

    HttpResponse::Ok().json(SimpleIPResponse { ip: ip_str })
}