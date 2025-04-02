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

    // Mount each module and HTTP method separately to avoid collisions
    rocket::build()
        .attach(Template::fairing())
        .attach(UniversalPathDb::init())
        // Web routes
        .mount("/", routes::web_routes())
        
        // API routes separated by module and HTTP method
        // Articles module
        .mount("/api/articles", api::articles::get_routes())
        .mount("/api/articles", api::articles::post_routes())
        .mount("/api/articles", api::articles::put_routes())
        .mount("/api/articles", api::articles::delete_routes())
        
        // Categories module
        .mount("/api/categories", api::categories::get_routes())
        .mount("/api/categories", api::categories::post_routes())
        .mount("/api/categories", api::categories::put_routes())
        .mount("/api/categories", api::categories::delete_routes())
        
        // Terms module (the original approach since it works)
        .mount("/api/terms", api::terms::routes())
        
        // Auth module (the original approach since it works)
        .mount("/api/auth", api::auth::routes())
        
        // Admin routes
        .mount("/admin", routes::admin_routes())
        
        // Static files
        .mount("/static", FileServer::from("static"))
        
        // Error catchers
        .register("/", routes::catchers())
}