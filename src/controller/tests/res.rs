use actix_web::{HttpResponse, HttpRequest};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppState {
  pub count: u8
}

pub async fn index(res: HttpRequest) -> HttpResponse {
  HttpResponse::Ok().body("this is a success req")
}
