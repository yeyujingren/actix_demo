use std::slice::SliceIndex;

use actix_web::{
	get, HttpRequest, Responder, body::BoxBody, http::{header, Error}, HttpResponse, web, Result, HttpServer
};
use serde::Serialize;

pub async fn mult_use() -> HttpResponse {
	HttpResponse::Ok().body("aha, meet you again")
}


#[get("/show")]
pub async fn show_users() -> HttpResponse {
	HttpResponse::Ok().body("show users")
}

#[get("/show/{id}")]
pub async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
	HttpResponse::Ok().body(
		format!("User detail: {}", path.into_inner().0)
	)
}

///
/// 所有的路由变量，你可以通过 HttpRequest::match_info获得。
/// 相对的，如果你希望获取定向的数据，可以通过 Path::get()
/// 
#[get("/a/{v1}/{v2}/")]
pub async fn match_all_infor(req: HttpRequest) -> Result<String> {
	let v1: u8 = req.match_info().get("v1").unwrap().parse().unwrap();
	let v2: u8 = req.match_info().query("v2").parse().unwrap();
	let (v3, v4): (u8, u8) = req.match_info().load().unwrap();
  Ok(format!("Values {} {} {} {}", v1, v2, v3, v4))
}

///
/// 对于path中的额外额外参数，你也可以使用元组的方式，但是必须保证类型一一对应
/// 
#[get("/{username}/{id}/index.html")]
pub async fn tuple_match(info: web::Path<(String, u32)>) -> Result<String> {
	let info = info.into_inner();
	Ok(format!(
		"Welcome {}! id {}", info.0, info.1
	))
}

///
/// 也可以通过结构体来匹配，但是必须实现 serde 的 Deserialize 的相关特征
/// 
use serde::Deserialize;
#[derive(Deserialize)]
pub struct Info {
	pub username: String
}

// extract path info using serde
#[get("/{username}/index.html")]
pub async fn struct_info(info: web::Path<Info>) -> Result<String> {
	Ok(
		format!(
			"Welcome {} !", info.username
		)
	)
}


///
/// 使用 HttpRequest.url_for() 来根据资源的模式生成 URL
/// 下面的例子展示了，在你为资源声明名称为 “foo” 情况下
/// 匹配 “{a}/{b}/{c}” 的情况
/// 
#[get("/test/")]
pub async fn url_for_demo(req: HttpRequest) -> Result<HttpResponse> {
	// <- generate url for "foo" resource
	let url = req.url_for("foo", &["1", "2", "3"])?;
	Ok(
		HttpResponse::Found()
			.insert_header((header::LOCATION, url.as_str()))
			.finish()
	)
}


///
/// custom route guard
/// 守卫就是一个简单的方法，来接收一个请求对象，然后返回一个 Boolean
/// 通常的，守卫可以是任何一个实现了 Guard 特征的对象
/// 
use actix_web::{
	guard::{
		Guard,
		GuardContext
	},
	http
};

pub struct  ContentTypeHeader;

impl Guard for ContentTypeHeader {
	fn check(&self, req: &GuardContext) -> bool {
		req.head()
			.headers()
			.contains_key(http::header::CONTENT_TYPE)
	}
}










