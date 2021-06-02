
use actix_web::{web, HttpResponse, Error, Responder};
use serde::{Serialize,Deserialize};

use crate::models::users::*;
//use crate::controllers::user_controller as uc;
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
    //TODO add password hasher
    
    let mut new_user = NewUser{
        username: &data.username,
        email: &data.email,
        password: &data.password,
        created_at: chrono::Local::now().naive_local(),
    };
    let conn = db.get().unwrap();
    let user = User::create(&conn, &mut new_user).unwrap();
    Ok(HttpResponse::Ok().body("OK"))
    // Ok(web::block(move|| uc::insert_new_user(db, new_user))
    //     .await
    //     .map(|user| HttpResponse::Created().json(user))
    //     .map_err(|_| HttpResponse::InternalServerError())?,
    // )

    // Ok(HttpResponse::Ok().body(format!("{}",new_user.email)))
}

pub async fn login() -> impl Responder {
    HttpResponse::Ok().body("User login")
}
