use actix_web::{post, web, App, Result};
use serde::Deserialize;

// A URL-encoded form body can be extracted to a struct,
// much like Json<T>. This type must implement serde::Deserialize.

#[derive(Deserialize)]
struct FormData {
	username: String
}

/// extract form data using serde
/// this handler gets called only if the content type is *x-www-form-urlencoded*
/// and the content of the request could be deserialized to a `FormData` struct
#[post("/form")]
pub async fn index(form: web::Form<FormData>) -> Result<String> {
	Ok(format!(
		"Welcome {} !", form.username
	))
}
