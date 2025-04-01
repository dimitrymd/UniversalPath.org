use rocket::{Route, get};
use rocket::response::Redirect;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::db::{UniversalPathDb, models::{Category, Article}};
use crate::utils::breadcrumbs::generate_breadcrumbs;

#[get("/category/<id>")]
async fn view_category(mut db: Connection<UniversalPathDb>, id: u32) -> Result<Template, Redirect> {
    // Get the category
    let category = match Category::find_by_id_with_counts(&mut db, id).await {
        Ok(Some(category)) => category,
        Ok(None) => return Err(Redirect::to("/")),
        Err(_) => return Err(Redirect::to("/")),
    };

    // Get subcategories
    let subcategories = match Category::find_subcategories(&mut db, id).await {
        Ok(subcats) => subcats,
        Err(_) => vec![],
    };

    // Get articles in this category
    let articles = match Article::find_by_category(&mut db, id, Some(100), Some(0)).await {
        Ok(arts) => arts,
        Err(_) => vec![],
    };

    // Generate breadcrumbs
    let breadcrumbs = match generate_breadcrumbs(&mut db, Some(id), None).await {
        Ok(crumbs) => crumbs,
        Err(_) => vec![],
    };

    Ok(Template::render("category", context! {
        title: &category.category.title,
        category: &category,
        subcategories: &subcategories,
        articles: &articles,
        breadcrumbs: &breadcrumbs,
    }))
}

pub fn routes() -> Vec<Route> {
    routes![view_category]
}