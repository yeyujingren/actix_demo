use actix_web::{web, App, HttpServer, middleware};

mod home;

///
/// 用于生成 HttpResponse 实例 的构造函数。HttpResponse 提供了多个可以返回 HttpResponseBuilder 的实例
/// 方法 .body, .finish, 和 .json 生成相应，并且返回一个 HttpResponse 实例
/// 这些方法只能在同一个 绑定器（builder） 上被调用一次，否则会出现 panic
/// 
/// ```rust
/// async fn index() -> HttpResponse {
///   HttpResponse::Ok()
///     .content_type(ContentType::plaintext())
///     .insert_header(("X-Hdr", "sample"))
///     .body("data")
/// }
/// ```
/// 
fn _demo_one() {}

#[actix_web::main]
pub async fn res_json_instance() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new().service(home::res_json)
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}


// 使用 brotli 压缩响应体
#[actix_web::main]
pub async fn compress_res_by_brotli_instance() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Compress::default())
      .service(home::index_br)
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}


#[actix_web::main]
async fn identity_compress_instance() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(home::index_ident)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[actix_web::main]
pub async fn gzip_compress_instance() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(home::gzip_compress)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}




