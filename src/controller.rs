use core::task;

use axum::{
	Json,
	extract::{Path, State},
	http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;

#[derive(Serialize)]
struct TaskRow {
	task_id: i32,
	name: String,
	priority: Option<i32>,
}

pub async fn get_tasks(State(pg_pool): State<PgPool>) -> Result<(StatusCode, String), (StatusCode, String)> {
	let rows = sqlx::query_as!(TaskRow, "SELECT * FROM tasks ORDER BY task_id")
		.fetch_all(&pg_pool)
		.await
		.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, json!({ "success": false, "message": e.to_string()}).to_string()))?;

	Ok((StatusCode::OK, json!({ "success": true, "data": rows }).to_string()))
}

#[derive(Deserialize)]
struct CreateTasksReq {
	name: String,
	priority: Option<i32>,
}

#[derive(Serialize)]
struct CreateTaskRow {
	task_id: i32,
	name: String,
	priority: Option<i32>,
}

pub async fn create_tasks(State(pg_pool): State<PgPool>, Json(task): Json<CreateTasksReq>) -> Result<(StatusCode, String), (StatusCode, String)> {
	let row = sqlx::query_as!(
		CreateTaskRow,
		"INSERT INTO tasks (name, priority) VALUES ($1, $2) RETURNING task_id, name, priority",
		task.name,
		task.priority
	)
	.fetch_one(&pg_pool)
	.await
	.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, json!({ "success": false, "message": e.to_string()}).to_string()))?;

	Ok((StatusCode::OK, json!({ "success": true, "data": row }).to_string()))
}

pub async fn get_task_by_id(State(pg_pool): State<PgPool>) -> Result<(StatusCode, String), (StatusCode, String)> {
	todo!()
}

#[derive(Deserialize)]
struct UpdateTaskReq {
	name: Option<String>,
	priority: Option<i32>,
}

pub async fn update_task_by_id(State(pg_pool): State<PgPool>, Path(task_id): Path<i32>, Json(payload): Json<UpdateTaskReq>) -> Result<(StatusCode, String), (StatusCode, String)> {
	let row = sqlx = sqlx::query!(
		"UPDATE tasks SET name = COALESCE($1, name), priority = COALESCE($2, priority) WHERE task_id = $3",
		payload.name,
		payload.priority,
		task_id
	)
	.execute(&pg_pool)
	.await
	.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, json!({ "success": false, "message": e.to_string()}).to_string()))?;

	Ok((StatusCode::OK, json!({ "success": true, "message": "Task updated successfully" }).to_string()))
}

pub async fn delete_task(State(pg_pool): State<PgPool>) -> Result<(StatusCode, String), (StatusCode, String)> {
	todo!()
}
