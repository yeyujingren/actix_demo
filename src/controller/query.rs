use actix_web::{
	get, web
};
use serde::Deserialize;

#[derive(Deserialize)]
struct  Info {
	username: String
}

// this handler gets called if the query deserializes into `Info` successfully
// otherwise a 400 Bad Request error response is returned
#[get("/query")]
pub async fn index(info: web::Query<Info>) -> String {
	format!("welcome {} !", info.username)
}



