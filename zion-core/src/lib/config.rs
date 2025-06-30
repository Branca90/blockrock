use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub api_port: u16,
    pub grpc_port: u16,
    pub static_path: String,
    pub trongrid_api_key: String,
}

impl Config {
    pub fn load() -> Result<Self, env::VarError> {
        dotenv().ok();
        Ok(Self {
            api_port: env::var("API_PORT")?.parse().unwrap_or(8000),
            grpc_port: env::var("GRPC_PORT")?.parse().unwrap_or(50051),
            static_path: env::var("STATIC_PATH").unwrap_or("static".to_string()),
            trongrid_api_key: env::var("TRONGRID_API_KEY")?,
        })
    }
}
