
use crate::handlers::users;
use actix_web::{web};

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg
    .service(web::resource("/signup").route(web::post().to(users::signup)))
    .service(web::resource("/login").route(web::get().to(users::login)));
}