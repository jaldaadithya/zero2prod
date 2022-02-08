use std::{io::{Error}, net::TcpListener};

use actix_web::{HttpServer, App, web, HttpRequest, Responder, HttpResponse, dev::Server};

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn check_health() -> impl Responder {
    HttpResponse::Ok()
}

async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    format!("Hello {}!", &form.name)
}

pub fn run(listener: TcpListener) -> Result<Server, Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/abc/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(check_health))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}