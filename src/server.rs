use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use tokio::sync::broadcast::Sender;
use tracing::info;

use crate::handlers::{
    file_info_get_handler::file_info_get_handler,
    index::{
        css_get_handler::css_get_handler, html_get_handler::html_get_handler,
        javascript_get_handler::javascript_get_handler,
    },
    upload_post_handler::upload_post_handler,
};

const FILE_UPLOAD_SIZE_LIMIT_BYTES: usize = 1000000000;

/// starts the http server. panics on bad port.
/// if the port is zero, will broadcast the assigned port from the sender.
pub async fn start_server(port: &str, tx: Option<Sender<usize>>) {
    // TODO: add tracing in handlers
    let app = Router::new()
        .route("/", get(html_get_handler))
        .route("/index.js", get(javascript_get_handler))
        .route("/index.css", get(css_get_handler))
        .route("/api/ping", get(ping_get_handler))
        .route("/api/files/info", get(file_info_get_handler))
        .route("/api/upload", post(upload_post_handler))
        .layer(DefaultBodyLimit::max(FILE_UPLOAD_SIZE_LIMIT_BYTES));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:".to_string() + port)
        .await
        .expect("failed binding listener");

    let bind_address = listener
        .local_addr()
        .expect("failed getting listener address");
    info!("bound to {bind_address}");

    if let Some(tx) = tx {
        let _ = tx.send(
            bind_address
                .to_string()
                .split(":")
                .last()
                .expect("failed getting port number: couldn't find port in address")
                .parse()
                .expect("failed getting port number: bad parse to integer"),
        );
    }

    axum::serve(listener, app).await.unwrap();
}

#[axum::debug_handler]
async fn ping_get_handler() -> StatusCode {
    StatusCode::OK
}

#[cfg(test)]
mod server_tests {
    use axum::http::StatusCode;
    use tokio::sync::broadcast::channel;

    use super::start_server;

    #[tokio::test]
    async fn server_runs() {
        let (tx, mut rx) = channel::<usize>(16);
        tokio::task::spawn(start_server("0", Some(tx)));

        let port = rx.recv().await.expect("failed to get bound port");
        assert_eq!(
            reqwest::get(format!("http://localhost:{}/api/ping", port))
                .await
                .expect("failed to make request")
                .status(),
            StatusCode::OK.as_u16()
        );
    }
}
