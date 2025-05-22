use axum::{routing::get, Router};
use std::io;
// Import local module
pub mod handlers;
pub mod utils;


/// The entry point for the server.
///
/// This function sets up the server and starts it listening on port 3000.
/// It also sets up a global tracing subscriber for the server that logs
/// messages at the `DEBUG` level.
#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Instantiate server tracer
    utils::logger::server_tracer(utils::deserialize_toml_config::read_toml_config("configs/config.toml").tracing_config.level).await;

    let app = Router::new()
        // `GET /` goes to `handlers::progress_handler::get_progress`
        .route("/get_progress", get(handlers::progress_handler::get_progress));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}