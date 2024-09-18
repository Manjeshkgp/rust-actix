use actix_web::{get, http::StatusCode, web::{Json, Path}, App, HttpServer, Responder};
use serde::Serialize;

#[get("/home")]
async fn home() -> impl Responder{
    let response: &str = "Welcome to actix web sever";
    response
}

#[get("/hello/{firstname}/{lastname}")]
async fn hello_user(params:Path<(String,String)>)-> impl Responder{
    let response =  User::new(params.0.clone(), params.1.clone());
    (Json(response),StatusCode::OK)
}

#[derive(Serialize)]

struct User {
    first_name:String,
    last_name:String
}

impl User {
    fn new(firstname:String,lastname:String)->Self{
        Self { first_name: firstname, last_name: lastname }
    }
}

#[actix_web::main]
async fn main()->std::io::Result<()> {
    let server = HttpServer::new(||{
        App::new()
        .service(home)
        .service(hello_user)
    }).bind(("127.0.0.1",8080))?.run();
    println!("Server running on PORT 127.0.0.1:8080");
    server.await
}
