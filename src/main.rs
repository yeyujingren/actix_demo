mod controller;
mod middleware;


fn main() {
    // ssl instance
    // controller::ssl_instance().unwrap();

    // asynchronously instance
    // controller::stream_instance().unwrap();

    // path instance
    // controller::path_instance().unwrap();

    // json request
    controller::json_instance().unwrap();
}
