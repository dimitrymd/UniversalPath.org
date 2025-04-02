pub mod articles;
pub mod categories;
pub mod terms;
pub mod auth;

use rocket::Route;
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub status: String,
    pub message: String,
}

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

// Original function that combines all routes
pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(articles::routes());
    routes.extend(categories::routes());
    routes.extend(terms::routes());
    routes.extend(auth::routes());
    routes
}

// Separate routes by HTTP method to avoid collisions
pub fn get_routes() -> Vec<Route> {
    let mut routes = Vec::new();
    // Only include GET routes
    routes.extend(articles::routes().into_iter()
        .filter(|r| r.method == rocket::http::Method::Get));
    routes.extend(categories::routes().into_iter()
        .filter(|r| r.method == rocket::http::Method::Get));
    routes.extend(terms::routes().into_iter()
        .filter(|r| r.method == rocket::http::Method::Get));
    routes.extend(auth::routes().into_iter()
        .filter(|r| r.method == rocket::http::Method::Get));
    routes
}

pub fn post_routes() -> Vec<Route> {
    let mut routes = Vec::new();
    // Only include POST routes
    routes.extend(articles::routes().into_iter()
        .filter(|r| r.method == rocket::http::Method::Post));
    routes.extend(categories::routes().into_iter()
        .filter(|r| r.method == rocket::http::Method::Post));
    routes.extend(terms::routes().into_iter()
        .filter(|r| r.method == rocket::http::Method::Post));
    routes.extend(auth::routes().into_iter()
        .filter(|r| r.method == rocket::http::Method::Post));
    routes
}

pub fn put_routes() -> Vec<Route> {
    let mut routes = Vec::new();
    // Only include PUT routes
    routes.extend(articles::routes().into_iter()
        .filter(|r| r.method == rocket::http::Method::Put));
    routes.extend(categories::routes().into_iter()
        .filter(|r| r.method == rocket::http::Method::Put));
    routes.extend(terms::routes().into_iter()
        .filter(|r| r.method == rocket::http::Method::Put));
    routes.extend(auth::routes().into_iter()
        .filter(|r| r.method == rocket::http::Method::Put));
    routes
}

pub fn delete_routes() -> Vec<Route> {
    let mut routes = Vec::new();
    // Only include DELETE routes
    routes.extend(articles::routes().into_iter()
        .filter(|r| r.method == rocket::http::Method::Delete));
    routes.extend(categories::routes().into_iter()
        .filter(|r| r.method == rocket::http::Method::Delete));
    routes.extend(terms::routes().into_iter()
        .filter(|r| r.method == rocket::http::Method::Delete));
    routes.extend(auth::routes().into_iter()
        .filter(|r| r.method == rocket::http::Method::Delete));
    routes
}