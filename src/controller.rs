use actix_web::{web, App, HttpServer};

mod home;

#[actix_web::main]
pub async fn format_info_instance() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new().route("/json", web::post().to(home::format_info))
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}


