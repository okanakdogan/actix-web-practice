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
}

impl Settings{
    pub fn load() -> Settings{
        dotenv().ok();
        Settings{
            db: Database{
                url:env::var("DATABASE_URL").unwrap_or("".to_string()),
            }
        }
    }
}
