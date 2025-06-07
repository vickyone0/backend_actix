use actix_web::{web, App, HttpResponse, HttpServer, Result, middleware::Logger};
use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use std::io::Write;
use sanitize_filename::sanitize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/upload", web::post().to(upload_file))

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("vignesh")
}

async fn upload_file(mut payload: Multipart) -> Result<HttpResponse> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename().unwrap_or("upload.bin");
        let filepath = format!("./uploads/{}",sanitize(&filename));
        let mut f = std::fs::File::create(filepath)?;

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f.write_all(&data)?;
            
        }
    }

    Ok(HttpResponse::Ok().body("file uploaded"))
}


