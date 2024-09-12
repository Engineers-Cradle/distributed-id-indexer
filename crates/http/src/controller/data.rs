use actix_web::{get, web, HttpRequest};
use serde::Deserialize;
use crate::libs::jwt::verify_jwt;
use crate::libs::redis::get_value;
use crate::libs::http::AppState;

fn get_auth_token<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("Authorization")?.to_str().ok()
}

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
async fn fetch_key(req: HttpRequest, info: web::Path<Retrievekey>, data: web::Data<AppState>) -> actix_web::web::Json<Retrievevalue> {
    let auth_token: Option<&str> = get_auth_token(&req);

    let auth_token: String = match auth_token {
        Some(token) => token.to_string(),
        None => "".to_string(),
    };

    if auth_token == "" {
        return actix_web::web::Json(Retrievevalue {
            success: false,
            value: None,
        });
    }

    let auth_token = auth_token.replace("Bearer ", "");

    let verify_token = verify_jwt(&auth_token).await;

    if !verify_token {
        return actix_web::web::Json(Retrievevalue {
            success: false,
            value: None,
        });
    }

    let key: String = format!("snowflake:id:{}", info.key.clone());
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