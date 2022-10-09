use actix_web::{
  error, get, middleware::Logger
};
use derive_more::{Display, Error};
use log::info;


#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
pub struct MyError {
    name: &'static str,
}


// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {}

#[get("/")]
async fn info_logger() -> Result<&'static str, MyError> {
    let err = MyError { name: "test error" };
    info!("{}", err);
    Err(err)
}


