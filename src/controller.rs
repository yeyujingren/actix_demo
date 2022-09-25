use std::sync::Mutex;

use actix_web::{App, HttpServer, web, guard, HttpResponse};
use home::{echo, manual_hello};
use config::{
	AppState,
	AppstateWithCounter
};

mod home;
mod config;
/// default demo
#[actix_web::main]
pub async fn instance() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home::index)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/// include namespace demo
#[actix_web::main]
pub async fn namespace_instance() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new().service(
			web::scope("/app")
				.route("/index", web::get().to(manual_hello))
		)
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}


/// state
/// share with all routes and resources within the same scope.
/// State is alse accessible for middleware
/// 
#[actix_web::main]
pub async fn state_instance() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.app_data(web::Data::new(AppState {
				app_name: String::from("Actix web")
			}))
			.service(config::index)
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}

/// mutable state
/// 
#[actix_web::main]
pub async fn mutable_state_instance() -> std::io::Result<()> {
	// must created outside HttpServer::new
	let counter = web::Data::new(AppstateWithCounter {
		counter: Mutex::new(0)
	});

	HttpServer::new(move || {
		App::new()
			.app_data(counter.clone())
			.service(config::cur_counter)
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}

// 
// The web::scope() method allows setting a resource group prefix. 
// This scope represents a resource prefix that will be prepended to all resource patterns added by the resource configuration. 
// 我愿意单方面成至为 “适配器”
// 
// #[actix_web::main]
// pub async fn adapter_instance() -> std::io::Result<()> {
// 	let scope = web::scope("/adapter").service(config::cur_counter);
// 	App::new().service(scope);
// }


/// a guard as a simple function that accepts a request object reference and returns true or false.
/// 
/// this demo is one of the guards: Header
#[actix_web::main]
pub async fn guard_instance() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.service(
				web::scope("/guard")
					.guard(guard::Header("Host", "www.rust-lang.org"))
					.route("", web::to(|| async {
						HttpResponse::Ok().body("www")
					}))
			)
			.service(
				web::scope("/guard")
				.guard(guard::Header("Host", "users.rust-lang.org"))
				.route("", web::to(|| async {
					HttpResponse::Ok().body("user")
				}))
			)
			.route("/guard", web::to(HttpResponse::Ok))
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}

/// For simplicity and reusability both App and web::Scope provide the configure method. 
/// This function is useful for moving parts of the configuration to a different module or even library. 
/// For example, some of the resource’s configuration could be moved to a different module.
#[actix_web::main]
pub async fn config_instace() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config::config)
            .service(web::scope("/api").configure(config::scoped_config))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}















