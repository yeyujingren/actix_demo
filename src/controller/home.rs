use actix_web::{
	get, HttpRequest, Responder, body::BoxBody, http::{header::ContentType, Error}, HttpResponse, web
};
use serde::Serialize;

#[get("/")]
pub async fn index(_req: HttpRequest) -> impl Responder {
	"Welcome"
}

#[derive(Serialize)]
struct MyObj {
	name: &'static str
}

// Responder
impl Responder for MyObj {
	type Body = BoxBody;
	fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
		let body = serde_json::to_string(&self).unwrap();

		// create response and set content type
		HttpResponse::Ok()
			.content_type(ContentType::json())
			.body(body)
	}
}

pub async fn custom_type() -> impl Responder {
	MyObj {
		name: "user"
	}
}

// response asynchronously
use futures::{
	future::ok,
	stream::once
};
#[get("/stream")]
pub async fn stream() -> HttpResponse {
	let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

	HttpResponse::Ok()
		.content_type("application/json")
		.streaming(body)
}
