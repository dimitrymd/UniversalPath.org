pub mod articles;
pub mod categories;
pub mod terms;
pub mod auth;

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