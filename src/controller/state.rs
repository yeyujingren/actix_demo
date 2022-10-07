// Application state is accessible from the handler with the web::Data extractor;
// however, state is accessible as a **read-only** reference.
// If you need mutable access to state, it must be implemented.

use actix_web::{web, Responder};
use std::cell::Cell;

#[derive(Clone)]
pub struct AppState {
    pub count: Cell<usize>,
}

pub async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!("counte: {}", data.count.get())
}

pub async fn add_one(data: web::Data<AppState>) -> impl Responder {
    let count = data.count.get();
    data.count.set(count + 1);

    format!("count: {}", data.count.get())
}
