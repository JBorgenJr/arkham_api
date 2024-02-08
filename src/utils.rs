use serde_json::Value;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

pub fn get_files() -> Vec<Value> {
    let mut file_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    file_path.push("data");

    let mut files = Vec::new();

    for entry in WalkDir::new(&file_path) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            if let Ok(file_content) = fs::read_to_string(entry.path()) {
                if let Ok(json_value) = serde_json::from_str::<Value>(&file_content) {
                    files.push(json_value);
                }
            }
        }
    }
    files
}
