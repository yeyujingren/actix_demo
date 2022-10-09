use std::io;
use actix_files::NamedFile;
use actix_web::{
	HttpRequest,
	get,
	error,
	Result, HttpResponse, 
	http:: {
		header::ContentType,
		StatusCode
	}
};
use derive_more::{
	Display,
	Error
};

pub fn not_found_file(_req: HttpRequest) -> io::Result<NamedFile> {
	Ok(
		NamedFile::open("public/index.html")?
	)
}

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
pub struct MyError {
	name: &'static str
}

// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {
	
}

pub async fn custom_error() -> Result<& 'static str, MyError> {
	Err(MyError {
		name: "test"
	})
}

#[derive(Debug, Display, Error)]
pub enum MyErrorEnum {
	#[display(fmt = "internal error")]
	InternalError,

	#[display(fmt = "bad request")]
	BadClientData,

	#[display(fmt = "timeout")]
	Timeout
}

impl error::ResponseError for MyErrorEnum {
	fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
		HttpResponse::build(self.status_code())
			.insert_header(ContentType::html())
			.body(self.to_string())
	}

	fn status_code(&self) -> StatusCode {
		match *self {
			MyErrorEnum::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
			MyErrorEnum::BadClientData => StatusCode::BAD_REQUEST,
			MyErrorEnum::Timeout => StatusCode::GATEWAY_TIMEOUT
		}
	}
}

#[get("/error/client")]
pub async fn response_error() -> Result<& 'static str, MyErrorEnum> {
	Err(
		MyErrorEnum::BadClientData
	)
}

// convert MyError, which doesn’t implement the ResponseError trait,
// to a 400 (bad request) using map_err:
#[derive(Debug)]
struct MyErrorConvert {
	name: & 'static str
}

#[get("/error/convert")]
pub async fn convert_error() -> Result<& 'static str> {
	let result: Result<&'static str, MyErrorConvert> = Err(
		MyErrorConvert {
			name: "test errror"
		}
	);

	Ok(
		result.map_err(|e| error::ErrorBadRequest(e.name))?	
	)
}



