mod handlers;
mod server;

use tracing::{subscriber, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // FIXME: make the max level customizable somewhere
    let trace_subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    subscriber::set_global_default(trace_subscriber).expect("failed setting default subscriber");

    // FIXME: use known port in production environment
    server::start_server("0", None).await;
}
