/// Initialize logger at "info" level.
pub fn init_info() {
    let env = env_logger::Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env).init();
}
