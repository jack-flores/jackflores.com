use crate::routes::{health_check, post_show};

use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::PgConnection;
use std::net::TcpListener;
pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/shows", web::post().to(post_show))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
