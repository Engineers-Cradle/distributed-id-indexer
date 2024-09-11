use actix_web::{get, web};

#[get("/health")]
async fn health() -> String {
    format!("{{\"status\": \"ok\"}}")
}

pub fn init_root_routes(config: &mut web::ServiceConfig) {
    config.service(health);
}