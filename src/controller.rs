use actix_web::{get, App, HttpRequest, HttpServer, Responder, http::KeepAlive};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod home;

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
use std::time::Duration;

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






