use log::info;

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("hello world");

    n_body_2::run();
}
