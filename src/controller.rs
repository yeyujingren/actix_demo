use actix_web::{
    http::{self, header::ContentEncoding, StatusCode},
    web, App, Error, HttpRequest, HttpResponse,
};
use futures::stream;
use std::task::Poll;

async fn sse(_req: HttpRequest) -> HttpResponse {
    let mut counter: usize = 5;

    // yields `data: N` where N in [5; 1]
    let server_events = stream::poll_fn(move |_cx| -> Poll<Option<Result<web::Bytes, Error>>> {
        if counter == 0 {
            return Poll::Ready(None);
        }
        let payload = format!("data: {}\n\n", counter);
        counter -= 1;
        Poll::Ready(Some(Ok(web::Bytes::from(payload))))
    });

	HttpResponse::build(StatusCode::OK)
        .insert_header((http::header::CONTENT_TYPE, "text/event-stream"))
        .insert_header(ContentEncoding::Identity)
        .streaming(server_events)

}


pub fn main() {
    App::new().route("/", web::get().to(sse));
}

// 单元测试
#[cfg(test)]
mod tests {
	use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test, web, App, body, body::MessageBody as _, rt::pin
    };
	use futures::future;
    mod res;

    #[actix_web::test]
    async fn test_index_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let resp = res::index(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_index_not_ok() {
        let req = test::TestRequest::default().to_http_request();
        let resp = res::index(req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().route("/", web::get().to(res::index))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success())
    }

    // mock data
    #[actix_web::test]
    async fn test_index_get_with_data() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(res::AppState { count: 4 }))
                .route("/", web::get().to(res::index)),
        )
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp: res::AppState = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.count, 4);
    }

    // stream response
	#[actix_web::test]
    async fn test_stream_chunk() {
        let app = test::init_service(App::new().route("/", web::get().to(sse))).await;
        let req = test::TestRequest::get().to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = resp.into_body();
        pin!(body);

        // first chunk
        let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
        assert_eq!(
            bytes.unwrap().unwrap(),
            web::Bytes::from_static(b"data: 5\n\n")
        );

        // second chunk
        let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
        assert_eq!(
            bytes.unwrap().unwrap(),
            web::Bytes::from_static(b"data: 4\n\n")
        );

        // remaining part
        for i in 0..3 {
            let expected_data = format!("data: {}\n\n", 3 - i);
            let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
            assert_eq!(bytes.unwrap().unwrap(), web::Bytes::from(expected_data));
        }
    }

	#[actix_web::test]
    async fn test_stream_full_payload() {
        let app = test::init_service(App::new().route("/", web::get().to(sse))).await;
        let req = test::TestRequest::get().to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;
        assert_eq!(
            bytes.unwrap(),
            web::Bytes::from_static(b"data: 5\n\ndata: 4\n\ndata: 3\n\ndata: 2\n\ndata: 1\n\n")
        );
    }
}
