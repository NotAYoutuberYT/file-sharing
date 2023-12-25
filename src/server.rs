use axum::{
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use tokio::sync::broadcast::Sender;
use tracing::info;

/// starts the http server. panics on bad port.
/// if the port is zero, will broadcast the assigned port from the sender.
pub async fn start_server(port: &str, tx: Option<Sender<usize>>) {
    let app = Router::new()
        .route("/api/ping", get(ping_get))
        .route("/api/upload", post(upload_post))
        .layer(DefaultBodyLimit::max(1000000000));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:".to_string() + port)
        .await
        .expect("failed binding listener");

    let bind_address = listener
        .local_addr()
        .expect("failed getting listener address");
    info!("bound to {bind_address}");

    if let Some(tx) = tx {
        let _ = tx.send(
            bind_address.to_string().split(":").collect::<Vec<_>>()[1]
                .parse()
                .expect("failed getting port number"),
        );
    }

    axum::serve(listener, app).await.unwrap();
}

async fn ping_get() -> StatusCode {
    StatusCode::OK
}

async fn upload_post(mut multipart: Multipart) -> StatusCode {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }

    StatusCode::ACCEPTED
}

#[cfg(test)]
mod server_tests {
    use axum::http::StatusCode;
    use tokio::sync::broadcast::channel;

    use super::start_server;

    #[tokio::test]
    async fn server_response() {
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
