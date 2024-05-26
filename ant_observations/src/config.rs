pub fn init() {
    dotenv::dotenv().ok();
    env_logger::init();
}
