use crate::entity::todo::Model as Todo;
use crate::AppState;
use actix_web::{
    get, post, web, web::Json, Error as ActixError, Responder, Result as ActixResult, Scope,
};
use log::{debug, error, info, log_enabled, Level};

#[get("")]
async fn get_todos(state: web::Data<AppState>) -> ActixResult<impl Responder, ActixError> {
    let todos = state.todo_repository.get_todos().await;
    debug!("this is a debug {}", todos[0].title);
    Ok(web::Json(todos))
}

#[get("/{id}")]
async fn get_todo_by_id(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> ActixResult<impl Responder, ActixError> {
    let todo = state.todo_repository.get_todo_by_id(id.into_inner()).await;
    Ok(web::Json(todo))
}

#[post("")]
async fn create_todo(
    state: web::Data<AppState>,
    new_todo: Json<crate::repository::todos::TodoRequest>,
) -> ActixResult<impl Responder, ActixError> {
    let todo: Option<Todo> = state.todo_repository.create_todo(new_todo).await;
    Ok(web::Json(todo))
}

pub fn todos_handler() -> Scope {
    web::scope("/todos")
        .service(get_todos)
        .service(get_todo_by_id)
        .service(create_todo)
}
