use actix_web::{
	web, post, HttpResponse, Error, error, get
};
use serde::{
	Deserialize,
	Serialize
};

///
/// 这边有多种方式用于 json 格式的数据的 反序列化
/// 
/// 第一种方式是 使用 Json 提取器。首先，你需要定一个接口 `Json<T>` 作为入参的方法。
/// 然后，你需要通过 `to()` 方法来注册该方法。关于T，你可以通过 `serder_json::Value`	来生成任意类型的合法的json对象
/// 
#[derive(Deserialize)]
pub struct Info {
	username: String
}

/// 使用 serde 来格式化 Info
pub async fn format_info(info: web::Json<Info>) -> actix_web::Result<String> {
	Ok(
		format!("Welcome {}!", info.username)
	)
}


/// 
/// 我们经常也会缓存下来 payload，然后再来解析他
/// 下面的例子中，我们定一个 MyObj 的 结构体，然后通过上述操作来解析json 
/// 
use futures::StreamExt;

#[derive(Serialize, Deserialize)]
struct  MyObj {
	name: String,
	number: i32
}

const MAX_SIZE: usize = 262_144; // 256kb

#[post("/")]
pub async fn manual_and_format_obj(mut payload: web::Payload) -> actix_web::Result<HttpResponse, Error> {
	// payload 是流式的二进制对象
	let mut body = web::BytesMut::new();
	while let Some(chunk) = payload.next().await {
		let chunk = chunk?;
		// 限制请求体大小
		if (body.len() + chunk.len()) > MAX_SIZE {
			return Err(error::ErrorBadRequest("overflow"));
		}
		body.extend_from_slice(&chunk);
	}

	// 缓存完 payload 现在反序列化
	let obj = serde_json::from_slice::<MyObj>(&body)?;
	Ok(
		HttpResponse::Ok().json(obj)
	)
}

// 流 请求
#[get("/")]
pub async fn stream_request(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item?;
        println!("Chunk: {:?}", &item);
        bytes.extend_from_slice(&item);
    }

    Ok(HttpResponse::Ok().finish())
}






