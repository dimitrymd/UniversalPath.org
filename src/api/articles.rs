use rocket::{Route, get, post, put, delete};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::db::{UniversalPathDb, models::{Article, ArticleWithAuthor, NewArticle, UpdateArticle}};
use crate::api::{ApiResponse, ApiError, ApiResult};
use crate::api::auth::{ApiKey, AdminUser};

// GET routes
#[get("/articles")]
async fn get_articles(mut db: Connection<UniversalPathDb>, _key: ApiKey) -> Json<ApiResult<Vec<ArticleWithAuthor>>> {
    match Article::find_recent(&mut db, 50).await {
        Ok(articles) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: articles,
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to get articles: {}", e),
        })),
    }
}

#[get("/articles/<id>")]
async fn get_article(mut db: Connection<UniversalPathDb>, _key: ApiKey, id: u32) -> Json<ApiResult<ArticleWithAuthor>> {
    match Article::find_by_id_with_author(&mut db, id).await {
        Ok(Some(article)) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: article,
        })),
        Ok(None) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Article with id {} not found", id),
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to get article: {}", e),
        })),
    }
}

#[get("/articles/category/<id>")]
async fn get_articles_by_category(mut db: Connection<UniversalPathDb>, _key: ApiKey, id: u32) -> Json<ApiResult<Vec<ArticleWithAuthor>>> {
    match Article::find_by_category(&mut db, id, None, None).await {
        Ok(articles) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: articles,
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to get articles: {}", e),
        })),
    }
}

#[get("/articles/search?<q>")]
async fn search_articles(mut db: Connection<UniversalPathDb>, _key: ApiKey, q: String) -> Json<ApiResult<Vec<ArticleWithAuthor>>> {
    match Article::search(&mut db, &q, None).await {
        Ok(articles) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: articles,
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to search articles: {}", e),
        })),
    }
}

// POST routes
#[post("/articles", format = "json", data = "<article>")]
async fn create_article(
    mut db: Connection<UniversalPathDb>, 
    _admin: AdminUser,
    article: Json<NewArticle>
) -> Json<ApiResult<u32>> {
    match Article::create(&mut db, article.0, _admin.user_id).await {
        Ok(id) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: id,
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to create article: {}", e),
        })),
    }
}

// PUT routes
#[put("/articles/<id>", format = "json", data = "<article>")]
async fn update_article(
    mut db: Connection<UniversalPathDb>, 
    admin: AdminUser,
    id: u32,
    article: Json<UpdateArticle>
) -> Json<ApiResult<bool>> {
    let mut article_update = article.0;
    article_update.id = id;
    article_update.lasteditedby_userid = admin.user_id;

    match Article::update(&mut db, article_update).await {
        Ok(success) => {
            if success {
                Json(Ok(ApiResponse {
                    status: "success".to_string(),
                    data: true,
                }))
            } else {
                Json(Err(ApiError {
                    status: "error".to_string(),
                    message: format!("Article with id {} not found", id),
                }))
            }
        },
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to update article: {}", e),
        })),
    }
}

// DELETE routes
#[delete("/articles/<id>")]
async fn delete_article(
    mut db: Connection<UniversalPathDb>, 
    _admin: AdminUser,
    id: u32
) -> Json<ApiResult<bool>> {
    match Article::delete(&mut db, id).await {
        Ok(success) => {
            if success {
                Json(Ok(ApiResponse {
                    status: "success".to_string(),
                    data: true,
                }))
            } else {
                Json(Err(ApiError {
                    status: "error".to_string(),
                    message: format!("Article with id {} not found", id),
                }))
            }
        },
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to delete article: {}", e),
        })),
    }
}

// Export routes separated by HTTP method
pub fn get_routes() -> Vec<Route> {
    routes![
        get_articles, 
        get_article, 
        get_articles_by_category, 
        search_articles
    ]
}

pub fn post_routes() -> Vec<Route> {
    routes![
        create_article
    ]
}

pub fn put_routes() -> Vec<Route> {
    routes![
        update_article
    ]
}

pub fn delete_routes() -> Vec<Route> {
    routes![
        delete_article
    ]
}

// Original function - now just combines all the separate routes
pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(get_routes());
    routes.extend(post_routes());
    routes.extend(put_routes());
    routes.extend(delete_routes());
    routes
}