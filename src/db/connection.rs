use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("universal_path_db")]
pub struct UniversalPathDb(sqlx::MySqlPool);

pub async fn init_pool(database_url: &str) -> Result<sqlx::MySqlPool, sqlx::Error> {
    sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}