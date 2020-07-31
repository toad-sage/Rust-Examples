use actix_web::http::ContentEncoding;
use actix_web::{middleware, web, App, HttpServer};

mod config;
mod db;
mod models;
mod repositories;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .wrap(middleware::Logger::default())
            .service(web::scope("/user").configure(repositories::user_repository::init_routes))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
