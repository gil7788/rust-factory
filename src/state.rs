use std::fs::File;
use std::fs;
use std::io::{Read};
use serde_json::{Map, Value, json};


/// Reads the state from a file, returning an empty `Map` if the file does not exist.
pub fn read_file(file_name: &str) -> Map<String, Value> {
    match File::open(&file_name) {
        Ok(mut file) => {
            let mut data = String::new();
            match file.read_to_string(&mut data) {
                Ok(_) => {
                    serde_json::from_str(&data).unwrap_or_else(|_| Map::new())
                },
                Err(_) => Map::new(),
            }
        },
        Err(_) => Map::new(),
    }
}

pub fn write_to_file(file_name: &str,  state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name.to_string(),  new_data.to_string()).expect("Unable to write file");
}
