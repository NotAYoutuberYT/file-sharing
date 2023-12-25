use axum::{
    http::StatusCode,
    response::{Html, Response},
};
use tracing::error;

#[axum::debug_handler]
pub async fn page_get_handler() -> (Response<()>, Html<String>) {
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
            html = Html(include_str!("../frontend/internal_server_error.html").to_string());
            response = Response::builder()
                .header("content-type", "text/html")
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(());
        }
    }

    (response.unwrap_or(Response::default()), html)
}

#[cfg(test)]
mod page_get_handler_tests {
    use tokio::sync::broadcast::channel;

    use crate::server::start_server;

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

        assert_eq!(html, include_str!("../frontend/index.html"));
    }
}
