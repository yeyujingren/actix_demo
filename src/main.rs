mod controller;
mod middleware;


fn main() {
    // ssl instance
    // controller::ssl_instance().unwrap();

    // asynchronously instance
    controller::stream_instance().unwrap();
}
