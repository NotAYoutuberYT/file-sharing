use axum::{routing::get, Router};
use tracing::{info, subscriber, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // FIXME: make lower max level for production environments
    let trace_subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    subscriber::set_global_default(trace_subscriber).expect("failed setting default subscriber");

    let app = Router::new().route("/", get(root));

    // FIXME: make a static port bind in a production environment
    let listener = tokio::net::TcpListener::bind("0.0.0.0:0")
        .await
        .expect("failed binding listener");

    let bind_address = listener
        .local_addr()
        .expect("failed getting listener address");
    info!("bound to {bind_address}");

    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
