use actix_files::NamedFile;
use actix_session::{CookieSession, Session};
use actix_web::web;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Result};
use serde::Deserialize;
use std::path::PathBuf;

async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show Users")
}

async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.0))
}

async fn index(req: HttpRequest) -> Result<String> {
    let v1: u8 = req.match_info().get("id").unwrap().parse().unwrap();
    // or
    let (id, name): (u8, String) = req.match_info().load().unwrap();
    Ok(format!("Values {} {} {}", v1, id, name))
}

async fn index2(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome {} of age {}", info.id, info.name))
}

async fn index3(session: Session) -> Result<HttpResponse> {
    // access session data
    if let Some(count) = session.get::<i32>("counter")? {
        session.set("counter", count + 1)?;
    } else {
        session.set("counter", 1)?;
    }

    Ok(HttpResponse::Ok().body(format!(
        "Count is {:?}!",
        session.get::<i32>("counter")?.unwrap()
    )))
}

async fn index4(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[derive(Deserialize)]
struct Info {
    id: u8,
    name: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            // .route("/", web::get().to(index3))
            // .route("/{filename:.*}", web::get().to(index4))
            .service(
                web::scope("/users")
                    .route("/show", web::get().to(show_users))
                    .route("/show/{id}", web::get().to(user_detail))
                    .route("/show/{id}/{name}", web::get().to(index))
                    .route("/shooter/{id}/{name}", web::get().to(index2)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
