
// use crate::schema::*;
// use crate::schema::users::dsl::*;
use crate::schema::users;
use serde::{Deserialize, Serialize};
use diesel::{Queryable,Insertable};
use chrono;
use diesel::pg::PgConnection;
use diesel::dsl::{insert_into};
use diesel::RunQueryDsl;
use argon2::{self, Config};



#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize,Deserialize,Clone)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

impl User{

    // pub fn find_all(conn: &PgConnection) -> Result<Vec<Self>,String> {
    //     Ok(Vec::new())
    // }

    pub fn find(conn: &PgConnection, id: &str) -> Result<Self,String>{
        Err("not found".to_string())
    }

    pub fn create(conn: &PgConnection, new_user: &mut NewUser) -> Result<Self, diesel::result::Error>{
        let config = Config::default();
        // TODO figure out this hash thing with borrow check issue
        // let hash = argon2::hash_encoded(new_user.password.as_bytes(), b"salt", &config).unwrap();

        // new_user.password = &hash;
        
        let res = insert_into(users::dsl::users).values(&*new_user).get_result(conn)?;
        Ok(res)
    }

    pub fn verify_password_hash(&self, password: &str)->bool{
        argon2::verify_encoded(&self.password, password.as_bytes()).unwrap()
    }

    pub fn delete(&self, conn: &PgConnection) -> Result<Self,String>{
        Err("not found".to_string())
    }
}
