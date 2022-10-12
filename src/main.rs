mod controller;


fn main() {
  // format json by serde
  controller::format_info_instance().unwrap();
}
