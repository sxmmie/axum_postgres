use axum::{extract::State, http::StatusCode};
use serde::Serialize;
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
		.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, json!({ "success": false, "message": e.to_string()}).to_string()))?;

	Ok((StatusCode::OK, json!({ "success": true, "data": rows }).to_string()))
}

pub async fn create_tasks(State(pg_pool): State<PgPool>) -> Result<(StatusCode, String), (StatusCode, String)> {
	todo!()
}

pub async fn get_task_by_id(State(pg_pool): State<PgPool>) -> Result<(StatusCode, String), (StatusCode, String)> {
	todo!()
}

pub async fn update_task_by_id(State(pg_pool): State<PgPool>) -> Result<(StatusCode, String), (StatusCode, String)> {
	todo!()
}

pub async fn delete_task(State(pg_pool): State<PgPool>) -> Result<(StatusCode, String), (StatusCode, String)> {
	todo!()
}
