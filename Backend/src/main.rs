mod api;
mod question_list;

use api::*;
use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use question_list::{QuestionList, Question};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Initialize the PostgreSQL connection pool
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("PG_USER").expect("PG_USER not set"),
        env::var("PG_PASSWORD").expect("PG_PASSWORD not set"),
        env::var("PG_HOST").expect("PG_HOST not set"),
        "5432",
        env::var("PG_DBNAME").expect("PG_DBNAME not set")
    );

    println!("Connecting to the database at {}", database_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    println!("Connected to the database");

    // Initialize the question list with the PostgreSQL pool
    let question_list = QuestionList::new(pool.clone()).await;
    let shared_question_list = Arc::new(question_list);

    // Configure the Axum router with routes 
    let app = Router::new()
        .route("/questions", get(fetch_all_questions).post(create_question))
        .route("/questions/:id", get(fetch_question).put(update_question).delete(remove_question))
        .layer(Extension(shared_question_list));

    let address = SocketAddr::from(([0, 0, 0, 0], 3000)); // Listen on all interfaces

    println!("Starting server on {}", address);

    // Run the Axum server
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn fallback_not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Endpoint does not exist")
}
