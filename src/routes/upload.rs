use crate::services::upload::{extract_filename, get_extension, save_file};
use axum::{extract::Multipart, http::StatusCode};
use std::path::Path;

pub async fn upload(mut form: Multipart) -> Result<(), (StatusCode, String)> {
    // Grab Fields
    while let Some(field) = form
        .next_field()
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
    {
        // Check file is found
        if field.name() == Some("file") {
            tracing::info!("File Received");

            // Get file name
            let filename = extract_filename(&field)?; // File path
            let filepath = Path::new("../../uploads/").join(&filename); // File path
            let _file_extension = get_extension(&filepath)?; // Get extension

            // Save file
            save_file(field, &filepath).await?;

            tracing::info!("File {} Processed", filename);

            // Allows only one file read
            return Ok(());
        }
    }
    tracing::info!("No File Found");
    Err((StatusCode::BAD_REQUEST, "No File Given".to_string()))
}
