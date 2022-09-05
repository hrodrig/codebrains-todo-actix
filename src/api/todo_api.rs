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
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TodoRequest {
    title: String,
    completed: bool,
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
pub async fn create_todo(data: web::Data<AppState>, new_todo: Json<TodoRequest>) -> HttpResponse {
    let conn = &data.conn;

    let todo = todo::ActiveModel {
        id: NotSet,
        title: ActiveValue::Set(new_todo.title.to_owned()),
        completed: Set(new_todo.completed.to_owned()),
    };

    let todo: todo::Model = todo.insert(conn).await.unwrap();
    HttpResponse::Ok().json(todo)
}

#[put("/todos/{id}")]
pub async fn update_todo(data: web::Data<AppState>, id: web::Path<i32>, update_todo: Json<TodoRequest>) -> HttpResponse {
    let conn = &data.conn;

    let mut todo: todo::ActiveModel = Todo::find_by_id(id.into_inner()).one(conn).await.unwrap().unwrap().into();

    todo.title = ActiveValue::Set(update_todo.title.to_owned());
    todo.completed = ActiveValue::Set(update_todo.completed.to_owned());
    let todo = todo.update(conn).await.unwrap();
    HttpResponse::Ok().json(todo)
}

#[delete("/todos/{id}")]
pub async fn delete_todo(data: web::Data<AppState>, id: web::Path<i32>) -> HttpResponse {
    let conn = &data.conn;

    let todo: todo::ActiveModel = Todo::find_by_id(id.into_inner()).one(conn).await.unwrap().unwrap().into();
    
    todo.delete(conn).await.unwrap();
    HttpResponse::Ok().body("update todo")
}