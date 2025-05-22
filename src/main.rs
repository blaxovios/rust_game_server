use axum::{routing::get, Extension, Router};
use tower_http::{trace::TraceLayer};
use tower::ServiceBuilder;
use std::io;
// Import local modules
pub mod handlers;
pub mod utils;


#[derive(Clone)]
struct State {}


/// The entry point for the server, which sets up the server and starts it listening on port 3000.
/// It also sets up a global tracer for the server that logs
/// messages at the user given level in the config TOML file.
#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Instantiate server tracer
    utils::logger::server_tracer(utils::deserialize_toml_config::read_toml_config("configs/config.toml").tracing_config.level).await;

    let app = Router::new()
        .route("/get_welcome", get(handlers::progress_handler::get_welcome))
        .layer(
            ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())  // Trace the incoming requests
            .layer(Extension(State {}))
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}