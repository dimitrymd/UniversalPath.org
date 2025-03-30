#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;

mod api;
mod config;
mod db;
mod routes;
mod utils;

use crate::db::UniversalPathDb;

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    env_logger::init();

    // Determine if we're in debug mode
    let debug_mode = std::env::var("ROCKET_ENV")
        .unwrap_or_else(|_| "development".to_string())
        .to_lowercase() != "production";

    // Log the environment mode and configuration
    if debug_mode {
        println!("Starting in DEBUG mode");
        println!("Database: MySQL root user (no password)");
        println!("Site URL: localhost:8000");
        println!("Access the site at: http://localhost:8000");
    } else {
        println!("Starting in PRODUCTION mode");
        println!("Database: MySQL catman user");
        println!("Site URL: {}", config::CONFIG.site_url);
    }

    rocket::build()
        .attach(Template::fairing())
        .attach(UniversalPathDb::init())
        .mount("/", routes::web_routes())
        .mount("/api", routes::api_routes())
        .mount("/admin", routes::admin_routes())
        .mount("/static", FileServer::from("static"))
        .register("/", routes::catchers())
}