use sqlx::MySqlPool;
use std::env::var;

pub async fn database_connection() -> Result<MySqlPool, sqlx::Error>{
    let db_url: String = var("DB_URL").unwrap();
    MySqlPool::connect(&db_url).await
}