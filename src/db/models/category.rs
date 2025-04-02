use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::{sqlx, Connection};
use crate::db::UniversalPathDb;
use crate::db::connection::ConnectionExt;
use anyhow::Result;
use sqlx::Row; 

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Category {
    pub id: u32,
    pub title: String,
    pub note: Option<String>,
    pub intro: Option<String>,
    pub sub: Option<String>,
    pub priority: Option<u32>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub role: Option<String>,
    pub master: bool,
    pub short_title: Option<String>,
    pub available_on_site: bool,
    pub available_on_api: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CategoryWithCounts {
    pub category: Category,
    pub article_count: i64,
    pub subcategory_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CategoryTreeItem {
    pub category: CategoryWithCounts,
    pub children: Vec<CategoryTreeItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewCategory {
    pub title: String,
    pub note: Option<String>,
    pub intro: Option<String>,
    pub sub: Option<String>,
    pub priority: Option<i32>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub role: Option<String>,
    pub master: bool,
    pub short_title: Option<String>,
    pub available_on_site: bool,
    pub available_on_api: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateCategory {
    pub id: u32,
    pub title: Option<String>,
    pub note: Option<String>,
    pub intro: Option<String>,
    pub sub: Option<String>,
    pub priority: Option<u32>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub role: Option<String>,
    pub master: Option<bool>,
    pub short_title: Option<String>,
    pub available_on_site: Option<bool>,
    pub available_on_api: Option<bool>,
}

impl Category {
    pub async fn find_by_id(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<Option<Category>> {
        let conn = db.get_conn();
        
        // Use a simpler query approach with explicit typing
        let row = sqlx::query(
            r#"
            SELECT 
                id, title, note, intro, sub, priority, keywords, description,
                role, master, short_title, available_on_site, available_on_api
            FROM categories 
            WHERE id = ? AND (available_on_site = 1 OR available_on_site IS NULL)
            "#
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;

        match row {
            Some(row) => {
                Ok(Some(Category {
                    id: row.get::<u32, _>("id"),
                    title: row.get("title"),
                    note: row.get("note"),
                    intro: row.get("intro"),
                    sub: row.get("sub"),
                    priority: row.get::<Option<u32>, _>("priority"),
                    keywords: row.get("keywords"),
                    description: row.get("description"),
                    role: row.get("role"),
                    master: row.get::<i32, _>("master") > 0,
                    short_title: row.get("short_title"),
                    available_on_site: row.get::<i32, _>("available_on_site") > 0,
                    available_on_api: row.get::<i32, _>("available_on_api") > 0,
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn find_by_id_with_counts(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<Option<CategoryWithCounts>> {
        let conn = db.get_conn();
        
        let row = sqlx::query(
            r#"
            SELECT 
                c.id, c.title, c.note, c.intro, c.sub, c.priority, c.keywords, c.description,
                c.role, c.master, c.short_title, c.available_on_site, c.available_on_api,
                (SELECT COUNT(*) FROM articles a WHERE a.category_id = c.id AND a.available_on_site = 1) as article_count,
                (SELECT COUNT(*) FROM categories_has_categories chc WHERE chc.parent_id = c.id) as subcategory_count
            FROM categories c
            WHERE c.id = ? AND (c.available_on_site = 1 OR c.available_on_site IS NULL)
            "#
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;

        match row {
            Some(row) => {
                let category = Category {
                    id: row.get::<u32, _>("id"),
                    title: row.get("title"),
                    note: row.get("note"),
                    intro: row.get("intro"),
                    sub: row.get("sub"),
                    priority: row.get::<Option<u32>, _>("priority"),
                    keywords: row.get("keywords"),
                    description: row.get("description"),
                    role: row.get("role"),
                    master: row.get::<i32, _>("master") > 0,
                    short_title: row.get("short_title"),
                    available_on_site: row.get::<i32, _>("available_on_site") > 0,
                    available_on_api: row.get::<i32, _>("available_on_api") > 0,
                };

                Ok(Some(CategoryWithCounts {
                    category,
                    article_count: row.get::<i64, _>("article_count"),
                    subcategory_count: row.get::<i64, _>("subcategory_count"),
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn find_root_categories(db: &mut Connection<UniversalPathDb>) -> Result<Vec<CategoryWithCounts>> {
        let conn = db.get_conn();
        
        let rows = sqlx::query(
            r#"
            SELECT 
                c.id, c.title, c.note, c.intro, c.sub, c.priority, c.keywords, c.description,
                c.role, c.master, c.short_title, c.available_on_site, c.available_on_api,
                (SELECT COUNT(*) FROM articles a WHERE a.category_id = c.id AND a.available_on_site = 1) as article_count,
                (SELECT COUNT(*) FROM categories_has_categories chc WHERE chc.parent_id = c.id) as subcategory_count
            FROM categories c
            WHERE c.id NOT IN (
                SELECT child_id1 FROM categories_has_categories
            ) AND (c.available_on_site = 1 OR c.available_on_site IS NULL)
            ORDER BY c.priority DESC, c.title
            "#
        )
        .fetch_all(conn)
        .await?;

        let categories = rows
            .into_iter()
            .map(|row| {
                let category = Category {
                    id: row.get::<u32, _>("id"),
                    title: row.get("title"),
                    note: row.get("note"),
                    intro: row.get("intro"),
                    sub: row.get("sub"),
                    priority: row.get::<Option<u32>, _>("priority"),
                    keywords: row.get("keywords"),
                    description: row.get("description"),
                    role: row.get("role"),
                    master: row.get::<i32, _>("master") > 0,
                    short_title: row.get("short_title"),
                    available_on_site: row.get::<i32, _>("available_on_site") > 0,
                    available_on_api: row.get::<i32, _>("available_on_api") > 0,
                };

                CategoryWithCounts {
                    category,
                    article_count: row.get::<i64, _>("article_count"),
                    subcategory_count: row.get::<i64, _>("subcategory_count"),
                }
            })
            .collect();

        Ok(categories)
    }

    pub async fn find_subcategories(db: &mut Connection<UniversalPathDb>, parent_id: u32) -> Result<Vec<CategoryWithCounts>> {
        let conn = db.get_conn();
        
        let rows = sqlx::query(
            r#"
            SELECT 
                c.id, c.title, c.note, c.intro, c.sub, c.priority, c.keywords, c.description,
                c.role, c.master, c.short_title, c.available_on_site, c.available_on_api,
                (SELECT COUNT(*) FROM articles a WHERE a.category_id = c.id AND a.available_on_site = 1) as article_count,
                (SELECT COUNT(*) FROM categories_has_categories subchc WHERE subchc.parent_id = c.id) as subcategory_count
            FROM categories c
            JOIN categories_has_categories chc ON chc.child_id1 = c.id
            WHERE chc.parent_id = ? AND (c.available_on_site = 1 OR c.available_on_site IS NULL)
            ORDER BY chc.priority IS NULL, chc.priority ASC, c.priority DESC, c.title
            "#
        )
        .bind(parent_id)
        .fetch_all(conn)
        .await?;

        let categories = rows
            .into_iter()
            .map(|row| {
                let category = Category {
                    id: row.get::<u32, _>("id"),
                    title: row.get("title"),
                    note: row.get("note"),
                    intro: row.get("intro"),
                    sub: row.get("sub"),
                    priority: row.get::<Option<u32>, _>("priority"),
                    keywords: row.get("keywords"),
                    description: row.get("description"),
                    role: row.get("role"),
                    master: row.get::<i32, _>("master") > 0,
                    short_title: row.get("short_title"),
                    available_on_site: row.get::<i32, _>("available_on_site") > 0,
                    available_on_api: row.get::<i32, _>("available_on_api") > 0,
                };

                CategoryWithCounts {
                    category,
                    article_count: row.get::<i64, _>("article_count"),
                    subcategory_count: row.get::<i64, _>("subcategory_count"),
                }
            })
            .collect();

        Ok(categories)
    }

    pub async fn build_category_path(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<Vec<Category>> {
        let mut path = Vec::new();
        
        // First get the current category
        if let Some(category) = Self::find_by_id(db, id).await? {
            path.push(category);
            
            // Then trace upwards through parents
            let mut current_id = id;
            let mut more_parents = true;
            
            while more_parents {
                // Get the parent ID for the current category
                let parent_id_result = sqlx::query("SELECT parent_id FROM categories_has_categories WHERE child_id1 = ?")
                    .bind(current_id)
                    .fetch_optional(db.get_conn())
                    .await?;
                
                match parent_id_result {
                    Some(row) => {
                        let parent_id: u32 = row.get("parent_id");
                        
                        // Get the parent category
                        if let Some(parent_category) = Self::find_by_id(db, parent_id).await? {
                            path.push(parent_category);
                            current_id = parent_id;
                        } else {
                            more_parents = false;
                        }
                    },
                    None => more_parents = false
                }
            }
        }

        path.reverse();
        Ok(path)
    }

    pub async fn create(db: &mut Connection<UniversalPathDb>, new_category: NewCategory) -> Result<u32> {
        let conn = db.get_conn();
        
        let result = sqlx::query!(
            r#"
            INSERT INTO categories 
            (title, note, intro, sub, priority, keywords, description,
            role, master, short_title, available_on_site, available_on_api)
            VALUES
            (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            new_category.title,
            new_category.note,
            new_category.intro,
            new_category.sub,
            new_category.priority,
            new_category.keywords,
            new_category.description,
            new_category.role,
            new_category.master,
            new_category.short_title,
            new_category.available_on_site,
            new_category.available_on_api
        )
        .execute(conn)
        .await?;

        Ok(result.last_insert_id() as u32)
    }

    pub async fn update(db: &mut Connection<UniversalPathDb>, update_category: UpdateCategory) -> Result<bool> {
        let conn = db.get_conn();
        
        let mut query = String::from("UPDATE categories SET");
        let mut params = Vec::new();
        let mut param_index = 1;

        if let Some(title) = &update_category.title {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" title = ?");
            params.push(title.clone());
            param_index += 1;
        }

        if let Some(note) = &update_category.note {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" note = ?");
            params.push(note.clone());
            param_index += 1;
        }

        if let Some(intro) = &update_category.intro {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" intro = ?");
            params.push(intro.clone());
            param_index += 1;
        }

        if let Some(sub) = &update_category.sub {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" sub = ?");
            params.push(sub.clone());
            param_index += 1;
        }

        if let Some(priority) = &update_category.priority {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" priority = ?");
            params.push(priority.to_string());
            param_index += 1;
        }

        if let Some(keywords) = &update_category.keywords {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" keywords = ?");
            params.push(keywords.clone());
            param_index += 1;
        }

        if let Some(description) = &update_category.description {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" description = ?");
            params.push(description.clone());
            param_index += 1;
        }

        if let Some(role) = &update_category.role {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" role = ?");
            params.push(role.clone());
            param_index += 1;
        }

        if let Some(master) = &update_category.master {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" master = ?");
            params.push(master.to_string());
            param_index += 1;
        }

        if let Some(short_title) = &update_category.short_title {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" short_title = ?");
            params.push(short_title.clone());
            param_index += 1;
        }

        if let Some(available_on_site) = &update_category.available_on_site {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" available_on_site = ?");
            params.push(available_on_site.to_string());
            param_index += 1;
        }

        if let Some(available_on_api) = &update_category.available_on_api {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" available_on_api = ?");
            params.push(available_on_api.to_string());
            param_index += 1;
        }

        // If nothing to update, return early
        if param_index == 1 {
            return Ok(true);
        }

        // Add the WHERE clause
        query.push_str(" WHERE id = ?");
        params.push(update_category.id.to_string());

        // Use sqlx::query to execute the dynamic query
        let mut query_builder = sqlx::query(&query);
        for param in &params {
            query_builder = query_builder.bind(param);
        }

        let result = query_builder.execute(conn).await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<bool> {
        // Check for subcategories first
        let subcategories_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM categories_has_categories WHERE parent_id = ?")
            .bind(id)
            .fetch_one(db.get_conn())
            .await?;

        if subcategories_count > 0 {
            // Cannot delete a category with subcategories
            return Ok(false);
        }

        // Check for articles
        let articles_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM articles WHERE category_id = ?")
            .bind(id)
            .fetch_one(db.get_conn())
            .await?;

        if articles_count > 0 {
            // Cannot delete a category with articles
            return Ok(false);
        }

        // Delete any relationships where this is a child
        sqlx::query("DELETE FROM categories_has_categories WHERE child_id1 = ?")
            .bind(id)
            .execute(db.get_conn())
            .await?;

        // Delete the category
        let result = sqlx::query("DELETE FROM categories WHERE id = ?")
            .bind(id)
            .execute(db.get_conn())
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn search(db: &mut Connection<UniversalPathDb>, query: &str, limit: Option<u32>) -> Result<Vec<CategoryWithCounts>> {
        let conn = db.get_conn();
        
        let limit = limit.unwrap_or(20);
        let search_query = format!("%{}%", query);

        let rows = sqlx::query(
            r#"
            SELECT 
                c.id, c.title, c.note, c.intro, c.sub, c.priority, c.keywords, c.description,
                c.role, c.master, c.short_title, c.available_on_site, c.available_on_api,
                (SELECT COUNT(*) FROM articles a WHERE a.category_id = c.id AND a.available_on_site = 1) as article_count,
                (SELECT COUNT(*) FROM categories_has_categories chc WHERE chc.parent_id = c.id) as subcategory_count
            FROM categories c
            WHERE (c.available_on_site = 1 OR c.available_on_site IS NULL) AND 
                  (c.title LIKE ? OR c.description LIKE ? OR c.keywords LIKE ?)
            ORDER BY c.priority DESC, c.title
            LIMIT ?
            "#
        )
        .bind(&search_query)
        .bind(&search_query)
        .bind(&search_query)
        .bind(limit)
        .fetch_all(conn)
        .await?;

        let categories = rows
            .into_iter()
            .map(|row| {
                let category = Category {
                    id: row.get::<u32, _>("id"),
                    title: row.get("title"),
                    note: row.get("note"),
                    intro: row.get("intro"),
                    sub: row.get("sub"),
                    priority: row.get::<Option<u32>, _>("priority"),
                    keywords: row.get("keywords"),
                    description: row.get("description"),
                    role: row.get("role"),
                    master: row.get::<u32, _>("master") > 0,
                    short_title: row.get("short_title"),
                    available_on_site: row.get::<i32, _>("available_on_site") > 0,
                    available_on_api: row.get::<i32, _>("available_on_api") > 0,
                };

                CategoryWithCounts {
                    category,
                    article_count: row.get::<i64, _>("article_count"),
                    subcategory_count: row.get::<i64, _>("subcategory_count"),
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
            // Use Box::pin to handle the recursion in async function
            let children = Box::pin(Self::build_tree(db, Some(category.category.id))).await?;
            tree.push(CategoryTreeItem {
                category,
                children,
            });
        }
    
        Ok(tree)
    }
    
    // Add parent-child relationship
    pub async fn add_subcategory(db: &mut Connection<UniversalPathDb>, parent_id: u32, child_id: u32, priority: Option<u32>) -> Result<bool> {
        let conn = db.get_conn();
        
        let priority = priority.unwrap_or(0);
        
        let result = sqlx::query(
            r#"
            INSERT INTO categories_has_categories 
            (parent_id, child_id1, priority)
            VALUES (?, ?, ?)
            "#
        )
        .bind(parent_id)
        .bind(child_id)
        .bind(priority)
        .execute(conn)
        .await?;
        
        Ok(result.rows_affected() > 0)
    }
    
    // Remove parent-child relationship
    pub async fn remove_subcategory(db: &mut Connection<UniversalPathDb>, parent_id: u32, child_id: u32) -> Result<bool> {
        let conn = db.get_conn();
        
        let result = sqlx::query(
            r#"
            DELETE FROM categories_has_categories 
            WHERE parent_id = ? AND child_id1 = ?
            "#
        )
        .bind(parent_id)
        .bind(child_id)
        .execute(conn)
        .await?;
        
        Ok(result.rows_affected() > 0)
    }
}