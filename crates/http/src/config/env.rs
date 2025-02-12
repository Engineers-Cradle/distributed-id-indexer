use std::env;
use dotenv::dotenv;

pub struct Env {
    pub redis_url: String,
    pub web_server_port: String,
    pub num_workers: usize,
    pub log_level: String,
    pub m2m_auth_registry_base_url: String,
}

impl Env {
    pub fn new() -> Self {
        dotenv().ok();
        
        Self {
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            web_server_port: env::var("WEB_SERVER_PORT").expect("WEB_SERVER_PORT must be set"),
            num_workers: env::var("NUM_WORKERS").expect("NUM_WORKERS must be set").parse::<usize>().unwrap(),
            log_level: env::var("LOG_LEVEL").expect("LOG_LEVEL must be set"),
            m2m_auth_registry_base_url: env::var("M2M_AUTH_REGISTRY_BASE_URL").expect("M2M_AUTH_REGISTRY_BASE_URL must be set"),
        }
    }
}

pub fn get_env() -> Env {
    Env::new()
}