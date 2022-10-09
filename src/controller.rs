use actix_web::{web, App, HttpRequest, HttpServer, Responder, http::KeepAlive, HttpResponse};
mod error;
mod error_handling;

#[actix_web::main]
pub async fn error_instance() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/error", web::get().to(error::custom_error))
            .service(error::response_error)
            .service(error_handling::error_handling)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
