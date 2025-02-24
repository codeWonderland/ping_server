use actix_web::{App, HttpServer};
use std::io;

mod routes;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // create webserver
    HttpServer::new(move || App::new().configure(routes::route_config))
        .bind("0.0.0.0:4569")?
        .run()
        .await
}
