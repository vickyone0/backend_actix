use actix_web::{post,web, App, HttpResponse, HttpServer, Result, middleware::Logger};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .service(submit)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


async fn index() -> HttpResponse {
    HttpResponse::Ok().body("vignesh")
}

#[derive(Deserialize)]
struct Info {
    username: String,
}


#[post("/submit")]
async fn submit(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))

}