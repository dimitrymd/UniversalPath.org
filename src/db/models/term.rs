use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::{sqlx, Connection};
use crate::db::UniversalPathDb;
use crate::db::connection::ConnectionExt;
use anyhow::Result;
use sqlx::Row; 

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Term {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub first_letter: String,
    pub created: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewTerm {
    pub title: String,
    pub description: String,
    pub first_letter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateTerm {
    pub id: u32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub first_letter: Option<String>,
}

impl Term {
    pub async fn find_by_id(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<Option<Term>> {
        let conn = db.get_conn();
        
        // Use a simpler query to avoid type conversion issues
        let row = sqlx::query(
            r#"
            SELECT id, title, description, first_letter, created
            FROM Term 
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;

        match row {
            Some(row) => {
                let term = Term {
                    id: row.get::<u32, _>("id"),
                    title: row.get("title"),
                    description: row.get("description"),
                    first_letter: row.get("first_letter"),
                    created: row.get("created"),
                };
                Ok(Some(term))
            }
            None => Ok(None),
        }
    }

    pub async fn find_all(db: &mut Connection<UniversalPathDb>) -> Result<Vec<Term>> {
        let conn = db.get_conn();
        
        // Use a simpler query to avoid type conversion issues
        let rows = sqlx::query(
            r#"
            SELECT id, title, description, first_letter, created
            FROM Term 
            ORDER BY title
            "#
        )
        .fetch_all(conn)
        .await?;

        let terms = rows
            .into_iter()
            .map(|row| {
                Term {
                    id: row.get::<u32, _>("id"),
                    title: row.get("title"),
                    description: row.get("description"),
                    first_letter: row.get("first_letter"),
                    created: row.get("created"),
                }
            })
            .collect();

        Ok(terms)
    }

    pub async fn find_by_letter(db: &mut Connection<UniversalPathDb>, letter: &str) -> Result<Vec<Term>> {
        let conn = db.get_conn();
        
        // Use a simpler query to avoid type conversion issues
        let rows = sqlx::query(
            r#"
            SELECT id, title, description, first_letter, created
            FROM Term 
            WHERE first_letter = ?
            ORDER BY title
            "#
        )
        .bind(letter)
        .fetch_all(conn)
        .await?;

        let terms = rows
            .into_iter()
            .map(|row| {
                Term {
                    id: row.get::<u32, _>("id"),
                    title: row.get("title"),
                    description: row.get("description"),
                    first_letter: row.get("first_letter"),
                    created: row.get("created"),
                }
            })
            .collect();

        Ok(terms)
    }

    pub async fn get_all_letters(db: &mut Connection<UniversalPathDb>) -> Result<Vec<String>> {
        let conn = db.get_conn();
        
        let rows = sqlx::query(
            r#"
            SELECT DISTINCT first_letter
            FROM Term 
            ORDER BY first_letter
            "#
        )
        .fetch_all(conn)
        .await?;

        let letters = rows
            .into_iter()
            .map(|row| row.get::<String, _>("first_letter"))
            .collect();

        Ok(letters)
    }

    pub async fn create(db: &mut Connection<UniversalPathDb>, new_term: NewTerm) -> Result<u32> {
        let conn = db.get_conn();
        
        let result = sqlx::query(
            r#"
            INSERT INTO Term 
            (title, description, first_letter, created)
            VALUES
            (?, ?, ?, NOW())
            "#
        )
        .bind(&new_term.title)
        .bind(&new_term.description)
        .bind(&new_term.first_letter)
        .execute(conn)
        .await?;

        Ok(result.last_insert_id() as u32)
    }

    pub async fn update(db: &mut Connection<UniversalPathDb>, update_term: UpdateTerm) -> Result<bool> {
        let conn = db.get_conn();
        
        let mut query = String::from("UPDATE Term SET");
        let mut params = Vec::new();
        let mut param_index = 1;

        if let Some(title) = &update_term.title {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" title = ?");
            params.push(title.clone());
            param_index += 1;
        }

        if let Some(description) = &update_term.description {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" description = ?");
            params.push(description.clone());
            param_index += 1;
        }

        if let Some(first_letter) = &update_term.first_letter {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" first_letter = ?");
            params.push(first_letter.clone());
            param_index += 1;
        }

        // If nothing to update, return early
        if param_index == 1 {
            return Ok(true);
        }

        // Add the WHERE clause
        query.push_str(" WHERE id = ?");
        params.push(update_term.id.to_string());

        // Use sqlx::query to execute the dynamic query
        let mut query_builder = sqlx::query(&query);
        for param in &params {
            query_builder = query_builder.bind(param);
        }

        let result = query_builder.execute(conn).await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<bool> {
        let conn = db.get_conn();
        
        let result = sqlx::query("DELETE FROM Term WHERE id = ?")
            .bind(id)
            .execute(conn)
            .await?;
        
        Ok(result.rows_affected() > 0)
    }

    pub async fn search(db: &mut Connection<UniversalPathDb>, query: &str) -> Result<Vec<Term>> {
        let conn = db.get_conn();
        
        let search_query = format!("%{}%", query);

        // Use a simpler query to avoid type conversion issues
        let rows = sqlx::query(
            r#"
            SELECT id, title, description, first_letter, created
            FROM Term 
            WHERE title LIKE ? OR description LIKE ?
            ORDER BY title
            "#
        )
        .bind(&search_query)
        .bind(&search_query)
        .fetch_all(conn)
        .await?;

        let terms = rows
            .into_iter()
            .map(|row| {
                Term {
                    id: row.get::<u32, _>("id"),
                    title: row.get("title"),
                    description: row.get("description"),
                    first_letter: row.get("first_letter"),
                    created: row.get("created"),
                }
            })
            .collect();

        Ok(terms)
    }
}