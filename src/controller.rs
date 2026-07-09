use axum::{extract::State, http::StatusCode};
use sqlx::PgPool;

pub async fn get_tasks(State(pg_pool): State<PgPool>) -> Result<(StatusCode, String), (StatusCode, String)> {
	todo!()
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
