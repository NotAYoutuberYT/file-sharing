use axum::{extract::Multipart, http::StatusCode};

#[axum::debug_handler]
pub async fn upload_post_handler(mut multipart: Multipart) -> StatusCode {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }

    StatusCode::ACCEPTED
}
