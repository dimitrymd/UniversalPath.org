use std::env;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = {
        Config::new()
    };
}

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub site_url: String,
    pub debug_mode: bool,
}

impl Config {
    fn new() -> Self {
        dotenv::dotenv().ok();
        
        // Check if we're in debug mode
        let debug_mode = env::var("ROCKET_ENV")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase() != "production";
            
        // For debug mode, use root without password
        let database_url = if debug_mode {
            env::var("DATABASE_URL_DEBUG")
                .unwrap_or_else(|_| "mysql://root@localhost:3306/catman".to_string())
        } else {
            env::var("DATABASE_URL")
                .unwrap_or_else(|_| "mysql://catman:xB6udTcQaR2afdDX@localhost:3306/catman".to_string())
        };
        
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default_secret_key_for_jwt_tokens_should_be_changed".to_string());
        
        let site_url = env::var("SITE_URL")
            .unwrap_or_else(|_| "universalpath.org".to_string());
        
        Config {
            database_url,
            jwt_secret,
            site_url,
            debug_mode,
        }
    }
}