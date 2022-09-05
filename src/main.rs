mod api;
use actix_web::{
    error, get, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder, Result,
};
use api::todo_api::{create_todo, delete_todo, get_todo, list_todos, update_todo};
use entity::todo;
use entity::todo::Entity as Todo;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use sea_orm::{entity::*, query::*};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    // establish connection to database and apply migrations
    // -> create post table if not exists
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(init)
    })
        .bind(("localhost", 8080))?
        .run()
        .await
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_todos);
    cfg.service(get_todo);
    cfg.service(create_todo);
    cfg.service(update_todo);
    cfg.service(delete_todo);
}
