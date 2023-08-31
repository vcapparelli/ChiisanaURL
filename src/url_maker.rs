use std::collections::HashMap;

pub fn get_custom_url(config: &HashMap<String, String>, custom_path: &str) -> String {
    let base_url = config.get("base_url").unwrap();

    if !base_url.ends_with("/"){
        base_url.to_owned().push_str("/");
    }

    return base_url.to_owned() + custom_path;
}

pub fn get_base_url_plus_path(config: &HashMap<String, String>, path: &str) -> String {
    let base_url = config.get("base_url").unwrap();

    if !base_url.ends_with("/"){
        base_url.to_owned().push_str("/");
    }

    return base_url.to_owned() + path;
}

pub fn get_generated_url(config: &HashMap<String, String>) -> String {
    use rand::Rng;
    let chars: &[u8] = config.get("chars").unwrap().as_bytes();
    let url_length: usize = config.get("path_length").unwrap().parse().unwrap();
    let mut rng = rand::thread_rng();

    let sufix: String = (0..url_length)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx] as char
        })
        .collect();
    let base_url = config.get("base_url").unwrap();

    if !base_url.ends_with("/"){
        base_url.to_owned().push_str("/");
    }

    return base_url.to_owned() + &sufix;
}

pub fn get_random_chars(config: &HashMap<String, String>) -> String {
    use rand::Rng;
    let chars: &[u8] = config.get("chars").unwrap().as_bytes();
    let mut rng = rand::thread_rng();
    let size: i16 = config.get("password_size").unwrap().parse().unwrap();

    let random_chars: String = (0..size)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx] as char
        })
        .collect();
    return random_chars;
}
