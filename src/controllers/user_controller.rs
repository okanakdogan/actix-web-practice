
use actix_web::{web};
use diesel::dsl::{insert_into};
use diesel::RunQueryDsl;

use crate::models::users::*;
use crate::postgres::Pool;
use crate::schema::users::dsl::*;

pub fn insert_new_user(db: web::Data<Pool>, new_user: NewUser) -> Result<User, diesel::result::Error>{
    let conn = db.get().unwrap();
    let res =insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}