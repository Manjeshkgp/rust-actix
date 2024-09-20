use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod routes;
use routes::*;

mod database;
use database::*;

#[tokio::main]
async fn main()->std::io::Result<()> {
    dotenv().ok();
    // running this will allow to load variables from .env file
    let database: sqlx::Pool<sqlx::MySql> = database_connection().await.expect("Failed to connect to the database");
    println!("Database connected");
    let server = HttpServer::new(move||{
        App::new()
        .app_data(database.clone())
        .service(home)
        .service(hello_user)
        .service(create_new_user)
    }).bind(("127.0.0.1",8080))?.run();
    println!("Server running on PORT 127.0.0.1:8080");
    server.await
}
