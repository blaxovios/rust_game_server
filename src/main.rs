// Entry point of your the web server

use axum::{response::Html, routing::get, Router};
// Import local module
pub mod handlers;
pub mod utils;

#[tokio::main]
async fn main() {
    // Instantiate server tracer
    utils::logger::server_tracer().await;

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(handlers::progress_handler::get_progress));
        // `POST /users` goes to `create_user`

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}