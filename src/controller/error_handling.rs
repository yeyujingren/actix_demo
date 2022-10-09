use actix_web::{
	error,
	get,
	http::{header::ContentType, StatusCode},
	App,
	HttpResponse,
	HttpServer
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum UserError {
	#[display(fmt="Validation error on field: {}", field)]
	ValidationError {
		field: String
	},

	#[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError {
	fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
		HttpResponse::build(self.status_code())
			.insert_header(ContentType::html())
			.body(self.to_string())
	}

	fn status_code(&self) -> StatusCode {
		match  *self {
			UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
			UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR
		}
	}
}

fn do_thing_that_fails() -> Result<&'static str, UserError> {
	Err(
		UserError::InternalError
	)
}


#[get("/error/handling")]
pub async fn error_handling() -> Result<&'static str, UserError> {
	do_thing_that_fails().map_err(|_e| UserError::InternalError)?;
    Ok("success!")
}


