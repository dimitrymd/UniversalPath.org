use rocket::Route;
use rocket::http::Status;
use rocket::Request;
use rocket::response::status::Custom;
use rocket::serde::json::{Json, Value};
use rocket::Catcher;
use serde_json::json;

mod index;
mod articles;
mod categories;
mod terms;
mod admin;

// Web routes for the frontend
pub fn web_routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(index::routes());
    routes.extend(articles::routes());
    routes.extend(categories::routes());
    routes.extend(terms::routes());
    routes
}

// API routes for programmatic access
pub fn api_routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(crate::api::articles::routes());
    routes.extend(crate::api::categories::routes());
    routes.extend(crate::api::terms::routes());
    routes.extend(crate::api::auth::routes());
    routes
}

// Admin panel routes
pub fn admin_routes() -> Vec<Route> {
    admin::routes()
}

pub fn catchers() -> Vec<Catcher> {
    catchers![not_found, server_error, unprocessable_entity]
}

#[catch(404)]
fn not_found(req: &Request) -> Custom<Json<Value>> {
    if req.uri().path().starts_with("/api") {
        // API routes return JSON
        Custom(
            Status::NotFound,
            Json(json!({
                "status": "error",
                "message": "Resource not found"
            })),
        )
    } else {
        // Web routes lead to template rendering
        // This will be handled by the web request context
        Custom(
            Status::NotFound,
            Json(json!({
                "status": "error",
                "message": "Page not found"
            })),
        )
    }
}

#[catch(500)]
fn server_error(req: &Request) -> Custom<Json<Value>> {
    if req.uri().path().starts_with("/api") {
        // API routes return JSON
        Custom(
            Status::InternalServerError,
            Json(json!({
                "status": "error",
                "message": "An internal server error occurred"
            })),
        )
    } else {
        // Web routes lead to template rendering
        // This will be handled by the web request context
        Custom(
            Status::InternalServerError,
            Json(json!({
                "status": "error",
                "message": "An internal server error occurred"
            })),
        )
    }
}

#[catch(422)]
fn unprocessable_entity(req: &Request) -> Custom<Json<Value>> {
    if req.uri().path().starts_with("/api") {
        // API routes return JSON
        Custom(
            Status::UnprocessableEntity,
            Json(json!({
                "status": "error",
                "message": "Unprocessable entity"
            })),
        )
    } else {
        // Web routes lead to template rendering
        // This will be handled by the web request context
        Custom(
            Status::UnprocessableEntity,
            Json(json!({
                "status": "error",
                "message": "Unprocessable entity"
            })),
        )
    }
}