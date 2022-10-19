use actix_web::{
	web, post, HttpResponse, Error, error, get, Result, Responder, http
};
use serde::{
	Deserialize,
	Serialize
};


// json response
// 必须实现 Serialize 特征
#[derive(Serialize)]
pub struct MyObj {
	name: String
}

#[get("/res/json/{name}")]
// 通过适应Json 这种方式来代替 HttpResponse 上的 .json 方法，可以清楚的函数返回的是JSON格式
// 而不是其他响应
pub async fn res_json(name: web::Path<String>) -> Result<impl Responder> {
	let obj = MyObj {
		name: name.to_string()
	};
	Ok(web::Json(obj))
}

// 使用 brotli 压缩响应体
#[get("/compress/br")]
pub async fn index_br() -> HttpResponse {
    HttpResponse::Ok().body("data")
}


// 使用 单独的 压缩方式
#[get("/compress/identity")]
pub async fn index_ident() -> HttpResponse {
	HttpResponse::Ok()
		// 通过增加标识来申明当前响应已经被压缩。防止二次压缩
		.insert_header(http::header::ContentEncoding::Identity)
		.body("data")
}

// 使用 Gzip 压缩
static HELLO_WORLD: &[u8] = &[
    0x1f, 0x8b, 0x08, 0x00, 0xa2, 0x30, 0x10, 0x5c, 0x00, 0x03, 0xcb, 0x48, 0xcd, 0xc9, 0xc9,
    0x57, 0x28, 0xcf, 0x2f, 0xca, 0x49, 0xe1, 0x02, 0x00, 0x2d, 0x3b, 0x08, 0xaf, 0x0c, 0x00,
    0x00, 0x00,
];

#[get("/compress/gzip")]
pub async fn gzip_compress() -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(http::header::ContentEncoding::Gzip)
        .body(HELLO_WORLD)
}




