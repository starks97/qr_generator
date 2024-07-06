use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub client_origin: String,
}

impl Config {
    pub fn init() -> Config {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
        let client_origin =
            env::var("CLIENT_ORIGIN").expect("CLIENT_ORIGIN must be set in .env file");

        Config {
            database_url,
            client_origin,
        }
    }
}
