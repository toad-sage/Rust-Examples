use actix_web::get;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

async fn index2() -> impl Responder {
    std::thread::sleep(std::time::Duration::from_secs(7));
    HttpResponse::Ok().body("Hello World again!")
}

#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index4() -> impl Responder {
    HttpResponse::Ok().body("App there!")
}

#[get("/rate")]
async fn index5() -> impl Responder {
    HttpResponse::Ok().body("Inside App rate")
}

struct AppState {
    app_name: String,
}

async fn index6(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;

    format!("Hello {}!", app_name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Listening at port 8080");

    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(index3)
            .service(
                web::scope("/app")
                    .route("/", web::get().to(index4))
                    .service(index5)
                    .route("/test", web::get().to(index6)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
