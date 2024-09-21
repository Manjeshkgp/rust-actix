use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, MySqlPool};

#[derive(Serialize, Deserialize)]
pub struct CreateNewTodo {
    title: String,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Todo {
    id: i32,
    title: String,
    description: Option<String>,
    status: String,
}

#[derive(Serialize)]
pub struct TypeDbError {
    error: String,
}

#[derive(Serialize, Deserialize)]

pub struct UpdateTitleStruct {
    id: i32,
    title: String,
}

#[derive(Serialize, Deserialize)]

pub struct UpdateDescriptionStruct {
    id: i32,
    description: String,
}

#[derive(Serialize, Deserialize)]

pub struct Id {
    id: i32,
}

#[post("/todos/mark/completed")]
async fn mark_status_completed(db: Data<MySqlPool>, body: Json<Id>) -> impl Responder {
    let res = sqlx::query("UPDATE todos SET status='Completed' WHERE id=?")
        .bind(body.id)
        .execute(&**db)
        .await;
    match res {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[post("/todos/description/update")]
async fn update_todo_description(
    db: Data<MySqlPool>,
    body: Json<UpdateDescriptionStruct>,
) -> impl Responder {
    let res = sqlx::query("UPDATE todos SET description=? WHERE id=?")
        .bind(&body.description)
        .bind(body.id)
        .execute(&**db)
        .await;
    match res {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[post("/todos/update")]
async fn update_todo_title(db: Data<MySqlPool>, body: Json<UpdateTitleStruct>) -> impl Responder {
    let res = sqlx::query("UPDATE todos SET title=? WHERE id=?")
        .bind(&body.title)
        .bind(body.id)
        .execute(&**db)
        .await;
    match res {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[post("/todos/create")]
pub async fn create_new_todo(db: Data<MySqlPool>, body: Json<CreateNewTodo>) -> impl Responder {
    let response = sqlx::query("INSERT INTO todos(title, description) values (?,?)")
        .bind(&body.title)
        .bind(&body.description)
        .execute(&**db)
        .await;
    match response {
        Ok(id) => HttpResponse::Created().json(Todo {
            id: id.last_insert_id() as i32,
            description: body.description.clone(),
            title: body.title.clone(),
            status: "New".to_string(),
        }),
        Err(_e) => HttpResponse::InternalServerError().json(TypeDbError {
            error: _e.to_string(),
        }),
    }
}

#[get("/todos/all")]
pub async fn get_all_todos(db: Data<MySqlPool>) -> impl Responder {
    let res: Result<Vec<Todo>, Error> =
        sqlx::query_as("select * from todos").fetch_all(&**db).await;
    match res {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_e) => HttpResponse::InternalServerError().json(TypeDbError {
            error: _e.to_string(),
        }),
    }
}
