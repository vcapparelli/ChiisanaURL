use redis::Commands;
use std::collections::HashMap;

fn setup_and_get_client(
    redis_endpoint: &str,
    redis_port: &str,
    redis_user: &str,
    redis_password: &str,
) -> redis::Client {
    let client;
    if !redis_user.is_empty() && !redis_password.is_empty() {
        client = redis::Client::open(
            "redis://".to_owned()
                + redis_user
                + ":"
                + redis_password
                + "@"
                + redis_endpoint
                + ":"
                + redis_port
                + "/",
        )
        .unwrap();
    } else {
        client =
            redis::Client::open("redis://".to_owned() + redis_endpoint + ":" + redis_port + "/")
                .unwrap();
    }

    return client;
}

pub fn set_value(key: &str, value: &str, config: &HashMap<String, String>) {
    let mut client = get_client(config);
    let _: () = client.set(key, value).unwrap();
}

pub fn get_value(key: &str, config: &HashMap<String, String>) -> String {
    let mut client = get_client(config);

    if client.exists(key).expect("Redis error!") {
        return client.get(key).unwrap();
    }

    return "".to_owned();
}

fn get_client(config: &HashMap<String, String>) -> redis::Client {
    let client = setup_and_get_client(
        config.get("endpoint_redis").unwrap(),
        config.get("port_redis").unwrap(),
        config.get("user_redis").unwrap(),
        config.get("password_redis").unwrap(),
    );

    return client;
}
