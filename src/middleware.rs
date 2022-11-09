mod index;
use actix_web::{dev::Server as _, web, App};
use futures_util::future::FutureExt;

#[actix_web::main]
pub async fn middleware_instance() {
    let app = App::new()
        .wrap_fn(|req, srv| {
            println!("Hi from start. You requested: {}", req.path());
            srv.call(req).map(|res| {
                println!("Hi from response");
                res
            })
        })
        .route(
            "/index.html",
            web::get().to(|| async { "Hello, middleware!" }),
        );
}
