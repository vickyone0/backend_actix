use actix_web::{post, web, App, HttpServer, Responder};
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Metadata {
    name: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "10MB")] // Limit file size to 10MB
    file: TempFile,
    json: actix_multipart::form::json::Json<Metadata>,
}

pub async fn upload_file(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    
    let file_path = form.file.file.path();
    let file_bytes = fs::read(file_path).expect("Failed to read uploaded file");


    format!(
        "Uploaded file for {}, size: {} bytes, first 10 bytes: {:?}",
        form.json.name,
        file_bytes.len(),
        &file_bytes[..file_bytes.len().min(10)]
    )
}

// To use this handler, add it to your App in main.rs:
// .service(upload_file)