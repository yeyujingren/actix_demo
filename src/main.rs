mod controller;
mod middleware;


fn main() {
    // namespace
    // controller::namespaceInstance().unwrap();

    // common state
    // controller::stateInstance().unwrap();
    // mutable state
    // controller::mutable_state_instance().unwrap();

    // config
    // controller::config_instace().unwrap();

    // multi threading
    controller::multi_thread_instance().unwrap();
}
