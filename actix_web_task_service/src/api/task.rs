use actix_web::{
    HttpResponse,
    error::ResponseError,
    get,
    http::{StatusCode, header::ContentType},
    post, put,
    web::Data,
    web::Json,
    web::Path,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Deserialized, Serialize)]
pub struct TaskIdentifier {
    task_global_id: String,
}

#[get("/task/{task_global_id}")]
pub async fn get_task(task_identifier: Path<TaskIdentifier>) -> Json<String> {
    Json(task_identifier.into_inner().task_global_id)
}
