use actix_multipart::Multipart;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Result};
use backend_actix::filesave::upload_chunk;
use env_logger::Env;
use futures_util::StreamExt as _;
use sanitize_filename::sanitize;
use sqlx::PgPool;
use std::io::Write;

mod filesave;

use crate::filesave::upload_file;
use backend_actix::deadpoolc;
use backend_actix::sqlxc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // let pool = sqlxc::create_pool(&std::env::var("DATABASE_URL").unwrap())
    //     .await
    //     .expect("Failed to create pool");

    let pool = deadpoolc::create_pool().await;

    HttpServer::new(move || {
        App::new()
            //.wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
            .route("/upload", web::post().to(upload_file))
            .route("/upload_chunk", web::post().to(upload_chunk))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("vignesh")
}

// async fn upload_file(mut payload: Multipart) -> Result<HttpResponse> {
//     while let Some(item) = payload.next().await {
//         let mut field = item?;
//         let content_disposition = field.content_disposition();
//         let filename = content_disposition.get_filename().unwrap_or("upload.bin");
//         let filepath = format!("./uploads/{}",sanitize(&filename));
//         let mut f = std::fs::File::create(filepath)?;

//         while let Some(chunk) = field.next().await {
//             let data = chunk?;
//             f.write_all(&data)?;

//         }
//     }

//     Ok(HttpResponse::Ok().body("file uploaded"))
// }
