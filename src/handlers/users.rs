
use actix_web::{web, HttpResponse, Error, Responder};
use serde::{Serialize,Deserialize};

use crate::models::users::*;
use crate::postgres::Pool;

// Notes: Using to different struct for getting input from request & inserting is weirdo
#[derive(Serialize,Deserialize)]
pub struct InputUser{
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn signup(db: web::Data<Pool>, data: web::Json<InputUser>) -> Result<HttpResponse, Error> {
    //NOTE: you should use this enpoint with ssl to protect password

    //TODO add validators for inputs (username,password,email etc.)
    
    let mut new_user = NewUser{
        username: data.username.clone(),
        email: data.email.clone(),
        password: data.password.clone(),
        created_at: chrono::Local::now().naive_local(),
    };
    let conn = db.get().unwrap();
    let user = User::create(&conn, &mut new_user).unwrap();
    Ok(HttpResponse::Ok().body(serde_json::to_string(&user).unwrap()))
}

pub async fn login() -> impl Responder {
    HttpResponse::Ok().body("User login")
}
