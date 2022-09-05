mod api;
use actix_web::{ get, web::{self, Data}, App, HttpResponse, HttpServer,
    Responder, middleware
};
use api::todo_api::{create_todo, delete_todo, get_todo, list_todos, update_todo};
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    dotenv::dotenv().ok();
    env_logger::init();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    // establish connection to database and apply migrations
    // -> create post table if not exists
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let db_data = Data::new(conn);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(db_data.clone())
            .configure(init)
    })
        .bind(("localhost", 8000))?
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
