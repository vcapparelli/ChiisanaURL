use std::cell::OnceCell;
use java_properties::read;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::MutexGuard;

pub fn initialize_config(file: MutexGuard<OnceCell<String>>) -> HashMap<String, String> {
    let config_file =  if !file.get().unwrap().is_empty() { file.get().unwrap() } else { ".\\application.properties" };

    let file = match File::open(config_file.clone()) {
        Err(why) => panic!("There was a error opening {}: {}", config_file, why),
        Ok(file) => file,
    };

    let config_map = match read(BufReader::new(file)) {
        Err(why) => panic!(
            "There was a error opening the config file {}: {}",
            config_file, why
        ),
        Ok(file) => file,
    };

    return config_map;
}
