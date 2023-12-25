use axum::{
    extract::{DefaultBodyLimit, Multipart},
    http::{Response, StatusCode},
    response::Html,
    routing::{get, post},
    Router,
};
use tokio::sync::broadcast::Sender;
use tracing::{error, info};

/// starts the http server. panics on bad port.
/// if the port is zero, will broadcast the assigned port from the sender.
pub async fn start_server(port: &str, tx: Option<Sender<usize>>) {
    let app = Router::new()
        .route("/", get(page_get_handler))
        .route("/api/ping", get(ping_get_handler))
        .route("/api/upload", post(upload_post_handler))
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
            bind_address
                .to_string()
                .split(":")
                .collect::<Vec<_>>()
                .get(1)
                .expect("failed getting port number: failed extracting port")
                .parse()
                .expect("failed getting port number: bad parse to integer"),
        );
    }

    axum::serve(listener, app).await.unwrap();
}

#[axum::debug_handler]
async fn page_get_handler() -> (Response<()>, Html<String>) {
    // FIXME: use include_str!() in a production environment
    let response;

    let html;
    let html_string = tokio::fs::read_to_string("src/frontend/index.html").await;

    match html_string {
        Ok(html_string) => {
            html = Html(html_string);
            response = Response::builder()
                .header("content-type", "text/html")
                .status(StatusCode::OK)
                .body(());
        }
        Err(file_error) => {
            error!("{file_error}");
            html = Html(include_str!("frontend/internal_server_error.html").to_string());
            response = Response::builder()
                .header("content-type", "text/html")
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(());
        }
    }

    (response.unwrap_or(Response::default()), html)
}

#[axum::debug_handler]
async fn ping_get_handler() -> StatusCode {
    StatusCode::OK
}

#[axum::debug_handler]
async fn upload_post_handler(mut multipart: Multipart) -> StatusCode {
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

    #[tokio::test]
    async fn serves_html() {
        let (tx, mut rx) = channel::<usize>(16);
        tokio::task::spawn(start_server("0", Some(tx)));

        let port = rx.recv().await.expect("failed to get bound port");
        let html = reqwest::get(format!("http://localhost:{}/", port))
            .await
            .expect("failed to make request")
            .text()
            .await
            .expect("failed to extract text");

        assert_eq!(html, include_str!("frontend/index.html"));
    }
}
