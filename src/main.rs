use actix_web::{guard,web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::resource("/path").route(
                web::route()
                    .guard(guard::Get())
                    .guard(guard::Header("content-type", "text/plain"))
                    .to(HttpResponse::Ok),
            ),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


async fn index() -> HttpResponse {
    HttpResponse::Ok().body("vignesh")
}