use rocket::{Route, get, post, put, delete};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::db::{UniversalPathDb, models::{Category, CategoryWithCounts, CategoryTreeItem, NewCategory, UpdateCategory}};
use crate::api::{ApiResponse, ApiError, ApiResult};
use crate::api::auth::{ApiKey, AdminUser};

// GET routes
#[get("/categories")]
async fn get_categories(mut db: Connection<UniversalPathDb>, _key: ApiKey) -> Json<ApiResult<Vec<CategoryWithCounts>>> {
    match Category::find_root_categories(&mut db).await {
        Ok(categories) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: categories,
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to get categories: {}", e),
        })),
    }
}

#[get("/categories/<id>")]
async fn get_category(mut db: Connection<UniversalPathDb>, _key: ApiKey, id: u32) -> Json<ApiResult<CategoryWithCounts>> {
    match Category::find_by_id_with_counts(&mut db, id).await {
        Ok(Some(category)) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: category,
        })),
        Ok(None) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Category with id {} not found", id),
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to get category: {}", e),
        })),
    }
}

#[get("/categories/<id>/subcategories")]
async fn get_subcategories(mut db: Connection<UniversalPathDb>, _key: ApiKey, id: u32) -> Json<ApiResult<Vec<CategoryWithCounts>>> {
    match Category::find_subcategories(&mut db, id).await {
        Ok(categories) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: categories,
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to get subcategories: {}", e),
        })),
    }
}

#[get("/categories/tree")]
async fn get_category_tree(mut db: Connection<UniversalPathDb>, _key: ApiKey) -> Json<ApiResult<Vec<CategoryTreeItem>>> {
    match Category::build_tree(&mut db, None).await {
        Ok(tree) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: tree,
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to get category tree: {}", e),
        })),
    }
}

#[get("/categories/<id>/path")]
async fn get_category_path(mut db: Connection<UniversalPathDb>, _key: ApiKey, id: u32) -> Json<ApiResult<Vec<Category>>> {
    match Category::build_category_path(&mut db, id).await {
        Ok(path) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: path,
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to get category path: {}", e),
        })),
    }
}

// POST routes
#[post("/categories", format = "json", data = "<category>")]
async fn create_category(
    mut db: Connection<UniversalPathDb>, 
    _admin: AdminUser,
    category: Json<NewCategory>
) -> Json<ApiResult<u32>> {
    match Category::create(&mut db, category.0).await {
        Ok(id) => Json(Ok(ApiResponse {
            status: "success".to_string(),
            data: id,
        })),
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to create category: {}", e),
        })),
    }
}

// PUT routes
#[put("/categories/<id>", format = "json", data = "<category>")]
async fn update_category(
    mut db: Connection<UniversalPathDb>, 
    _admin: AdminUser,
    id: u32,
    category: Json<UpdateCategory>
) -> Json<ApiResult<bool>> {
    let mut category_update = category.0;
    category_update.id = id;

    match Category::update(&mut db, category_update).await {
        Ok(success) => {
            if success {
                Json(Ok(ApiResponse {
                    status: "success".to_string(),
                    data: true,
                }))
            } else {
                Json(Err(ApiError {
                    status: "error".to_string(),
                    message: format!("Category with id {} not found", id),
                }))
            }
        },
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to update category: {}", e),
        })),
    }
}

// DELETE routes
#[delete("/categories/<id>")]
async fn delete_category(
    mut db: Connection<UniversalPathDb>, 
    _admin: AdminUser,
    id: u32
) -> Json<ApiResult<bool>> {
    match Category::delete(&mut db, id).await {
        Ok(success) => {
            if success {
                Json(Ok(ApiResponse {
                    status: "success".to_string(),
                    data: true,
                }))
            } else {
                Json(Err(ApiError {
                    status: "error".to_string(),
                    message: format!("Category with id {} not found or has subcategories/articles", id),
                }))
            }
        },
        Err(e) => Json(Err(ApiError {
            status: "error".to_string(),
            message: format!("Failed to delete category: {}", e),
        })),
    }
}

// Export routes separated by HTTP method
pub fn get_routes() -> Vec<Route> {
    routes![
        get_categories, 
        get_category, 
        get_subcategories,
        get_category_tree,
        get_category_path
    ]
}

pub fn post_routes() -> Vec<Route> {
    routes![
        create_category
    ]
}

pub fn put_routes() -> Vec<Route> {
    routes![
        update_category
    ]
}

pub fn delete_routes() -> Vec<Route> {
    routes![
        delete_category
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