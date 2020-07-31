use actix_web::{web, App, HttpServer, Responder};
use mongodb::Client;
// use mongodb::{options::ClientOptions};
use std::env;
use std::sync::*;

mod logs_handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUSR_LOG", "actix_web=debug");

    // let mut client_options = ClientOptions::parse("mongodb+srv://ruffle:10qpalzm@cluster0-fw7ue.mongodb.net/<dbname>?retryWrites=true&w=majority").await.unwrap();
    // client_options.app_name = Some("PlantApi".to_string());
    // let client = Client::with_options(client_options).unwrap();

    // or

    env::set_var("CONNECTION_STRING_LOGS", "mongodb+srv://ruffle:10qpalzm@cluster0-fw7ue.mongodb.net/avenger?retryWrites=true&w=majority");
    let mongo_url = env::var("CONNECTION_STRING_LOGS").unwrap();
    let client = Client::with_uri_str(&mongo_url).await.unwrap();
    println!("Connected To The Database");

    let client = web::Data::new(Mutex::new(client));

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(web::scope("/api").configure(logs_handlers::scoped_config))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn _hello() -> impl Responder {
    format!("Hello World")
}
