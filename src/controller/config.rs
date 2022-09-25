use actix_web::{get, web::{self, ServiceConfig}, HttpResponse};
use std::sync::Mutex;

// common state
pub struct AppState {
    pub app_name: String,
}

// shared mutable state
pub struct AppstateWithCounter {
	pub counter: Mutex<i32>
}

#[get("/")]
pub async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("hello {app_name}")
}

#[get("/counter")]
pub async fn cur_counter(data: web::Data<AppstateWithCounter>) -> String {
	let mut counter = data.counter.lock().unwrap();
	*counter += 1;
	format!("this page has interview {counter} times")
}

// this function could be located in a different module
pub fn scoped_config(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::resource("/test")
			.route(web::get().to(|| async {
				HttpResponse::Ok().body("test")
			}))
			.route(web::head().to(HttpResponse::MethodNotAllowed))
	);
}

// this function could be located in a different module
pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
