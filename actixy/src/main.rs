use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn rate() -> impl Responder {
    HttpResponse::Ok().body("rate")
}

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(rate))
            .route(web::get().to(|| HttpResponse::MethodNotAllowed())),
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| HttpResponse::Ok().body("app")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route("/", web::get().to(|| HttpResponse::Ok().body("/")))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
