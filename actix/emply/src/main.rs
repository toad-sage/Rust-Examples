#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

async fn welcome(request: HttpRequest) -> impl Responder {
    let name = request.match_info().get("name").unwrap_or("world");
    format!("{}", name)
}

mod db;
mod employees;
mod error_handler;
mod schema;
mod teachers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(welcome))
            .configure(employees::init_routes)
            .configure(teachers::init_routes)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
