use ::config::Config;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub pg_conn_string: String,
    pub secret: String,
    pub server_addr: String,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv().unwrap();
        let config = Config::builder()
            .add_source(::config::Environment::default())
            .build()
            .unwrap();
        config.try_deserialize().unwrap()
    }
}
