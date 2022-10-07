use actix_web::{web, App, HttpRequest, HttpServer, Responder, http::KeepAlive, HttpResponse};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod home;


