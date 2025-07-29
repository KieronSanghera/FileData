use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use axum::extract::multipart::Field;
use axum::http::StatusCode;
use tokio::{fs::File, io::AsyncWriteExt};

fn check_filename(filename: String) -> Result<String, (StatusCode, String)> {
    let trimmed = filename.trim();
    if trimmed.is_empty()
        || trimmed.len() > 255
        || trimmed.starts_with("..")
        || trimmed.contains('/')
        || trimmed.contains('\\')
        || trimmed.contains("..")
        || trimmed
            .chars()
            .any(|c| !(c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-'))
    {
        tracing::info!("Invalid Filename");
        return Err((StatusCode::BAD_REQUEST, "Invalid filename".to_string()));
    }
    Ok(trimmed.to_string())
}

pub fn get_extension(filepath: &Path) -> Result<String, (StatusCode, String)> {
    let extension = filepath
        .extension()
        .and_then(OsStr::to_str)
        .ok_or((StatusCode::BAD_REQUEST, "Invalid filename".to_string()))?;

    Ok(extension.to_owned())
}

pub fn extract_filename(data: &Field) -> Result<String, (StatusCode, String)> {
    check_filename(
        data.file_name()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.to_string())
            .ok_or((
                StatusCode::BAD_REQUEST,
                "Missing or Empty Filename".to_string(),
            ))?,
    )
}

pub async fn save_file(
    mut data: Field<'_>,
    filepath: &PathBuf,
) -> Result<(), (StatusCode, String)> {
    // Create File
    let mut file = File::create(&filepath)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    // Save data to file
    while let Some(chunk) = data
        .chunk()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
    {
        file.write_all(chunk.as_ref())
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    }
    Ok(())
}
