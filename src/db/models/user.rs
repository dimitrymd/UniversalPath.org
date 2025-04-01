use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::{sqlx, Connection};
use sqlx::Row;
use crate::db::UniversalPathDb;
use crate::db::connection::ConnectionExt;
use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use md5;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: u32,
    pub username: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub status: i32,
    pub is_admin: bool,
    pub created: Option<chrono::NaiveDateTime>,
    pub last_login: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewUser {
    pub username: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateUser {
    pub id: u32,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub status: Option<i32>,
    pub is_admin: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserToken {
    pub id: u32,
    pub username: String,
    pub is_admin: bool,
    pub exp: u64,
}

impl User {
    pub async fn find_by_id(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<Option<User>> {
        let conn = db.get_conn();
        
        // Use a simpler query approach to avoid issues with NULL fields
        let row = sqlx::query(
            r#"
            SELECT 
                id, login, name, email, pass,
                CASE WHEN rights = 'A' THEN 1 ELSE 0 END as is_admin
            FROM users 
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        
        match row {
            Some(row) => {
                let user = User {
                    id: row.get("id"),
                    username: row.get("login"),
                    email: row.get("email"),
                    password: row.get("pass"),
                    firstname: None,
                    lastname: Some(row.get("name")),
                    status: 1, // Default active status
                    is_admin: row.get::<i32, _>("is_admin") != 0,
                    created: None,
                    last_login: None,
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    pub async fn find_by_username(db: &mut Connection<UniversalPathDb>, username: &str) -> Result<Option<User>> {
        let conn = db.get_conn();
        
        // Use a simpler query approach to avoid issues with NULL fields
        let row = sqlx::query(
            r#"
            SELECT 
                id, login, name, email, pass,
                CASE WHEN rights = 'A' THEN 1 ELSE 0 END as is_admin
            FROM users 
            WHERE login = ?
            "#
        )
        .bind(username)
        .fetch_optional(conn)
        .await?;
        
        match row {
            Some(row) => {
                let user = User {
                    id: row.get("id"),
                    username: row.get("login"),
                    email: row.get("email"),
                    password: row.get("pass"),
                    firstname: None,
                    lastname: Some(row.get("name")),
                    status: 1, // Default active status
                    is_admin: row.get::<i32, _>("is_admin") != 0,
                    created: None,
                    last_login: None,
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    pub async fn find_by_email(db: &mut Connection<UniversalPathDb>, email: &str) -> Result<Option<User>> {
        let conn = db.get_conn();
        
        // Use a simpler query approach to avoid issues with NULL fields
        let row = sqlx::query(
            r#"
            SELECT 
                id, login, name, email, pass,
                CASE WHEN rights = 'A' THEN 1 ELSE 0 END as is_admin
            FROM users 
            WHERE email = ?
            "#
        )
        .bind(email)
        .fetch_optional(conn)
        .await?;
        
        match row {
            Some(row) => {
                let user = User {
                    id: row.get("id"),
                    username: row.get("login"),
                    email: row.get("email"),
                    password: row.get("pass"),
                    firstname: None,
                    lastname: Some(row.get("name")),
                    status: 1, // Default active status
                    is_admin: row.get::<i32, _>("is_admin") != 0,
                    created: None,
                    last_login: None,
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    pub async fn find_all(db: &mut Connection<UniversalPathDb>) -> Result<Vec<User>> {
        let conn = db.get_conn();
        
        // Use a simpler query approach to avoid issues with NULL fields
        let rows = sqlx::query(
            r#"
            SELECT 
                id, login, name, email, pass,
                CASE WHEN rights = 'A' THEN 1 ELSE 0 END as is_admin
            FROM users
            ORDER BY login
            "#
        )
        .fetch_all(conn)
        .await?;
        
        let users = rows
            .into_iter()
            .map(|row| {
                User {
                    id: row.get("id"),
                    username: row.get("login"),
                    email: row.get("email"),
                    password: row.get("pass"),
                    firstname: None,
                    lastname: Some(row.get("name")),
                    status: 1, // Default active status
                    is_admin: row.get::<i32, _>("is_admin") != 0,
                    created: None,
                    last_login: None,
                }
            })
            .collect();

        Ok(users)
    }

    pub async fn create(db: &mut Connection<UniversalPathDb>, new_user: NewUser) -> Result<u32> {
        let conn = db.get_conn();
        
        // Hash the password
        let hashed_password = hash(new_user.password, DEFAULT_COST)?;
        
        // Combine name from firstname and lastname
        let name = match (&new_user.firstname, &new_user.lastname) {
            (Some(first), Some(last)) => format!("{} {}", first, last),
            (Some(first), None) => first.clone(),
            (None, Some(last)) => last.clone(),
            (None, None) => new_user.username.clone(),
        };
        
        // Convert is_admin to the rights field
        let rights = if new_user.is_admin { "A" } else { "E" };

        let result = sqlx::query!(
            r#"
            INSERT INTO users 
            (login, name, pass, email, rights)
            VALUES
            (?, ?, ?, ?, ?)
            "#,
            new_user.username,
            name,
            hashed_password,
            new_user.email,
            rights
        )
        .execute(conn)
        .await?;

        Ok(result.last_insert_id() as u32)
    }

    pub async fn update(db: &mut Connection<UniversalPathDb>, update_user: UpdateUser) -> Result<bool> {
        let mut conn = db.get_conn();
        
        let mut query = String::from("UPDATE users SET");
        let mut params = Vec::new();
        let mut param_index = 1;

        // First, if firstname or lastname changed, update the name field
        let mut name_update = None;
        
        if update_user.firstname.is_some() || update_user.lastname.is_some() {
            // We need to do a separate query to get the current user data
            // to avoid the double borrow issue
            let current_user_result = sqlx::query(
                "SELECT name FROM users WHERE id = ?"
            )
            .bind(update_user.id)
            .fetch_optional(&mut *conn)
            .await?;
            
            if let Some(current_user_row) = current_user_result {
                let current_name: String = current_user_row.get("name");
                
                // Now work with the update data
                let firstname = update_user.firstname.as_deref().unwrap_or("");
                let lastname = update_user.lastname.as_deref().unwrap_or("");
                
                // If we have firstname and lastname, combine them
                // Otherwise, use the existing name
                if !firstname.is_empty() || !lastname.is_empty() {
                    let new_name = if !firstname.is_empty() && !lastname.is_empty() {
                        format!("{} {}", firstname, lastname)
                    } else if !firstname.is_empty() {
                        firstname.to_string()
                    } else {
                        lastname.to_string()
                    };
                    
                    name_update = Some(new_name);
                }
            }
        }
        
        if let Some(name) = name_update {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" name = ?");
            params.push(name);
            param_index += 1;
        }

        if let Some(email) = &update_user.email {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(" email = ?");
            params.push(email.clone());
            param_index += 1;
        }

        if let Some(password) = &update_user.password {
            if param_index > 1 {
                query.push_str(",");
            }
            let hashed_password = hash(password, DEFAULT_COST)?;
            query.push_str(" pass = ?");
            params.push(hashed_password);
            param_index += 1;
        }

        if let Some(is_admin) = &update_user.is_admin {
            if param_index > 1 {
                query.push_str(",");
            }
            let rights = if *is_admin { "A" } else { "E" };
            query.push_str(" rights = ?");
            params.push(rights.to_string());
            param_index += 1;
        }

        // If nothing to update, return early
        if param_index == 1 {
            return Ok(true);
        }

        // Add the WHERE clause
        query.push_str(" WHERE id = ?");
        params.push(update_user.id.to_string());

        // Execute the query
        let mut query_builder = sqlx::query(&query);
        for param in &params {
            query_builder = query_builder.bind(param);
        }

        let result = query_builder.execute(&mut *conn).await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<bool> {
        let conn = db.get_conn();
        
        let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(conn)
            .await?;
        
        Ok(result.rows_affected() > 0)
    }

    pub async fn verify_password(&self, password: &str) -> Result<bool> {
        // The original database stores passwords as MD5 hashes, not bcrypt
        // We need to adjust the verification method based on the password format
        if self.password.len() == 32 {
            // MD5 hash (original format from DB)
            let result = format!("{:x}", md5::compute(password));
            Ok(result == self.password)
        } else {
            // Bcrypt hash (new format)
            let verified = verify(password, &self.password)?;
            Ok(verified)
        }
    }

    pub async fn login(db: &mut Connection<UniversalPathDb>, login: &LoginUser) -> Result<Option<User>> {
        if let Some(user) = Self::find_by_username(db, &login.username).await? {
            if user.verify_password(&login.password).await? {
                // In the original database, there's no last_login field to update
                // But we can still return the user if authentication is successful
                return Ok(Some(user));
            }
        }

        Ok(None)
    }
}