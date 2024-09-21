use actix_web::{App, HttpServer, web::Data};
use dotenv::dotenv;

mod routes;
use routes::*;

mod database;
use database::*;
use sqlx::Executor;

#[tokio::main]
async fn main()->std::io::Result<()> {
    dotenv().ok();
    // running this will allow to load variables from .env file
    let database: sqlx::Pool<sqlx::MySql> = database_connection().await.expect("Failed to connect to the database");
    println!("Database connected");
    let create_table_sql = r#"
    CREATE TABLE IF NOT EXISTS todos (
        id INT AUTO_INCREMENT PRIMARY KEY,
        title VARCHAR(100) NOT NULL,
        description VARCHAR(100) NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )
    "#;
    let _ = database.execute(create_table_sql).await;
    let server = HttpServer::new(move||{
        App::new()
        .app_data(Data::new(database.clone()))
        .service(home)
        .service(hello_user)
        .service(create_new_user)
        .service(create_new_todo)
        .service(get_all_todos)
    }).bind(("127.0.0.1",8080))?.run();
    println!("Server running on PORT 127.0.0.1:8080");
    server.await
}
