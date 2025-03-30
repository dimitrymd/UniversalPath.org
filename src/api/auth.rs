use rocket::{Route, post, get, http::Status};
use rocket::serde::json::Json;
use rocket::request::{self, Request, FromRequest, Outcome};
use rocket_db_pools::Connection;
use jsonwebtoken::{decode, encode, Header, Validation, EncodingKey, DecodingKey};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
use crate::db::{UniversalPathDb, models::{User, LoginUser, UserToken}};
use crate::api::{ApiResponse, ApiError, ApiResult};

pub struct ApiKey {
    pub key: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        
        match keys.len() {
            0 => Outcome::Error((Status::Unauthorized, ApiError {
                status: "error".to_string(),
                message: "API key required".to_string(),
            })),
            1 => {
                // In a real app, you would validate this against a database
                // For now, we'll accept any key that's present
                let key = keys[0].to_string();
                Outcome::Success(ApiKey { key })
            }
            _ => Outcome::Error((Status::BadRequest, ApiError {
                status: "error".to_string(),
                message: "Multiple API keys found".to_string(),
            })),
        }
    }
}

pub struct AdminUser {
    pub user_id: u32,
    pub username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ApiError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth_headers: Vec<_> = request.headers().get("Authorization").collect();
        
        if auth_headers.is_empty() {
            return Outcome::Error((Status::Unauthorized, ApiError {
                status: "error".to_string(),
                message: "Authorization header required".to_string(),
            }));
        }

        let auth_header = auth_headers[0];
        if !auth_header.starts_with("Bearer ") {
            return Outcome::Error((Status::Unauthorized, ApiError {
                status: "error".to_string(),
                message: "Invalid authorization format, must be Bearer token".to_string(),
            }));
        }

        let token = auth_header.trim_start_matches("Bearer ").trim();
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key".to_string());

        match decode::<UserToken>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(token_data) => {
                let user = token_data.claims;
                
                if !user.is_admin {
                    return Outcome::Error((Status::Forbidden, ApiError {
                        status: "error".to_string(),
                        message: "Admin access required".to_string(),
                    }));
                }
                
                Outcome::Success(AdminUser {
                    user_id: user.id,
                    username: user.username,
                })
            }
            Err(_) => Outcome::Error((Status::Unauthorized, ApiError {
                status: "error".to_string(),
                message: "Invalid token".to_string(),
            })),
        }
    }
}

#[post("/login", format = "json", data = "<login>")]
async fn login(mut db: Connection<UniversalPathDb>, login: Json<LoginUser>) -> Json<ApiResult<String>> {
    match User::login(&mut db, &login.0).await {
        Ok(Some(user)) => {
            // Generate a JWT token
            let expiration = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 24 * 3600; // Token valid for 24 hours

            let claims = UserToken {
                id: user.id,
                username: user.username,
                is_admin: user.is_admin,
                exp: expiration,
            };

            let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key".to_string());
            
            match encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_bytes()),
            ) {
                Ok(token) => Json(Ok(ApiResponse {
                    status: "success".to_string(),
                    data: token,
                })),
                Err(_) => Json(Err(ApiError {
                    status: "error".to_string(),
                    message: "Failed to generate token".to_string(),
                })),
            }
        }
        Ok(None) => Json(Err(ApiError {
            status: "error".to_string(),
            message: "Invalid username or password".to_string(),
        })),
        Err(_) => Json(Err(ApiError {
            status: "error".to_string(),
            message: "Login failed".to_string(),
        })),
    }
}

#[get("/verify", rank = 1)]
async fn verify_token(_admin: AdminUser) -> Json<ApiResult<bool>> {
    Json(Ok(ApiResponse {
        status: "success".to_string(),
        data: true,
    }))
}

#[get("/verify", rank = 2)]
async fn verify_token_fail() -> Json<ApiResult<bool>> {
    Json(Err(ApiError {
        status: "error".to_string(),
        message: "Invalid or expired token".to_string(),
    }))
}

pub fn routes() -> Vec<Route> {
    routes![login, verify_token, verify_token_fail]
}