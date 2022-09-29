mod controller;
mod middleware;


fn main() {
    // ssl instance
    controller::ssl_instance().unwrap();
}
