use actix_web::{web, App, HttpRequest, HttpServer, Responder, http::KeepAlive, HttpResponse};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod home;
mod path;
mod query;
mod json;
mod forms;
mod state;

///
/// before
/// To create the key.pem and cert.pem use the command. Fill in your own subject
/// ```
/// $ openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -sha256
/// ```
/// 
/// To remove the password, then copy nopass.pem to key.pem
/// ```
/// $ openssl rsa -in key.pem -out nopass.pem
/// ```
/// 

#[actix_web::main]
pub async fn ssl_instance() -> std::io::Result<()> {
    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| App::new().service(home::index))
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}

///
/// Keep-Alive
/// `Duration::from_secs(75)`or `KeepAlive::Timeout(75)`: enables 75 second keep-alive timer.
/// `KeepAlive::Os`: uses OS keep-alive.
/// `None` or `KeepAlive::Disabled`: disables keep-alive.
/// 
use std::{time::Duration, cell::Cell};

use self::state::AppState;

#[actix_web::main]
pub async fn keep_alive_instance() -> std::io::Result<()> {
    // Set keep-alive to 75s
    let _one = HttpServer::new(|| {
        App::new().service(home::index)
    }).keep_alive(Duration::from_secs(75));

    // use OS's keep-alive (usually quite long)
    let _two = HttpServer::new(|| {
        App::new().service(home::index)
    }).keep_alive(KeepAlive::Os);

    // Disabled keep-alive
    let _there = HttpServer::new(|| {
        App::new().service(home::index)
    }).keep_alive(None);
    // HttpServer::shutdown_timeout(self, Duration::from_secs(30));
    Ok(())
}


/// 
/// Response body can be generated asynchronously. 
/// In this case, body must implement the stream trait Stream<Item=Bytes, Error=Error>
/// 
#[actix_web::main]
pub async fn stream_instance() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(home::stream)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


///
/// path
/// 
#[actix_web::main]
pub async fn path_instance() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(query::index))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


///
/// Json
/// 
/// Some extractors provide a way to configure the extraction process. 
/// To configure an extractor, pass its configuration object to the resource’s `.app_data()` method. 
/// In the case of Json extractor it returns a JsonConfig. 
/// You can configure the maximum size of the JSON payload as well as a custom error handler function.
/// 
#[actix_web::main]
pub async fn json_instance() -> std::io::Result<()> {
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(3072)
            .error_handler(|err, _req| {
                actix_web::error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });
        App::new().service(
            web::resource("/json")
                // change json extractor configuration
                .app_data(json_config)
                .route(web::post().to(json::index))
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


/// dynamic change app state
/// 
#[actix_web::main]
pub async fn dynamic_state_instance() -> std::io::Result<()> {
    let data = AppState {
        count: Cell::new(0)
    };


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .route("/state", web::to(state::show_count))
            .route("/state/add", web::to(state::add_one))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}




