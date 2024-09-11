use redis::AsyncCommands;

pub async fn connection_to_redis(
    redis_url: &str,
) -> redis::Client {
    let client: redis::Client = redis::Client::open(redis_url).unwrap();
    client
}

pub async fn get_value(connection: &mut redis::aio::MultiplexedConnection, key: &str) -> String {
    let value = connection.get(key).await;
   
    match value {
        Ok(value) => value,
        Err(_) => "".to_string(),
    }
}

pub async fn set_value(connection: &mut redis::aio::MultiplexedConnection, key: &str, value: &str) {
    let _: () = connection.set(key, value).await.unwrap();
}

pub async fn start_pub_sub() {
    let env_config = crate::config::env::get_env();

    let redis_client: redis::Client = crate::libs::redis::connection_to_redis(
        &env_config.redis_url
    ).await;

    let mut connection = redis_client.clone().get_connection().unwrap();
    let mut pubsub = connection.as_pubsub();

    pubsub.psubscribe("snowflake:id:set:*").unwrap();

    println!("Subscribed to snowflake:id:set:*");

    loop {
        let msg = pubsub.get_message().unwrap();
        let payload: String = msg.get_payload().unwrap();
        
        let payload_parts: Vec<&str> = payload.split(":").collect();

        if payload_parts.len() != 3 {
            continue;
        }

        let db_name: &str = payload_parts[0];
        let table_name: &str = payload_parts[1];
        let entry_id: &str = payload_parts[2];

        let key: String = format!("snowflake:id:{}", &entry_id);
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