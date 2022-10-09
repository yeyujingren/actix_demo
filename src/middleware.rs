pub mod logger;
use actix_web::{web, App, middleware::{Logger, self}, HttpServer, Responder, http::KeepAlive, HttpResponse};

#[rustfmt::skip]
#[actix_web::main]
pub async fn logger_instance() -> std::io::Result<()> {
	std::env::set_var("RUST_LOG", "info");
	std::env::set_var("RUST_BACKTRACE", "1");
	env_logger::init();
	HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(logger::info_logger)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
