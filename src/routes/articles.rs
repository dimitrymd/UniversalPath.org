use rocket::{Route, get};
use rocket::response::Redirect;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::db::{UniversalPathDb, models::{Article}};
use crate::utils::breadcrumbs::generate_breadcrumbs;
use crate::utils::helpers::format_article_content;

#[get("/article/<id>")]
async fn view_article(mut db: Connection<UniversalPathDb>, id: u32) -> Result<Template, Redirect> {
    // Get the article
    let article = match Article::find_by_id_with_author(&mut db, id).await {
        Ok(Some(article)) => article,
        Ok(None) => return Err(Redirect::to("/")),
        Err(_) => return Err(Redirect::to("/")),
    };

    // Generate breadcrumbs for this article
    let breadcrumbs = match generate_breadcrumbs(&mut db, article.article.category_id, Some(id)).await {
        Ok(crumbs) => crumbs,
        Err(_) => vec![],
    };

    // Format article content
    let formatted_content = match &article.article.txtfield {
        Some(content) => format_article_content(content),
        None => String::new(),
    };

    Ok(Template::render("article", context! {
        title: article.article.title.clone().unwrap_or_default(),
        article: &article,
        breadcrumbs: &breadcrumbs,
        content: formatted_content,
    }))
}

#[get("/search?<q>")]
async fn search(mut db: Connection<UniversalPathDb>, q: Option<String>) -> Template {
    let search_query = q.unwrap_or_default();
    
    if search_query.is_empty() {
        return Template::render("search", context! {
            title: "Поиск",
            query: &search_query,
            articles: Vec::<Article>::new(),
            has_results: false,
        });
    }

    // Search for articles
    let articles = match Article::search(&mut db, &search_query, Some(50)).await {
        Ok(articles) => articles,
        Err(_) => vec![],
    };

    Template::render("search", context! {
        title: format!("Поиск: {}", &search_query),
        query: &search_query,
        articles: &articles,
        has_results: !articles.is_empty(),
    })
}

pub fn routes() -> Vec<Route> {
    routes![view_article, search]
}