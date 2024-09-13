use actix_web::{get, post, web, HttpRequest};
use serde::Deserialize;
use crate::libs::jwt::verify_jwt;
use crate::libs::redis::get_value;
use crate::libs::http::AppState;

fn get_auth_token<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("Authorization")?.to_str().ok()
}

#[derive(Deserialize)]
struct RetrieveKey {
    key: String,
    key_type: String,
}

#[derive(Deserialize, serde::Serialize)]
struct RetrieveValue {
    success: bool,
    value: Option<String>,
}

#[get("/retrieve/{key_type}/{key}")]
async fn fetch_key(req: HttpRequest, info: web::Path<RetrieveKey>, data: web::Data<AppState>) -> actix_web::web::Json<RetrieveValue> {
    let auth_token: Option<&str> = get_auth_token(&req);

    let auth_token: String = match auth_token {
        Some(token) => token.to_string(),
        None => "".to_string(),
    };

    if auth_token == "" {
        return actix_web::web::Json(RetrieveValue {
            success: false,
            value: None,
        });
    }

    let auth_token = auth_token.replace("Bearer ", "");

    let verify_token = verify_jwt(&auth_token).await;

    if !verify_token {
        return actix_web::web::Json(RetrieveValue {
            success: false,
            value: None,
        });
    }

    let key: String = format!("{}:id:{}", info.key_type.clone(), info.key.clone());
    let mut redis_multiplex_connection: redis::aio::MultiplexedConnection = data.redis_client.get_multiplexed_async_connection().await.unwrap();

    let value: String = get_value(&mut redis_multiplex_connection, &key).await;

    if value == "" {
        return actix_web::web::Json(RetrieveValue {
            success: false,
            value: None,
        });
    }

    actix_web::web::Json(RetrieveValue {
        success: true,
        value: Some(value),
    })
}

#[derive(Deserialize)]
struct RetrieveKeyBatch {
    key_type: String,
    keys: Vec<String>,
}

#[derive(Deserialize, serde::Serialize)]
struct EachBatchRetrieveValue {
    value: String,
    success: bool,
}

#[derive(Deserialize, serde::Serialize)]
struct RetrieveBatchValue {
    success: bool,
    values: Option<Vec<EachBatchRetrieveValue>>,
}

#[post("/retrieve/batch")]
async fn fetch_batch_keys(req: HttpRequest, info: web::Json<RetrieveKeyBatch>, data: web::Data<AppState>) -> actix_web::web::Json<RetrieveBatchValue> {
    let auth_token: Option<&str> = get_auth_token(&req);

    let auth_token: String = match auth_token {
        Some(token) => token.to_string(),
        None => "".to_string(),
    };

    if auth_token == "" {
        return actix_web::web::Json(RetrieveBatchValue {
            success: false,
            values: None,
        });
    }

    let auth_token = auth_token.replace("Bearer ", "");

    let verify_token = verify_jwt(&auth_token).await;

    if !verify_token {
        return actix_web::web::Json(RetrieveBatchValue {
            success: false,
            values: None,
        });
    }

    let mut redis_multiplex_connection: redis::aio::MultiplexedConnection = data.redis_client.get_multiplexed_async_connection().await.unwrap();

    let mut values: Vec<EachBatchRetrieveValue> = Vec::new();

    for key in info.keys.clone() {
        let key: String = format!("{}:id:{}", info.key_type.clone(), key.clone());
        let value: String = get_value(&mut redis_multiplex_connection, &key).await;

        if value == "" {
            values.push(EachBatchRetrieveValue {
                value: "".to_string(),
                success: false,
            });
        } else {
            values.push(EachBatchRetrieveValue {
                value: value,
                success: true,
            });
        }
    }

    actix_web::web::Json(RetrieveBatchValue {
        success: true,
        values: Some(values),
    })
}

pub fn init_data_routes(config: &mut web::ServiceConfig) {
    config.service(fetch_key);
    config.service(fetch_batch_keys);
}