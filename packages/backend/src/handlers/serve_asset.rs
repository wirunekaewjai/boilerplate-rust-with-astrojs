use std::path::PathBuf;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use http::{StatusCode, header};

pub async fn serve_asset(
    State(directory): State<PathBuf>,
    Path(filename): Path<String>,
) -> Result<Response, StatusCode> {
    let Some(content) = get_content(directory, &filename).await else {
        return Err(StatusCode::NOT_FOUND);
    };

    let mime = mime_guess::from_path(&filename).first_or_octet_stream();
    let headers = [
        (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
        (header::CONTENT_TYPE, &mime.essence_str()),
    ];

    Ok((headers, content).into_response())
}

async fn get_content(directory: PathBuf, filename: &str) -> Option<Vec<u8>> {
    use path_clean::PathClean;
    use tokio::fs;

    let path = PathClean::clean(&directory.join(filename));

    if !path.starts_with(&directory) {
        return None;
    }

    let Ok(metadata) = fs::metadata(&path).await else {
        return None;
    };

    if !metadata.is_file() {
        return None;
    }

    let Ok(buffer) = fs::read(&path).await else {
        return None;
    };

    Some(buffer)
}
