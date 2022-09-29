use actix_web::{
	get, HttpRequest, Responder
};

#[get("/")]
pub async fn index(_req: HttpRequest) -> impl Responder {
	"Welcome"
}
