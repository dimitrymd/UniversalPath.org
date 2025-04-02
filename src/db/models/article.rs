use chrono::{NaiveDate, NaiveDateTime};
use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::{sqlx, Connection};
use crate::db::UniversalPathDb;
use crate::db::connection::ConnectionExt;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Article {
    pub id: u32,
    pub title: Option<String>,
    pub release_date: Option<NaiveDateTime>,
    pub publish_date: Option<NaiveDate>,
    pub author_id: u32,
    pub note: Option<String>,
    pub category_id: Option<u32>,
    pub resume: Option<String>,
    pub txtfield: Option<String>,
    pub copyright: Option<String>,
    pub lasteditedby_userid: u32,
    pub lastedited_date: Option<NaiveDate>,
    pub priority: Option<u32>,
    #[serde(rename = "type_")]
    pub type_: Option<String>,
    pub event_id: Option<u32>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub short_title: Option<String>,
    pub available_on_site: bool,
    pub available_on_api: bool,
    pub master: bool,
    pub new_: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ArticleWithAuthor {
    #[serde(flatten)]
    pub article: Article,
    pub author_name: Option<String>,
    pub category_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewArticle {
    pub title: Option<String>,
    pub release_date: Option<NaiveDateTime>,
    pub publish_date: Option<NaiveDate>,
    pub author_id: u32,
    pub note: Option<String>,
    pub category_id: Option<u32>,
    pub resume: Option<String>,
    pub txtfield: Option<String>,
    pub copyright: Option<String>,
    pub priority: Option<u32>,
    #[serde(rename = "type_")]
    pub type_: Option<String>,
    pub event_id: Option<u32>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub short_title: Option<String>,
    pub available_on_site: bool,
    pub available_on_api: bool,
    pub master: bool,
    pub new_: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateArticle {
    pub id: u32,
    pub title: Option<String>,
    pub release_date: Option<NaiveDateTime>,
    pub publish_date: Option<NaiveDate>,
    pub author_id: Option<u32>,
    pub note: Option<String>,
    pub category_id: Option<u32>,
    pub resume: Option<String>,
    pub txtfield: Option<String>,
    pub copyright: Option<String>,
    pub lasteditedby_userid: u32,
    pub priority: Option<u32>,
    #[serde(rename = "type_")]
    pub type_: Option<String>,
    pub event_id: Option<u32>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub short_title: Option<String>,
    pub available_on_site: Option<bool>,
    pub available_on_api: Option<bool>,
    pub master: Option<bool>,
    pub new_: Option<bool>,
}

impl Article {
    pub async fn find_by_id(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<Option<Article>> {
        let conn = db.get_conn();
        
        let article = sqlx::query_as!(
            Article,
            r#"
            SELECT 
                id, title, release_date, publish_date, author_id, note, category_id, 
                resume, txtfield, copyright, lasteditedby_userid, lastedited_date, 
                priority, type as "type_: String", event_id, keywords, description, 
                short_title, 
                available_on_site as "available_on_site: bool", 
                available_on_api as "available_on_api: bool", 
                master as "master: bool", 
                new_ as "new_: bool"
            FROM articles 
            WHERE id = ? AND available_on_site = 1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(article)
    }

    pub async fn find_by_id_with_author(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<Option<ArticleWithAuthor>> {
        let conn = db.get_conn();
        
        let result = sqlx::query!(
            r#"
            SELECT 
                a.id, a.title, a.release_date, a.publish_date, a.author_id, a.note, a.category_id, 
                a.resume, a.txtfield, a.copyright, a.lasteditedby_userid, a.lastedited_date, 
                a.priority, a.type as "type_", a.event_id, a.keywords, a.description, 
                a.short_title, a.available_on_site, a.available_on_api, a.master, a.new_,
                u.name as author_name, c.title as category_name
            FROM articles a
            LEFT JOIN users u ON a.author_id = u.id
            LEFT JOIN categories c ON a.category_id = c.id
            WHERE a.id = ? AND a.available_on_site = 1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        match result {
            Some(row) => {
                let article = Article {
                    id: row.id,
                    title: row.title,
                    release_date: row.release_date,
                    publish_date: row.publish_date,
                    author_id: row.author_id,
                    note: row.note,
                    category_id: row.category_id,
                    resume: row.resume,
                    txtfield: row.txtfield,
                    copyright: row.copyright,
                    lasteditedby_userid: row.lasteditedby_userid,
                    lastedited_date: row.lastedited_date,
                    priority: row.priority,
                    type_: row.type_,
                    event_id: row.event_id,
                    keywords: row.keywords,
                    description: row.description,
                    short_title: row.short_title,
                    available_on_site: row.available_on_site != 0,
                    available_on_api: row.available_on_api != 0,
                    master: row.master != 0,
                    new_: row.new_ != 0,
                };

                Ok(Some(ArticleWithAuthor {
                    article,
                    author_name: row.author_name,
                    category_name: row.category_name,
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn find_recent(db: &mut Connection<UniversalPathDb>, limit: u32) -> Result<Vec<ArticleWithAuthor>> {
        let conn = db.get_conn();
        
        let results = sqlx::query!(
            r#"
            SELECT 
                a.id, a.title, a.release_date, a.publish_date, a.author_id, a.note, a.category_id, 
                a.resume, a.txtfield, a.copyright, a.lasteditedby_userid, a.lastedited_date, 
                a.priority, a.type as "type_", a.event_id, a.keywords, a.description, 
                a.short_title, a.available_on_site, a.available_on_api, a.master, a.new_,
                u.name as author_name, c.title as category_name
            FROM articles a
            LEFT JOIN users u ON a.author_id = u.id
            LEFT JOIN categories c ON a.category_id = c.id
            WHERE a.available_on_site = 1
            ORDER BY a.publish_date DESC, a.id DESC
            LIMIT ?
            "#,
            limit
        )
        .fetch_all(conn)
        .await?;

        let articles = results
            .into_iter()
            .map(|row| {
                let article = Article {
                    id: row.id,
                    title: row.title,
                    release_date: row.release_date,
                    publish_date: row.publish_date,
                    author_id: row.author_id,
                    note: row.note,
                    category_id: row.category_id,
                    resume: row.resume,
                    txtfield: row.txtfield,
                    copyright: row.copyright,
                    lasteditedby_userid: row.lasteditedby_userid,
                    lastedited_date: row.lastedited_date,
                    priority: row.priority,
                    type_: row.type_,
                    event_id: row.event_id,
                    keywords: row.keywords,
                    description: row.description,
                    short_title: row.short_title,
                    available_on_site: row.available_on_site != 0,
                    available_on_api: row.available_on_api != 0,
                    master: row.master != 0,
                    new_: row.new_ != 0,
                };

                ArticleWithAuthor {
                    article,
                    author_name: row.author_name,
                    category_name: row.category_name,
                }
            })
            .collect();

        Ok(articles)
    }

    pub async fn find_by_category(db: &mut Connection<UniversalPathDb>, category_id: u32, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<ArticleWithAuthor>> {
        let conn = db.get_conn();
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);

        let results = sqlx::query!(
            r#"
            SELECT 
                a.id, a.title, a.release_date, a.publish_date, a.author_id, a.note, a.category_id, 
                a.resume, a.txtfield, a.copyright, a.lasteditedby_userid, a.lastedited_date, 
                a.priority, a.type as "type_", a.event_id, a.keywords, a.description, 
                a.short_title, a.available_on_site, a.available_on_api, a.master, a.new_,
                u.name as author_name, c.title as category_name
            FROM articles a
            LEFT JOIN users u ON a.author_id = u.id
            LEFT JOIN categories c ON a.category_id = c.id
            WHERE a.category_id = ? AND a.available_on_site = 1
            ORDER BY a.priority DESC, a.publish_date DESC, a.id DESC
            LIMIT ? OFFSET ?
            "#,
            category_id, limit, offset
        )
        .fetch_all(conn)
        .await?;

        let articles = results
            .into_iter()
            .map(|row| {
                let article = Article {
                    id: row.id,
                    title: row.title,
                    release_date: row.release_date,
                    publish_date: row.publish_date,
                    author_id: row.author_id,
                    note: row.note,
                    category_id: row.category_id,
                    resume: row.resume,
                    txtfield: row.txtfield,
                    copyright: row.copyright,
                    lasteditedby_userid: row.lasteditedby_userid,
                    lastedited_date: row.lastedited_date,
                    priority: row.priority,
                    type_: row.type_,
                    event_id: row.event_id,
                    keywords: row.keywords,
                    description: row.description,
                    short_title: row.short_title,
                    available_on_site: row.available_on_site != 0,
                    available_on_api: row.available_on_api != 0,
                    master: row.master != 0,
                    new_: row.new_ != 0,
                };

                ArticleWithAuthor {
                    article,
                    author_name: row.author_name,
                    category_name: row.category_name,
                }
            })
            .collect();

        Ok(articles)
    }

    pub async fn search(db: &mut Connection<UniversalPathDb>, query: &str, limit: Option<u32>) -> Result<Vec<ArticleWithAuthor>> {
        let conn = db.get_conn();
        
        let limit = limit.unwrap_or(20);
        let search_query = format!("%{}%", query);

        let results = sqlx::query!(
            r#"
            SELECT 
                a.id, a.title, a.release_date, a.publish_date, a.author_id, a.note, a.category_id, 
                a.resume, a.txtfield, a.copyright, a.lasteditedby_userid, a.lastedited_date, 
                a.priority, a.type as "type_", a.event_id, a.keywords, a.description, 
                a.short_title, a.available_on_site, a.available_on_api, a.master, a.new_,
                u.name as author_name, c.title as category_name
            FROM articles a
            LEFT JOIN users u ON a.author_id = u.id
            LEFT JOIN categories c ON a.category_id = c.id
            WHERE a.available_on_site = 1 AND 
                  (a.title LIKE ? OR a.resume LIKE ? OR a.txtfield LIKE ? OR a.keywords LIKE ?)
            ORDER BY a.priority DESC, a.publish_date DESC, a.id DESC
            LIMIT ?
            "#,
            search_query, search_query, search_query, search_query, limit
        )
        .fetch_all(conn)
        .await?;

        let articles = results
            .into_iter()
            .map(|row| {
                let article = Article {
                    id: row.id,
                    title: row.title,
                    release_date: row.release_date,
                    publish_date: row.publish_date,
                    author_id: row.author_id,
                    note: row.note,
                    category_id: row.category_id,
                    resume: row.resume,
                    txtfield: row.txtfield,
                    copyright: row.copyright,
                    lasteditedby_userid: row.lasteditedby_userid,
                    lastedited_date: row.lastedited_date,
                    priority: row.priority,
                    type_: row.type_,
                    event_id: row.event_id,
                    keywords: row.keywords,
                    description: row.description,
                    short_title: row.short_title,
                    available_on_site: row.available_on_site != 0,
                    available_on_api: row.available_on_api != 0,
                    master: row.master != 0,
                    new_: row.new_ != 0,
                };

                ArticleWithAuthor {
                    article,
                    author_name: row.author_name,
                    category_name: row.category_name,
                }
            })
            .collect();

        Ok(articles)
    }

    pub async fn create(
        db: &mut Connection<UniversalPathDb>, 
        new_article: NewArticle,
        user_id: u32
    ) -> Result<u32> {
        let conn = db.get_conn();
        
        let result = sqlx::query!(
            r#"
            INSERT INTO articles 
            (title, release_date, publish_date, author_id, note, category_id,
             resume, txtfield, copyright, lasteditedby_userid, lastedited_date,
             priority, type, event_id, keywords, description, short_title,
             available_on_site, available_on_api, master, new_)
            VALUES
            (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURDATE(), ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            new_article.title,
            new_article.release_date,
            new_article.publish_date,
            new_article.author_id,
            new_article.note,
            new_article.category_id,
            new_article.resume,
            new_article.txtfield,
            new_article.copyright,
            user_id,
            new_article.priority,
            new_article.type_,
            new_article.event_id,
            new_article.keywords,
            new_article.description,
            new_article.short_title,
            new_article.available_on_site,
            new_article.available_on_api,
            new_article.master,
            new_article.new_
        )
        .execute(conn)
        .await?;

        Ok(result.last_insert_id() as u32)
    }

    pub async fn update(
        db: &mut Connection<UniversalPathDb>, 
        update_article: UpdateArticle
    ) -> Result<bool> {
        let conn = db.get_conn();
        
        let mut query = String::from("UPDATE articles SET lasteditedby_userid = ?, lastedited_date = CURDATE()");

        if update_article.title.is_some() {
            query.push_str(", title = ?");
        }
        if update_article.release_date.is_some() {
            query.push_str(", release_date = ?");
        }
        if update_article.publish_date.is_some() {
            query.push_str(", publish_date = ?");
        }
        if update_article.author_id.is_some() {
            query.push_str(", author_id = ?");
        }
        if update_article.note.is_some() {
            query.push_str(", note = ?");
        }
        if update_article.category_id.is_some() {
            query.push_str(", category_id = ?");
        }
        if update_article.resume.is_some() {
            query.push_str(", resume = ?");
        }
        if update_article.txtfield.is_some() {
            query.push_str(", txtfield = ?");
        }
        if update_article.copyright.is_some() {
            query.push_str(", copyright = ?");
        }
        if update_article.priority.is_some() {
            query.push_str(", priority = ?");
        }
        if update_article.type_.is_some() {
            query.push_str(", type = ?");
        }
        if update_article.event_id.is_some() {
            query.push_str(", event_id = ?");
        }
        if update_article.keywords.is_some() {
            query.push_str(", keywords = ?");
        }
        if update_article.description.is_some() {
            query.push_str(", description = ?");
        }
        if update_article.short_title.is_some() {
            query.push_str(", short_title = ?");
        }
        if update_article.available_on_site.is_some() {
            query.push_str(", available_on_site = ?");
        }
        if update_article.available_on_api.is_some() {
            query.push_str(", available_on_api = ?");
        }
        if update_article.master.is_some() {
            query.push_str(", master = ?");
        }
        if update_article.new_.is_some() {
            query.push_str(", new_ = ?");
        }

        query.push_str(" WHERE id = ?");

        let mut query_builder = sqlx::query(&query);

        // Add parameters in the same order as the placeholders
        query_builder = query_builder.bind(update_article.lasteditedby_userid);

        if let Some(title) = &update_article.title {
            query_builder = query_builder.bind(title);
        }
        if let Some(release_date) = &update_article.release_date {
            query_builder = query_builder.bind(release_date);
        }
        if let Some(publish_date) = &update_article.publish_date {
            query_builder = query_builder.bind(publish_date);
        }
        if let Some(author_id) = &update_article.author_id {
            query_builder = query_builder.bind(author_id);
        }
        if let Some(note) = &update_article.note {
            query_builder = query_builder.bind(note);
        }
        if let Some(category_id) = &update_article.category_id {
            query_builder = query_builder.bind(category_id);
        }
        if let Some(resume) = &update_article.resume {
            query_builder = query_builder.bind(resume);
        }
        if let Some(txtfield) = &update_article.txtfield {
            query_builder = query_builder.bind(txtfield);
        }
        if let Some(copyright) = &update_article.copyright {
            query_builder = query_builder.bind(copyright);
        }
        if let Some(priority) = &update_article.priority {
            query_builder = query_builder.bind(priority);
        }
        if let Some(type_) = &update_article.type_ {
            query_builder = query_builder.bind(type_);
        }
        if let Some(event_id) = &update_article.event_id {
            query_builder = query_builder.bind(event_id);
        }
        if let Some(keywords) = &update_article.keywords {
            query_builder = query_builder.bind(keywords);
        }
        if let Some(description) = &update_article.description {
            query_builder = query_builder.bind(description);
        }
        if let Some(short_title) = &update_article.short_title {
            query_builder = query_builder.bind(short_title);
        }
        if let Some(available_on_site) = &update_article.available_on_site {
            query_builder = query_builder.bind(available_on_site);
        }
        if let Some(available_on_api) = &update_article.available_on_api {
            query_builder = query_builder.bind(available_on_api);
        }
        if let Some(master) = &update_article.master {
            query_builder = query_builder.bind(master);
        }
        if let Some(new_) = &update_article.new_ {
            query_builder = query_builder.bind(new_);
        }

        // Finally, add the id for the WHERE clause
        query_builder = query_builder.bind(update_article.id);

        let result = query_builder.execute(conn).await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<bool> {
        let conn = db.get_conn();
        
        let result = sqlx::query!("DELETE FROM articles WHERE id = ?", id)
            .execute(conn)
            .await?;
        
        Ok(result.rows_affected() > 0)
    }
}