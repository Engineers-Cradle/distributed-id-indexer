use actix_web::{get, web};
use serde::Deserialize;
use crate::libs::redis::get_value;
use crate::libs::http::AppState;

#[derive(Deserialize)]
struct Retrievekey {
    key: String,
}

#[derive(Deserialize, serde::Serialize)]
struct Retrievevalue {
    success: bool,
    value: Option<String>,
}

#[get("/retrieve/{key}")]
async fn fetch_key(info: web::Path<Retrievekey>, data: web::Data<AppState>) -> actix_web::web::Json<Retrievevalue> {
    let key: String = info.key.clone();
    let mut redis_multiplex_connection: redis::aio::MultiplexedConnection = data.redis_client.get_multiplexed_async_connection().await.unwrap();

    let value: String = get_value(&mut redis_multiplex_connection, &key).await;

    if value == "" {
        return actix_web::web::Json(Retrievevalue {
            success: false,
            value: None,
        });
    }

    actix_web::web::Json(Retrievevalue {
        success: true,
        value: Some(value),
    })
}

pub fn init_data_routes(config: &mut web::ServiceConfig) {
    config.service(fetch_key);
}