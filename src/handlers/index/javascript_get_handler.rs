use axum::{http::StatusCode, response::Response};
use tracing::error;

#[axum::debug_handler]
pub async fn javascript_get_handler() -> Response<String> {
    // FIXME: use include_str!() in a production environment
    let response;

    let javascript = tokio::fs::read_to_string("src/frontend/index.js").await;

    match javascript {
        Ok(javascript) => {
            response = Response::builder()
                .header("content-type", "application/javascript;charset=utf-8")
                .status(StatusCode::OK)
                .body(javascript);
        }
        Err(file_error) => {
            error!("{file_error}");
            response = Response::builder()
                .header("content-type", "application/javascript;charset=utf-8")
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(String::default());
        }
    }

    response.unwrap_or(Response::default())
}

#[cfg(test)]
mod javascript_get_handler_tests {
    use tokio::sync::broadcast::channel;

    use crate::server::start_server;

    #[tokio::test]
    async fn serves_javascript() {
        let (tx, mut rx) = channel::<usize>(16);
        tokio::task::spawn(start_server("0", Some(tx)));

        let port = rx.recv().await.expect("failed to get bound port");
        let javascript = reqwest::get(format!("http://localhost:{}/index.js", port))
            .await
            .expect("failed to make request")
            .text()
            .await
            .expect("failed to extract text");

        assert_eq!(javascript, include_str!("../../frontend/index.js"));
    }
}
