use axum::{extract::Multipart, http::StatusCode};
use tracing::error;

#[axum::debug_handler]
pub async fn upload_post_handler(mut multipart: Multipart) -> StatusCode {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        if let Err(write_error) = tokio::fs::write(format!("file_store/{name}"), data).await {
            error!("{write_error}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }

    StatusCode::ACCEPTED
}

#[cfg(test)]
mod upload_post_handler_tests {
    use axum::http::StatusCode;
    use reqwest::multipart::{Form, Part};
    use tokio::sync::broadcast::channel;

    use crate::server::start_server;

    #[tokio::test]
    async fn uploads_files_one() {
        let (tx, mut rx) = channel::<usize>(16);
        tokio::task::spawn(start_server("0", Some(tx)));
        let port_future = rx.recv();

        const FILE_NAME: &str = "upload_test_sample_file_one.txt";
        const FILE_TEXT: &str = "this is a sample file for an upload test.";

        let part = Part::text(FILE_TEXT)
            .file_name(FILE_TEXT)
            .mime_str("text/plain")
            .expect("failed to create part");
        let form = Form::new().part(FILE_NAME, part);

        let port = port_future.await.expect("failed to get bound port");
        let response = reqwest::Client::new()
            .post(format!("http://localhost:{}/api/upload", port))
            .multipart(form)
            .send()
            .await
            .expect("failed to make request");

        assert_eq!(response.status(), StatusCode::ACCEPTED.as_u16());

        let file_contents = tokio::fs::read(format!("file_store/{FILE_NAME}")).await.expect("failed to read from upload test file");
        assert_eq!(file_contents, FILE_TEXT.as_bytes());

        tokio::fs::remove_file(format!("file_store/{FILE_NAME}")).await.expect("failed to remove upload test file");
    }

    #[tokio::test]
    async fn uploads_files_two() {
        let (tx, mut rx) = channel::<usize>(16);
        tokio::task::spawn(start_server("0", Some(tx)));
        let port_future = rx.recv();

        const FILE_NAME: &str = "upload_test_sample_file_two.rs";
        const FILE_TEXT: &str = "fn main() { println!(\"this is a sample file for an upload test\"); }";

        let part = Part::text(FILE_TEXT)
            .file_name(FILE_TEXT)
            .mime_str("text/plain")
            .expect("failed to create part");
        let form = Form::new().part(FILE_NAME, part);

        let port = port_future.await.expect("failed to get bound port");
        let response = reqwest::Client::new()
            .post(format!("http://localhost:{}/api/upload", port))
            .multipart(form)
            .send()
            .await
            .expect("failed to make request");

        assert_eq!(response.status(), StatusCode::ACCEPTED.as_u16());

        let file_contents = tokio::fs::read(format!("file_store/{FILE_NAME}")).await.expect("failed to read from upload test file");
        assert_eq!(file_contents, FILE_TEXT.as_bytes());

        tokio::fs::remove_file(format!("file_store/{FILE_NAME}")).await.expect("failed to remove upload test file");
    }
}
