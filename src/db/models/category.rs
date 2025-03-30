use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::{sqlx, Connection};
use crate::db::UniversalPathDb;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub title: Option<String>,
    pub parent_id: Option<u32>,
    pub root_id: Option<u32>,
    pub level: i32,
    pub url: Option<String>,
    pub preview: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub created: Option<chrono::NaiveDateTime>,
    pub available: Option<bool>,
    pub text: Option<String>,
    pub redirect: Option<String>,
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CategoryWithCounts {
    #[serde(flatten)]
    pub category: Category,
    pub article_count: i64,
    pub subcategory_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CategoryTreeItem {
    #[serde(flatten)]
    pub category: CategoryWithCounts,
    pub children: Vec<CategoryTreeItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewCategory {
    pub name: String,
    pub title: Option<String>,
    pub parent_id: Option<u32>,
    pub url: Option<String>,
    pub preview: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub available: Option<bool>,
    pub text: Option<String>,
    pub redirect: Option<String>,
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateCategory {
    pub id: u32,
    pub name: Option<String>,
    pub title: Option<String>,
    pub parent_id: Option<u32>,
    pub url: Option<String>,
    pub preview: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub available: Option<bool>,
    pub text: Option<String>,
    pub redirect: Option<String>,
    pub priority: Option<i32>,
}

impl Category {
    pub async fn find_by_id(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<Option<Category>> {
        let category = sqlx::query_as!(
            Category,
            r#"
            SELECT 
                id, name, title, parent_id, root_id, level, url, preview, 
                description, keywords, created, available, text, redirect, priority
            FROM categories 
            WHERE id = ? AND (available = 1 OR available IS NULL)
            "#,
            id
        )
        .fetch_optional(&mut **db)
        .await?;

        Ok(category)
    }

    pub async fn find_by_id_with_counts(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<Option<CategoryWithCounts>> {
        let result = sqlx::query!(
            r#"
            SELECT 
                c.id, c.name, c.title, c.parent_id, c.root_id, c.level, c.url, c.preview, 
                c.description, c.keywords, c.created, c.available, c.text, c.redirect, c.priority,
                (SELECT COUNT(*) FROM articles a WHERE a.category_id = c.id AND a.available_on_site = 1) as article_count,
                (SELECT COUNT(*) FROM categories sub WHERE sub.parent_id = c.id AND (sub.available = 1 OR sub.available IS NULL)) as subcategory_count
            FROM categories c
            WHERE c.id = ? AND (c.available = 1 OR c.available IS NULL)
            "#,
            id
        )
        .fetch_optional(&mut **db)
        .await?;

        match result {
            Some(row) => {
                let category = Category {
                    id: row.id,
                    name: row.name,
                    title: row.title,
                    parent_id: row.parent_id,
                    root_id: row.root_id,
                    level: row.level,
                    url: row.url,
                    preview: row.preview,
                    description: row.description,
                    keywords: row.keywords,
                    created: row.created,
                    available: row.available.map(|v| v != 0),
                    text: row.text,
                    redirect: row.redirect,
                    priority: row.priority,
                };

                Ok(Some(CategoryWithCounts {
                    category,
                    article_count: row.article_count.unwrap_or(0),
                    subcategory_count: row.subcategory_count.unwrap_or(0),
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn find_root_categories(db: &mut Connection<UniversalPathDb>) -> Result<Vec<CategoryWithCounts>> {
        let results = sqlx::query!(
            r#"
            SELECT 
                c.id, c.name, c.title, c.parent_id, c.root_id, c.level, c.url, c.preview, 
                c.description, c.keywords, c.created, c.available, c.text, c.redirect, c.priority,
                (SELECT COUNT(*) FROM articles a WHERE a.category_id = c.id AND a.available_on_site = 1) as article_count,
                (SELECT COUNT(*) FROM categories sub WHERE sub.parent_id = c.id AND (sub.available = 1 OR sub.available IS NULL)) as subcategory_count
            FROM categories c
            WHERE c.parent_id IS NULL AND (c.available = 1 OR c.available IS NULL)
            ORDER BY c.priority DESC, c.name
            "#
        )
        .fetch_all(&mut **db)
        .await?;

        let categories = results
            .into_iter()
            .map(|row| {
                let category = Category {
                    id: row.id,
                    name: row.name,
                    title: row.title,
                    parent_id: row.parent_id,
                    root_id: row.root_id,
                    level: row.level,
                    url: row.url,
                    preview: row.preview,
                    description: row.description,
                    keywords: row.keywords,
                    created: row.created,
                    available: row.available.map(|v| v != 0),
                    text: row.text,
                    redirect: row.redirect,
                    priority: row.priority,
                };

                CategoryWithCounts {
                    category,
                    article_count: row.article_count.unwrap_or(0),
                    subcategory_count: row.subcategory_count.unwrap_or(0),
                }
            })
            .collect();

        Ok(categories)
    }

    pub async fn find_subcategories(db: &mut Connection<UniversalPathDb>, parent_id: u32) -> Result<Vec<CategoryWithCounts>> {
        let results = sqlx::query!(
            r#"
            SELECT 
                c.id, c.name, c.title, c.parent_id, c.root_id, c.level, c.url, c.preview, 
                c.description, c.keywords, c.created, c.available, c.text, c.redirect, c.priority,
                (SELECT COUNT(*) FROM articles a WHERE a.category_id = c.id AND a.available_on_site = 1) as article_count,
                (SELECT COUNT(*) FROM categories sub WHERE sub.parent_id = c.id AND (sub.available = 1 OR sub.available IS NULL)) as subcategory_count
            FROM categories c
            WHERE c.parent_id = ? AND (c.available = 1 OR c.available IS NULL)
            ORDER BY c.priority DESC, c.name
            "#,
            parent_id
        )
        .fetch_all(&mut **db)
        .await?;

        let categories = results
            .into_iter()
            .map(|row| {
                let category = Category {
                    id: row.id,
                    name: row.name,
                    title: row.title,
                    parent_id: row.parent_id,
                    root_id: row.root_id,
                    level: row.level,
                    url: row.url,
                    preview: row.preview,
                    description: row.description,
                    keywords: row.keywords,
                    created: row.created,
                    available: row.available.map(|v| v != 0),
                    text: row.text,
                    redirect: row.redirect,
                    priority: row.priority,
                };

                CategoryWithCounts {
                    category,
                    article_count: row.article_count.unwrap_or(0),
                    subcategory_count: row.subcategory_count.unwrap_or(0),
                }
            })
            .collect();

        Ok(categories)
    }

    pub async fn build_category_path(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<Vec<Category>> {
        let mut path = Vec::new();
        let mut current_id = Some(id);

        while let Some(id) = current_id {
            if let Some(category) = Self::find_by_id(db, id).await? {
                current_id = category.parent_id;
                path.push(category);
            } else {
                current_id = None;
            }
        }

        path.reverse();
        Ok(path)
    }

    pub async fn create(db: &mut Connection<UniversalPathDb>, new_category: NewCategory) -> Result<u32> {
        // Calculate level and root_id based on parent_id
        let (level, root_id) = if let Some(parent_id) = new_category.parent_id {
            if let Some(parent) = Self::find_by_id(db, parent_id).await? {
                (parent.level + 1, parent.root_id.or(Some(parent.id)))
            } else {
                (0, None)
            }
        } else {
            (0, None)
        };

        let result = sqlx::query!(
            r#"
            INSERT INTO categories 
            (name, title, parent_id, root_id, level, url, preview, description,
             keywords, created, available, text, redirect, priority)
            VALUES
            (?, ?, ?, ?, ?, ?, ?, ?, ?, NOW(), ?, ?, ?, ?)
            "#,
            new_category.name,
            new_category.title,
            new_category.parent_id,
            root_id,
            level,
            new_category.url,
            new_category.preview,
            new_category.description,
            new_category.keywords,
            new_category.available,
            new_category.text,
            new_category.redirect,
            new_category.priority
        )
        .execute(&mut **db)
        .await?;

        Ok(result.last_insert_id() as u32)
    }

    pub async fn update(db: &mut Connection<UniversalPathDb>, update_category: UpdateCategory) -> Result<bool> {
        // If parent_id is changing, we need to recalculate level and root_id
        let (level, root_id) = if let Some(parent_id) = update_category.parent_id {
            if let Some(parent) = Self::find_by_id(db, parent_id).await? {
                (Some(parent.level + 1), parent.root_id.or(Some(parent.id)))
            } else {
                // If parent not found but parent_id provided, use defaults
                (Some(0), None)
            }
        } else if update_category.parent_id.is_some() {
            // Parent explicitly set to NULL
            (Some(0), None)
        } else {
            // Parent not changing
            (None, None)
        };

        let mut query = String::from("UPDATE categories SET");
        let mut params = Vec::new();
        let mut param_index = 1;

        if let Some(name) = &update_category.name {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" name = ?"));
            params.push(name.clone());
            param_index += 1;
        }

        if let Some(title) = &update_category.title {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" title = ?"));
            params.push(title.clone());
            param_index += 1;
        }

        if update_category.parent_id.is_some() {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" parent_id = ?"));
            params.push(update_category.parent_id.unwrap().to_string());
            param_index += 1;

            if let Some(lvl) = level {
                query.push_str(&format!(", level = ?"));
                params.push(lvl.to_string());
                param_index += 1;
            }

            if let Some(rt) = root_id {
                query.push_str(&format!(", root_id = ?"));
                params.push(rt.to_string());
                param_index += 1;
            }
        }

        if let Some(url) = &update_category.url {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" url = ?"));
            params.push(url.clone());
            param_index += 1;
        }

        if let Some(preview) = &update_category.preview {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" preview = ?"));
            params.push(preview.clone());
            param_index += 1;
        }

        if let Some(description) = &update_category.description {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" description = ?"));
            params.push(description.clone());
            param_index += 1;
        }

        if let Some(keywords) = &update_category.keywords {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" keywords = ?"));
            params.push(keywords.clone());
            param_index += 1;
        }

        if let Some(available) = &update_category.available {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" available = ?"));
            params.push(available.to_string());
            param_index += 1;
        }

        if let Some(text) = &update_category.text {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" text = ?"));
            params.push(text.clone());
            param_index += 1;
        }

        if let Some(redirect) = &update_category.redirect {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" redirect = ?"));
            params.push(redirect.clone());
            param_index += 1;
        }

        if let Some(priority) = &update_category.priority {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" priority = ?"));
            params.push(priority.to_string());
            param_index += 1;
        }

        // If nothing to update, return early
        if param_index == 1 {
            return Ok(true);
        }

        // Add the WHERE clause
        query.push_str(&format!(" WHERE id = ?"));
        params.push(update_category.id.to_string());

        // Use sqlx::query to execute the dynamic query
        let mut query_builder = sqlx::query(&query);
        for param in &params {
            query_builder = query_builder.bind(param);
        }

        // Fix: Cast database connection to the correct type
        let conn = &mut **db as &mut sqlx::MySqlConnection;
        let result = query_builder.execute(conn).await?;
        
        // Update also requires updating all subcategories' level and root_id if parent_id changed
        if update_category.parent_id.is_some() {
            Self::update_subcategories_hierarchy(db, update_category.id).await?;
        }

        Ok(result.rows_affected() > 0)
    }

    async fn update_subcategories_hierarchy(db: &mut Connection<UniversalPathDb>, parent_id: u32) -> Result<()> {
        // Get the parent to determine its level and root_id
        if let Some(parent) = Self::find_by_id(db, parent_id).await? {
            let parent_level = parent.level;
            let parent_root_id = parent.root_id.unwrap_or(parent.id);

            // Find all direct subcategories
            let subcategories = sqlx::query!(
                "SELECT id FROM categories WHERE parent_id = ?",
                parent_id
            )
            .fetch_all(&mut **db)
            .await?;

            // Update each subcategory
            for subcategory in subcategories {
                // Update the current subcategory
                sqlx::query!(
                    "UPDATE categories SET level = ?, root_id = ? WHERE id = ?",
                    parent_level + 1,
                    parent_root_id,
                    subcategory.id
                )
                .execute(&mut **db)
                .await?;

                // Recursively update its subcategories
                Self::update_subcategories_hierarchy(db, subcategory.id).await?;
            }
        }

        Ok(())
    }

    pub async fn delete(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<bool> {
        // Check for subcategories first
        let subcategories = sqlx::query!(
            "SELECT COUNT(*) as count FROM categories WHERE parent_id = ?",
            id
        )
        .fetch_one(&mut **db)
        .await?;

        if subcategories.count > 0 {
            // Cannot delete a category with subcategories
            return Ok(false);
        }

        // Check for articles
        let articles = sqlx::query!(
            "SELECT COUNT(*) as count FROM articles WHERE category_id = ?",
            id
        )
        .fetch_one(&mut **db)
        .await?;

        if articles.count > 0 {
            // Cannot delete a category with articles
            return Ok(false);
        }

        // Delete the category
        let result = sqlx::query!("DELETE FROM categories WHERE id = ?", id)
            .execute(&mut **db)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn search(db: &mut Connection<UniversalPathDb>, query: &str, limit: Option<u32>) -> Result<Vec<CategoryWithCounts>> {
        let limit = limit.unwrap_or(20);
        let search_query = format!("%{}%", query);

        let results = sqlx::query!(
            r#"
            SELECT 
                c.id, c.name, c.title, c.parent_id, c.root_id, c.level, c.url, c.preview, 
                c.description, c.keywords, c.created, c.available, c.text, c.redirect, c.priority,
                (SELECT COUNT(*) FROM articles a WHERE a.category_id = c.id AND a.available_on_site = 1) as article_count,
                (SELECT COUNT(*) FROM categories sub WHERE sub.parent_id = c.id AND (sub.available = 1 OR sub.available IS NULL)) as subcategory_count
            FROM categories c
            WHERE (c.available = 1 OR c.available IS NULL) AND 
                  (c.name LIKE ? OR c.title LIKE ? OR c.description LIKE ? OR c.keywords LIKE ?)
            ORDER BY c.priority DESC, c.name
            LIMIT ?
            "#,
            search_query, search_query, search_query, search_query, limit
        )
        .fetch_all(&mut **db)
        .await?;

        let categories = results
            .into_iter()
            .map(|row| {
                let category = Category {
                    id: row.id,
                    name: row.name,
                    title: row.title,
                    parent_id: row.parent_id,
                    root_id: row.root_id,
                    level: row.level,
                    url: row.url,
                    preview: row.preview,
                    description: row.description,
                    keywords: row.keywords,
                    created: row.created,
                    available: row.available.map(|v| v != 0),
                    text: row.text,
                    redirect: row.redirect,
                    priority: row.priority,
                };

                CategoryWithCounts {
                    category,
                    article_count: row.article_count.unwrap_or(0),
                    subcategory_count: row.subcategory_count.unwrap_or(0),
                }
            })
            .collect();

        Ok(categories)
    }

    pub async fn build_tree(db: &mut Connection<UniversalPathDb>, parent_id: Option<u32>) -> Result<Vec<CategoryTreeItem>> {
        let categories = match parent_id {
            Some(id) => Self::find_subcategories(db, id).await?,
            None => Self::find_root_categories(db).await?,
        };

        let mut tree = Vec::new();
        for category in categories {
            let children = Self::build_tree(db, Some(category.category.id)).await?;
            tree.push(CategoryTreeItem {
                category,
                children,
            });
        }

        Ok(tree)
    }
}