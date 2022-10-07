use actix_web::{
  get, web, Result, HttpRequest
};

///
/// **demo 01**
/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
/// 
// #[get("/users/{user_id}/{friend}")]
// pub async fn index(path: web::Path<(u32, String)>) -> Result<String> {
// 	let (user_id, friend) = path.into_inner();
// 	Ok(format!(
// 		"Welcome {}, user_id {}!", friend, user_id
// 	))
// }


use serde::Deserialize;

#[derive(Deserialize)]
struct  Info {
	user_id: u32,
	friend: String
}

/// **demo 02**
/// extract path info using serde
/// 
/// 
// #[get("/users/{user_id}/{friend}")] // <- define path parameters
// async fn index(info: web::Path<Info>) -> Result<String> {
//     Ok(format!(
//         "Welcome {}, user_id {}!",
//         info.friend, info.user_id
//     ))
// }

/// **demo 03**
/// As a non-type-safe alternative, it’s also possible to query 
/// (see match_info docs) the request for path parameters by name within a handler:
/// 
#[get("/users/{user_id}/{friend}")]
pub async fn index(req: HttpRequest) -> Result<String> {
	let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
	let userid: i32 = req.match_info().query("user_id").parse().unwrap();
	Ok(format!("Welcome {}, user_id {}!", name, userid)) 
}











