use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::{sqlx, Connection};
use crate::db::UniversalPathDb;
use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};

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
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, username, firstname, lastname, email, password, 
                status, is_admin, created, last_login
            FROM users 
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&mut **db)
        .await?;

        Ok(user)
    }

    pub async fn find_by_username(db: &mut Connection<UniversalPathDb>, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, username, firstname, lastname, email, password, 
                status, is_admin, created, last_login
            FROM users 
            WHERE username = ?
            "#,
            username
        )
        .fetch_optional(&mut **db)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(db: &mut Connection<UniversalPathDb>, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, username, firstname, lastname, email, password, 
                status, is_admin, created, last_login
            FROM users 
            WHERE email = ?
            "#,
            email
        )
        .fetch_optional(&mut **db)
        .await?;

        Ok(user)
    }

    pub async fn find_all(db: &mut Connection<UniversalPathDb>) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, username, firstname, lastname, email, password, 
                status, is_admin, created, last_login
            FROM users 
            ORDER BY username
            "#
        )
        .fetch_all(&mut **db)
        .await?;

        Ok(users)
    }

    pub async fn create(db: &mut Connection<UniversalPathDb>, new_user: NewUser) -> Result<u32> {
        // Hash the password
        let hashed_password = hash(new_user.password, DEFAULT_COST)?;

        let result = sqlx::query!(
            r#"
            INSERT INTO users 
            (username, firstname, lastname, email, password, status, is_admin, created)
            VALUES
            (?, ?, ?, ?, ?, 1, ?, NOW())
            "#,
            new_user.username,
            new_user.firstname,
            new_user.lastname,
            new_user.email,
            hashed_password,
            new_user.is_admin
        )
        .execute(&mut **db)
        .await?;

        Ok(result.last_insert_id() as u32)
    }

    pub async fn update(db: &mut Connection<UniversalPathDb>, update_user: UpdateUser) -> Result<bool> {
        let mut query = String::from("UPDATE users SET");
        let mut params = Vec::new();
        let mut param_index = 1;

        if let Some(firstname) = &update_user.firstname {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" firstname = ?"));
            params.push(firstname.clone());
            param_index += 1;
        }

        if let Some(lastname) = &update_user.lastname {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" lastname = ?"));
            params.push(lastname.clone());
            param_index += 1;
        }

        if let Some(email) = &update_user.email {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" email = ?"));
            params.push(email.clone());
            param_index += 1;
        }

        if let Some(password) = &update_user.password {
            if param_index > 1 {
                query.push_str(",");
            }
            let hashed_password = hash(password, DEFAULT_COST)?;
            query.push_str(&format!(" password = ?"));
            params.push(hashed_password);
            param_index += 1;
        }

        if let Some(status) = &update_user.status {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" status = ?"));
            params.push(status.to_string());
            param_index += 1;
        }

        if let Some(is_admin) = &update_user.is_admin {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" is_admin = ?"));
            params.push(is_admin.to_string());
            param_index += 1;
        }

        // If nothing to update, return early
        if param_index == 1 {
            return Ok(true);
        }

        // Add the WHERE clause
        query.push_str(&format!(" WHERE id = ?"));
        params.push(update_user.id.to_string());

        // Use sqlx::query to execute the dynamic query
        let mut query_builder = sqlx::query(&query);
        for param in &params {
            query_builder = query_builder.bind(param);
        }

        // Fix: Cast database connection to the correct type
        let conn = &mut **db as &mut sqlx::MySqlConnection;
        let result = query_builder.execute(conn).await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(&mut **db)
            .await?;
        
        Ok(result.rows_affected() > 0)
    }

    pub async fn update_last_login(db: &mut Connection<UniversalPathDb>, id: u32) -> Result<bool> {
        let result = sqlx::query!("UPDATE users SET last_login = NOW() WHERE id = ?", id)
            .execute(&mut **db)
            .await?;
        
        Ok(result.rows_affected() > 0)
    }

    pub async fn verify_password(&self, password: &str) -> Result<bool> {
        let verified = verify(password, &self.password)?;
        Ok(verified)
    }

    pub async fn login(db: &mut Connection<UniversalPathDb>, login: &LoginUser) -> Result<Option<User>> {
        if let Some(user) = Self::find_by_username(db, &login.username).await? {
            if user.verify_password(&login.password).await? {
                // Update last login time
                Self::update_last_login(db, user.id).await?;
                return Ok(Some(user));
            }
        }

        Ok(None)
    }
}