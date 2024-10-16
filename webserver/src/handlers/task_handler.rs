use crate::db::DbPool;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::services::task_service;

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    description: String,
    reward: i64,
}

pub fn task_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/tasks").service(create_task));
}

#[post("")]
pub async fn create_task(
    pool: web::Data<DbPool>,
    task: web::Json<CreateTaskRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection.");
    match task_service::create_task(&mut conn, &task.description, &task.reward).await {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => HttpResponse::InternalServerError().json("Error creating new task"),
    }
}

#[get("")]
pub async fn get_tasks(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection.");
    match task_service::get_tasks(&mut conn).await {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().json("Error getting tasks"),
    }
}
