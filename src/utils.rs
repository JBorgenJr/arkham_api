use serde_json::Value;
use std::{env, fs, io, path::PathBuf};
use walkdir::WalkDir;

// =================================
//  User Input & Data Loading Utilities
// =================================

pub fn get_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

pub fn get_all_cards() -> Vec<Value> {
    let mut file_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    file_path.push("data");

    let mut cards = Vec::new();

    for entry in WalkDir::new(&file_path) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            if let Ok(file_content) = fs::read_to_string(entry.path()) {
                if let Ok(json_value) = serde_json::from_str::<Value>(&file_content) {
                    cards.push(json_value);
                }
            }
        }
    }

    let mut all_cards = Vec::new();

    for file in cards.iter() {
        all_cards.push(file.clone());
    }

    cards
}
