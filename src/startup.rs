use std::{net::TcpListener, io::Error};

use actix_web::{HttpServer, App, web, dev::Server};
use sqlx::{MySqlPool};
use crate::routes::{greet,check_health,subscribe};

pub fn run(listener: TcpListener, connection: MySqlPool) -> Result<Server, Error> {
    let conn = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/abc/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(check_health))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(conn.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}