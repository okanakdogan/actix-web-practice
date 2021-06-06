//use std::env;

use serde::{Deserialize};
use std::env;
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
pub struct Database{
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings{
    pub db: Database,
    pub hash_salt: String,
}

impl Settings{
    pub fn load() -> Settings{
        dotenv().ok();
        Settings{
            db: Database{
                url:env::var("DATABASE_URL").unwrap_or("".to_string()),
            },
            hash_salt: env::var("HASH_SALT").unwrap_or("myhashsalt".to_string())
        }
    }
}
