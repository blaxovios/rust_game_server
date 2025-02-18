use axum::{Router, routing::get};


pub async fn get_progress() -> &'static str {
    "You came to my game server!"
}