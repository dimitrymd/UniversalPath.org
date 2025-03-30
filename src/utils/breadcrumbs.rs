use rocket::serde::Serialize;
use rocket_db_pools::Connection;
use crate::db::{UniversalPathDb, models::{Category, Article}};
use anyhow::Result;

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Breadcrumb {
    pub title: String,
    pub url: String,
    pub is_last: bool,
}

pub async fn generate_breadcrumbs(
    db: &mut Connection<UniversalPathDb>,
    category_id: Option<u32>,
    article_id: Option<u32>,
) -> Result<Vec<Breadcrumb>> {
    let mut breadcrumbs = Vec::new();
    
    // Always add home
    breadcrumbs.push(Breadcrumb {
        title: "Главная".to_string(),
        url: "/".to_string(),
        is_last: false,
    });

    // If we have a category, add the category path
    if let Some(category_id) = category_id {
        let category_path = Category::build_category_path(db, category_id).await?;
        
        for (i, category) in category_path.iter().enumerate() {
            let is_last = i == category_path.len() - 1 && article_id.is_none();
            
            breadcrumbs.push(Breadcrumb {
                title: category.title.clone().unwrap_or_else(|| category.name.clone()),
                url: format!("/category/{}", category.id),
                is_last,
            });
        }
    }

    // If we have an article, add the article
    if let Some(article_id) = article_id {
        if let Ok(Some(article)) = Article::find_by_id(db, article_id).await {
            if let Some(title) = &article.title {
                breadcrumbs.push(Breadcrumb {
                    title: title.clone(),
                    url: format!("/article/{}", article.id),
                    is_last: true,
                });
            }
        }
    }

    // Mark the last one as last
    if let Some(last) = breadcrumbs.last_mut() {
        last.is_last = true;
    }

    Ok(breadcrumbs)
}