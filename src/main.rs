mod controller;
mod middleware;


fn main() {
    // controller::error_instance().unwrap();
    middleware::logger_instance().unwrap();
}
