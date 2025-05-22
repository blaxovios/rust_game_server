use tracing::Level;
use tracing_subscriber::FmtSubscriber;


/// Sets a global tracing subscriber for the server that logs messages at the specified level.
pub async fn server_tracer (level: Level) -> () {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}