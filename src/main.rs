use axum::{
	Router,
	routing::{delete, get, patch},
};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use crate::controller::{create_tasks, delete_task, get_task_by_id, get_tasks, update_task_by_id};

mod controller;

#[tokio::main]
async fn main() {
	// expose env variables
	// dotenvy::dotenv() returns a Result enum if it can't find the .env file, we use expect() to handle the error and print a message if it fails
	dotenvy::dotenv().expect("Unable to load the .env file");

	// set environment variables from .env file
	let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1".to_owned()); // assign a fallback value if the env variable is not set
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"); // stops the program if the env variable is not set

	// create database connection pool
	// initialize poolOption struct with database_url and max_connections
	let db_pool = PgPoolOptions::new()
		.max_connections(15)
		.connect(&database_url)
		.await // connection asynchronously to the database, returns a Result enum, we use expect() to handle the error and print a message if it fails
		.expect("Failed to connect to the database");

	// create TCP listener
	let listener = TcpListener::bind(&server_address).await.expect("Failed to bind to address");
	println!("Server running on {}", listener.local_addr().unwrap()); // if successful, print the server address to the console

	// compose the routes
	let app = Router::new()
		.route("/", get(|| async { "Hello, World!" }))
		.route("/tasks", get(get_tasks))
		.route("/tasks", delete(delete_task))
		.route("/tasks/:id", get(get_task_by_id))
		.route("/tasks/:id", patch(update_task_by_id))
		.with_state(db_pool); // pass the database connection pool to the app state, so each route handler can access it

	// run/serve server
	axum::serve(listener, app).await.expect("Failed to start the server");
}
