use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    web,
    HttpResponse,
};

use sea_orm::{ActiveValue::NotSet, DeleteResult};
use entity::todo;
use entity::todo::Entity as Todo;
use sea_orm::DatabaseConnection;
use sea_orm::{entity::*, query::*};
use std::env;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[get("/todos")]
pub async fn list_todos(data: web::Data<AppState>) -> HttpResponse {
    let conn = &data.conn;

    let todos = Todo::find()
        .all(conn)
        .await
        .unwrap();

    HttpResponse::Ok().json(todos)
}

#[get("/todos/{id}")]
pub async fn get_todo(data: web::Data<AppState>, id: web::Path<i32>) -> HttpResponse {
    let conn = &data.conn;

    let todo: Option<todo::Model> = Todo::find_by_id(id.into_inner()).one(conn).await.unwrap();
    HttpResponse::Ok().json(todo)
}

#[post("/todos")]
pub async fn create_todo(data: web::Data<AppState>, new_todo: web::Json<Todo>) -> HttpResponse {
    let conn = &data.conn;

    let todo = todo::ActiveModel {
        id: NotSet,
        title: Set(new_todo.title.to_owned()),
        completed: Set(new_todo.completed.to_owned()),
    };

    let todo: todo::Model = todo.insert(conn).await.unwrap();
    HttpResponse::Ok().json(todo)
}

#[put("/todos/{id}")]
pub async fn update_todo(data: web::Data<AppState>, id: web::Path<i32>, new_todo: Json<Todo>) -> HttpResponse {
    let conn = &data.conn;

    HttpResponse::Ok().body("update todo")
}

#[delete("/todos/{id}")]
pub async fn delete_todo(data: web::Data<AppState>, id: web::Path<i32>) -> HttpResponse {
    let conn = &data.conn;

    let todo: todo::ActiveModel = Todo::find_by_id(id.into_inner()).one(conn).await.unwrap().unwrap().into();
    
    todo.delete(conn).await.unwrap();
    HttpResponse::Ok().body("update todo")
}