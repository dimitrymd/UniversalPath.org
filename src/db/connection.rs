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

// Helper trait to properly get an executor from the Rocket connection
pub trait ConnectionExt {
    fn get_conn(&mut self) -> &mut sqlx::MySqlConnection;
}

impl ConnectionExt for rocket_db_pools::Connection<UniversalPathDb> {
    fn get_conn(&mut self) -> &mut sqlx::MySqlConnection {
        // This is the correct way to access the underlying connection
        let pool_conn = &mut **self;
        // Now actually get the MySqlConnection out of the PoolConnection
        unsafe { 
            // This is a bit hacky but necessary to get to the underlying connection
            std::mem::transmute::<_, &mut sqlx::MySqlConnection>(pool_conn)
        }
    }
}