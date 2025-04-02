use rocket::{Route, get, post, put, delete};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::db::{UniversalPathDb, models::{Term, NewTerm, UpdateTerm}};
use crate::api::{ApiResponse, ApiError, ApiResult};
use crate::api::auth::{ApiKey, AdminUser};

// Note: All these routes will be mounted at /api/terms
// No rank is needed since they'll be mounted at a different base path than web routes

#[get("/terms")]
async fn get_terms(mut db: Connection<UniversalPathDb>, _key: ApiKey) -> Json<ApiResult<Vec<Term>>> {
    match Term::find_all(&mut db).await {
        Ok(terms) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: terms,
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to get terms: {}", e),
        })),
    }
}

#[get("/terms/<id>")]
async fn get_term(mut db: Connection<UniversalPathDb>, _key: ApiKey, id: u32) -> Json<ApiResult<Term>> {
    match Term::find_by_id(&mut db, id).await {
        Ok(Some(term)) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: term,
        })),
        Ok(None) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Term with id {} not found", id),
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to get term: {}", e),
        })),
    }
}

#[get("/terms/letter/<letter>")]
async fn get_terms_by_letter(mut db: Connection<UniversalPathDb>, _key: ApiKey, letter: String) -> Json<ApiResult<Vec<Term>>> {
    match Term::find_by_letter(&mut db, &letter).await {
        Ok(terms) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: terms,
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to get terms by letter: {}", e),
        })),
    }
}

#[get("/terms/letters")]
async fn get_letters(mut db: Connection<UniversalPathDb>, _key: ApiKey) -> Json<ApiResult<Vec<String>>> {
    match Term::get_all_letters(&mut db).await {
        Ok(letters) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: letters,
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to get letters: {}", e),
        })),
    }
}

#[get("/terms/search?<q>")]
async fn search_terms(mut db: Connection<UniversalPathDb>, _key: ApiKey, q: String) -> Json<ApiResult<Vec<Term>>> {
    match Term::search(&mut db, &q).await {
        Ok(terms) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: terms,
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to search terms: {}", e),
        })),
    }
}

#[post("/terms", format = "json", data = "<term>")]
async fn create_term(
    mut db: Connection<UniversalPathDb>, 
    _admin: AdminUser,
    term: Json<NewTerm>
) -> Json<ApiResult<u32>> {
    match Term::create(&mut db, term.0).await {
        Ok(id) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: id,
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to create term: {}", e),
        })),
    }
}

#[put("/terms/<id>", format = "json", data = "<term>")]
async fn update_term(
    mut db: Connection<UniversalPathDb>, 
    _admin: AdminUser,
    id: u32,
    term: Json<UpdateTerm>
) -> Json<ApiResult<bool>> {
    let mut term_update = term.0;
    term_update.id = id;

    match Term::update(&mut db, term_update).await {
        Ok(success) => {
            if success {
                Json(Ok(ApiResponse {
                    status: "success".to_string(),
                    data: true,
                }))
            } else {
                Json(Err(ApiError {
                    status: "error".to_string(),
                    message: format!("Term with id {} not found", id),
                }))
            }
        },
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to update term: {}", e),
        })),
    }
}

#[delete("/terms/<id>")]
async fn delete_term(
    mut db: Connection<UniversalPathDb>, 
    _admin: AdminUser,
    id: u32
) -> Json<ApiResult<bool>> {
    match Term::delete(&mut db, id).await {
        Ok(success) => {
            if success {
                Json(Ok(ApiResponse {
                    status: "success".to_string(),
                    data: true,
                }))
            } else {
                Json(Err(ApiError {
                    status: "error".to_string(),
                    message: format!("Term with id {} not found", id),
                }))
            }
        },
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to delete term: {}", e),
        })),
    }
}

pub fn routes() -> Vec<Route> {
    routes![
        get_terms, 
        get_term, 
        get_terms_by_letter,
        get_letters,
        search_terms,
        create_term,
        update_term,
        delete_term
    ]
}