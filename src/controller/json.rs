use actix_web::{get, web, App, Result, Responder};
use serde::Deserialize;

//
// Json<T> allows deserialization of a request body into a struct.
// To extract typed information from a request’s body,
// the type T must implement serde::Deserialize.
//

#[derive(Deserialize)]
pub struct Info {
    username: String,
}

/// deserialize "info" from request's body
pub async fn index(info: web::Json<Info>) -> impl Responder{
    format!("Welcome {} !", info.username)
}
