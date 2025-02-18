use axum::{Router, routing::get};


pub async fn get_progress() -> &'static str {
    "Hello, world!"
}