use rocket::{Route, get};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::db::{UniversalPathDb, models::{Category, Article}};

#[get("/")]
async fn index(mut db: Connection<UniversalPathDb>) -> Template {
    // Get root categories
    let categories = match Category::find_root_categories(&mut db).await {
        Ok(cats) => cats,
        Err(_) => vec![],
    };

    // Get recent articles
    let recent_articles = match Article::find_recent(&mut db, 10).await {
        Ok(articles) => articles,
        Err(_) => vec![],
    };

    Template::render("index", context! {
        title: "Универсальный Путь",
        categories: &categories,
        recent_articles: &recent_articles,
    })
}

pub fn routes() -> Vec<Route> {
    routes![index]
}