use rocket::{Route, get, post, put, delete};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::db::{UniversalPathDb, models::{Article, ArticleWithAuthor, NewArticle, UpdateArticle}};
use crate::api::{ApiResponse, ApiError, ApiResult};
use crate::api::auth::{ApiKey, AdminUser};

#[get("/articles", rank = 1)]
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

#[get("/articles/<id>", rank = 1)]
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

#[get("/articles/category/<id>", rank = 1)]
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

#[get("/articles/search?<q>", rank = 1)]
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

#[post("/articles", format = "json", data = "<article>", rank = 1)]
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

#[put("/articles/<id>", format = "json", data = "<article>", rank = 1)]
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

#[delete("/articles/<id>", rank = 1)]
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

pub fn routes() -> Vec<Route> {
    routes![
        get_articles, 
        get_article, 
        get_articles_by_category, 
        search_articles,
        create_article,
        update_article,
        delete_article
    ]
}