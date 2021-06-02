
#[macro_use]
extern crate diesel;

use actix_web::{get,web, App, HttpResponse, HttpServer, Responder};
mod settings;
use settings::Settings;
mod postgres;

mod schema;
mod models;
mod handlers;
mod routes;
mod controllers;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world! test")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let settings = Settings::load();

    println!("{:?}", settings.db);
    let pool = postgres::get_pool(&settings.db.url);    

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::scope("/user").configure(routes::user_config))
            .service(hello)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}