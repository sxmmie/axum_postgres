use axum::{extract::State, http::StatusCode};
use sqlx::PgPool;

async fn get_tasks(State(pg_pool): State<PgPool>) -> Result<(StatusCode, String), (StatusCode, String)> {
	todo!()
}

async fn create_tasks(State(pg_pool): State<PgPool>) -> Result<(StatusCode, String), (StatusCode, String)> {
	todo!()
}

async fn get_task_by_id(State(pg_pool): State<PgPool>) -> Result<(StatusCode, String), (StatusCode, String)> {}

async fn update_task_by_id(State(pg_pool): State<PgPool>) -> Result<(StatusCode, String), (StatusCode, String)> {
	todo!()
}
