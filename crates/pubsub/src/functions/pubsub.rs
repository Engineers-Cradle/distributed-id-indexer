use crate::libs::redis::{get_value, set_value};

pub async fn start_pub_sub() {
    let env_config = crate::config::env::get_env();

    let redis_client: redis::Client = crate::libs::redis::connection_to_redis(
        &env_config.redis_url
    ).await;

    let mut connection = redis_client.clone().get_connection().unwrap();
    let mut pubsub = connection.as_pubsub();

    pubsub.psubscribe("snowflake:id:set:*").unwrap();
    pubsub.psubscribe("email:id:set:*").unwrap();

    println!("Subscribed to snowflake:id:set:*");
    println!("Subscribed to email:id:set:*");

    loop {
        let msg = pubsub.get_message().unwrap();
        let payload: String = msg.get_payload().unwrap();
        
        let payload_parts: Vec<&str> = payload.split(":").collect();

        if payload_parts.len() != 3 {
            continue;
        }

        let db_name: &str = payload_parts[0];
        let table_name: &str = payload_parts[1];
        let entry_type: &str = payload_parts[2];
        let entry_id: &str = payload_parts[3];

        let key: String;

        if entry_type == "snowflake" {
            key = format!("snowflake:id:{}", &entry_id);
        } else if entry_type == "email" {
            key = format!("email:id:{}", &entry_id);
        } else {
            continue;
        }

        let mut redis_multiplex_connection: redis::aio::MultiplexedConnection = redis_client.get_multiplexed_async_connection().await.unwrap();

        let pre_exist_value: String = get_value(&mut redis_multiplex_connection, &key).await;

        if pre_exist_value != "" {
            continue;
        }

        let value: String = format!("{}:{}", db_name, table_name);

        set_value(&mut redis_multiplex_connection, &key, &value).await;

        println!("Key: {}, Value: {}", key, value);
    }
}