
use crate::schema::users;
use serde::{Deserialize, Serialize};
use diesel::{Queryable,Insertable};
use chrono;
use diesel::pg::PgConnection;
use diesel::dsl::{insert_into};
use diesel::RunQueryDsl;
use argon2::{self, Config};
use crate::settings::Settings;

//TODO use mail and username as primay key to prevent duplicate users.
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize,Deserialize,Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}

impl User{

    //TODO implement it
    pub fn login(conn: &PgConnection, param_email: &str, pw: &str) -> Result<Self,String>{
        // use crate::schema::users::dsl::*;
        // let user :User = users.filter(email.eq(param_email)).first(conn)?;
        // let pw_ok = user.verify_password_hash(pw);
        // if pw_ok==true {
        //     Ok(user)
        // }else{
        //     Err(String::from("User Not Found"))
        // }
        Err("User Not Found".to_string())
    }

    pub fn create(conn: &PgConnection, new_user: &mut NewUser) -> Result<Self, diesel::result::Error>{
        let config = Config::default();

        // TODO take salt from env
        new_user.password = argon2::hash_encoded(new_user.password.as_bytes(), b"myhashsalt", &config).unwrap();
        
        let res = insert_into(users::dsl::users).values(&*new_user).get_result::<User>(conn)?;
        Ok(res)
    }

    pub fn verify_password_hash(&self, password: &str)->bool{
        argon2::verify_encoded(&self.password, password.as_bytes()).unwrap()
    }
    //TODO implement it
    pub fn delete(&self, conn: &PgConnection) -> Result<Self,String>{
        Err("not found".to_string())
    }
}
