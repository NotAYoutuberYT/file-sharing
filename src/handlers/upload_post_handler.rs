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
