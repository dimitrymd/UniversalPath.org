// src/debug_build.rs
use rocket::fs::FileServer;
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;
use crate::db::UniversalPathDb;

// Function to build rocket with only web routes
pub fn build_web_only() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Template::fairing())
        .attach(UniversalPathDb::init())
        .mount("/", crate::routes::web_routes())
        .mount("/static", FileServer::from("static"))
        .register("/", crate::routes::catchers())
}

// Function to build rocket with only API routes (read-only version)
pub fn build_api_readonly() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Template::fairing())
        .attach(UniversalPathDb::init())
        .mount("/api", crate::api::routes_readonly())
        .mount("/static", FileServer::from("static"))
        .register("/", crate::routes::catchers())
}

// Function to build rocket with only API routes
pub fn build_api_only() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Template::fairing())
        .attach(UniversalPathDb::init())
        .mount("/api", crate::api::routes())
        .mount("/static", FileServer::from("static"))
        .register("/", crate::routes::catchers())
}

// Function to build rocket with only admin routes
pub fn build_admin_only() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Template::fairing())
        .attach(UniversalPathDb::init())
        .mount("/admin", crate::routes::admin_routes())
        .mount("/static", FileServer::from("static"))
        .register("/", crate::routes::catchers())
}

// Function to build rocket with web and API only
pub fn build_web_and_api() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Template::fairing())
        .attach(UniversalPathDb::init())
        .mount("/", crate::routes::web_routes())
        .mount("/api", crate::api::routes())
        .mount("/static", FileServer::from("static"))
        .register("/", crate::routes::catchers())
}

// Function to build rocket with web and admin only
pub fn build_web_and_admin() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Template::fairing())
        .attach(UniversalPathDb::init())
        .mount("/", crate::routes::web_routes())
        .mount("/admin", crate::routes::admin_routes())
        .mount("/static", FileServer::from("static"))
        .register("/", crate::routes::catchers())
}

// Function to build rocket with API and admin only
pub fn build_api_and_admin() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Template::fairing())
        .attach(UniversalPathDb::init())
        .mount("/api", crate::api::routes())
        .mount("/admin", crate::routes::admin_routes())
        .mount("/static", FileServer::from("static"))
        .register("/", crate::routes::catchers())
}

// Function to build rocket with only API auth routes
pub fn build_auth_only() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Template::fairing())
        .attach(UniversalPathDb::init())
        .mount("/api", crate::api::auth::routes())
        .mount("/static", FileServer::from("static"))
        .register("/", crate::routes::catchers())
}

// Test only articles routes
pub fn build_articles_only() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Template::fairing())
        .attach(UniversalPathDb::init())
        .mount("/api", crate::api::articles::routes())
        .mount("/static", FileServer::from("static"))
        .register("/", crate::routes::catchers())
}

// Test only categories routes
pub fn build_categories_only() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Template::fairing())
        .attach(UniversalPathDb::init())
        .mount("/api", crate::api::categories::routes())
        .mount("/static", FileServer::from("static"))
        .register("/", crate::routes::catchers())
}

// Test only terms routes
pub fn build_terms_only() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Template::fairing())
        .attach(UniversalPathDb::init())
        .mount("/api", crate::api::terms::routes())
        .mount("/static", FileServer::from("static"))
        .register("/", crate::routes::catchers())
}