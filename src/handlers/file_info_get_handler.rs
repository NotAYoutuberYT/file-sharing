use axum::{
    http::{Response, StatusCode},
    Json,
};
use tracing::error;
#[axum::debug_handler]
pub async fn file_info_get_handler() -> (Response<()>, Json<Vec<String>>) {
    let response;
    let json;
    let file_paths = tokio::fs::read_dir("file_store").await; // TODO: make this customizable

    match file_paths {
        Ok(mut file_paths) => {
            let mut file_names: Vec<String> = vec![];
            while let Ok(Some(file_directory_entry)) = file_paths.next_entry().await {
                file_names.push(
                    file_directory_entry
                        .file_name()
                        .to_str()
                        .unwrap_or("invalid utf name")
                        .to_string(),
                );
            }

            response = Response::builder().status(StatusCode::OK).body(());
            json = Json(file_names);
        }
        Err(file_error) => {
            error!("{file_error}");
            response = Response::builder()
                .header("content-type", "text/css;charset=utf-8")
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(());
            json = Json::default();
        }
    }

    (response.unwrap_or(Response::default()), json)
}
