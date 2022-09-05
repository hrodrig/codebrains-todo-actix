use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

#[get("/todos")]
pub async fn list_todos() -> HttpResponse {
    HttpResponse::Ok().body("list")
}
#[post("/todos")]
pub async fn create_todo() -> HttpResponse {
    HttpResponse::Ok().body("create todo")
}

#[get("/todos/{id}")]
pub async fn get_todo() -> HttpResponse {
    HttpResponse::Ok().body("get todo")
}

#[put("/todos/{id}")]
pub async fn update_todo() -> HttpResponse {
    HttpResponse::Ok().body("update todo")
}

#[delete("/todos/{id}")]
pub async fn delete_todo() -> HttpResponse {
    HttpResponse::Ok().body("delete todo")
}