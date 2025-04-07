use crate::routes::health_check;

use actix_web::{App, HttpServer, dev::Server, web};
use std::net::TcpListener;
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server =
        HttpServer::new(move || App::new().route("/health_check", web::get().to(health_check)))
            .listen(listener)?
            .run();
    Ok(server)
}
