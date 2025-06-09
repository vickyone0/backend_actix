use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::fs::{self, OpenOptions};
use std::io::Write;

#[derive(serde::Deserialize)]
pub struct ChunkInfo {
    file_id: String,
    chunk_number: u32,
    total_chunks: u32,
}

pub async fn upload_chunk(info: web::Query<ChunkInfo>, body: web::Bytes) -> impl Responder {
    let dir = "./uploads/chunks";
    fs::create_dir_all(dir).ok();

    let chunk_path = format!("{}/{}_{}", dir, info.file_id, info.chunk_number);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&chunk_path)
        .expect("Failed to open chunk file");

    file.write_all(&body).expect("Failed to write chunk");

    HttpResponse::Ok().body(format!(
        "Received chunk {} of {} for file {}",
        info.chunk_number, info.total_chunks, info.file_id
    ))
}

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
