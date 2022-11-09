use actix_web::{web, App, guard, HttpServer, http, HttpResponse, middleware};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod home;

#[actix_web::main]
pub async fn mult_use_instance() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.route("/", web::get().to(home::mult_use))
			.route("/user", web::post().to(home::mult_use))
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}


///
/// App::route() 提供了注册陆游的简单方法，但是要访问完整的资源配置，必须使用不同的方法（我的理解是需要在路由表中多次注册重复资源）
/// 此时，可以使用 App::service() 方法将单个资源添加到应用程序的路由表中。
/// 此方法解接受路径模式、守卫、一个或者多个路由
/// 

#[actix_web::main]
pub async fn service_instance() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.service(web::resource("/prefix").to(home::mult_use))
			.service(
				web::resource("/user/{name}")
					.name("user_detail")
					.guard(guard::Header("content-type", "application/json"))
					.route(web::get().to(HttpResponse::Ok))
					.route(web::put().to(HttpResponse::Ok))
			)
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}


///
/// 路由配置：一个路由可以包含多个 守卫（guards） 但是只能有一个处理程序（handler）
/// ```rust
/// App::new().service(
/// 	web::resource("/path").route(
/// 		web::route()
/// 			.guard(guard::Get())
/// 			.guard(guard::Header("content-type", "text/plain"))
/// 			.to(HttpResponse::Ok)
/// 	)
/// )
/// 
/// ```
/// 
/// 相对的，每个 route 定义以下方法
/// - Route::guard(): any number
/// - Route::method(): any number
/// - Route::to(): an async handler func for this route. Only one, usually handler registration is the last config operation
/// 
fn configurate_a_route() {
	// dothing
}


/// 
/// scoping routes
/// - /users
/// - /users/show
/// - /users/show/{id}
/// 
/// ==============
/// You can get variable path segments from HttpRequest::match_info(). 
/// Path extractor also is able to extract scope level variable segments.
/// 
/// 
#[actix_web::main]
pub async fn scope_route() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new().service(
			web::scope("/users")
				.service(home::show_users)
				.service(home::user_detail)
		)
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}


///
/// match information
/// 
#[actix_web::main]
pub async fn match_info_instance() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.service(home::match_all_infor)
	})
	.bind("127.0.0.1:7070")?
	.run()
	.await
}


// Generating resource URLs
#[actix_web::main]
pub async fn generate_ruls_instance() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.service(
				web::resource("/test/{a}/{b}/{c}")
					.name("foo")
					.guard(guard::Get())
					.to(HttpResponse::Ok)
			)
			.service(home::url_for_demo)
	})
	.bind("127.0.0.1:7070")?
	.run()
	.await
}


///
/// 路由格式化以及路由重定向
/// 一旦找到正确解析的路径，处理程序就会返回。如果全部启用，则规范化条件的顺序是：
/// 1、合并 2、合并和追加 3、追加
/// 如果路径至少满足这些条件之一，他将重定向到新的路径
/// 
#[actix_web::main]
pub async fn path_redirect_instance() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.wrap(middleware::NormalizePath::default())
			.route("/resource/", web::to(home::mult_use))
			.default_service(web::route().method(http::Method::GET))
	})
	.bind("127.0.0.1:7070")?
	.run()
	.await
}

///
/// 自定义路由守卫
/// 
#[actix_web::main]
pub async fn custom_guard_instance() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.route("/", web::route().guard(home::ContentTypeHeader).to(HttpResponse::Ok))
	})
	.bind("127.0.0.1:7070")?
	.run()
	.await
}

///
/// 重写默认路由——NOT FOUND
/// 你可以通过 App::default_service() 来重新 默认路由
/// 
#[actix_web::main]
pub async fn overwrite_default_route_instance() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.service(web::resource("/").route(web::get().to(home::mult_use)))
			.default_service(
				web::route()
					.guard(guard::Not(guard::Get()))
					.to(HttpResponse::MethodNotAllowed)
			)
	})
	.bind("127.0.0.1:7070")?
	.run()
	.await
}








