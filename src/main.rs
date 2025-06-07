use actix_web::{web, App, HttpResponse, HttpServer, get};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/users")
                .service(show_users)
                .service(user_detail),
            )        
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/show")]
async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("show users")
}

#[get("/show/{id}")]
async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
}