use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
	HttpResponse::Ok().body("hello actix!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
	HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
	HttpResponse::Ok().body("Hey there!")
}

