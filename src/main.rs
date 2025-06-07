use actix_web::{web, App, HttpResponse, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/usser", web::post().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


async fn index() -> HttpResponse {
    HttpResponse::Ok().body("vignesh")
}