use crate::{handlers, types::cards::CardType, utils};
use serde_json::Value;
use std::{env, fs, path::PathBuf};

pub fn categorize_cards(resp: String) {
    println!("Categorizing cards...");
    let json = serde_json::from_str(&resp).unwrap();

    if let Value::Array(cards) = json {
        for card in cards {
            if let Some(type_code) = card
                .get("type_code")
                .and_then(|tc| CardType::from_str(tc.as_str().unwrap()))
            {
                let code = card.get("code").and_then(|c| c.as_str()).unwrap();
                let handler = handlers::get_card_handler(type_code);
                match handler.handle_card(card.clone()) {
                    Ok(_) => {}
                    Err(e) => {
                        // println!("Type code: {:?}", type_code);
                        println!("Handling card: {}", code);
                        println!("Error: {:?}", e);
                    }
                }
            };
        }
    }
    println!("Categorizing complete");
}

pub fn create_card_file(card: Value) -> PathBuf {
    let pack_code = card.get("pack_code").and_then(|pc| pc.as_str()).unwrap();
    let code = card.get("code").and_then(|c| c.as_str()).unwrap();
    let type_code = card.get("type_code").and_then(|tc| tc.as_str()).unwrap();

    let path_components = ["data", pack_code, type_code, code];

    let mut file_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    file_path.extend(&path_components);
    file_path.set_extension("json");

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create directories");
    };

    file_path
}

pub fn save_card_to_file(path: PathBuf, contents: String) {
    match fs::write(path.clone(), contents) {
        Ok(_) => {}
        Err(e) => println!("Error writing to {:?}: {:?}", path, e),
    }
}

pub fn search() {
    println!("Search by name (enter to skip): > ");
    let name_query = utils::get_input();
    println!("Search by type (enter to skip): > ");
    let type_query = utils::get_input();

    println!("You entered the following search criteria:");
    println!("Name: {}", name_query);
    println!("Type: {}", type_query);

    let cards: Vec<Value> = utils::get_all_cards();

    let results: Vec<&Value> = cards
        .iter()
        .filter(|c| {
            let name_matches = if let Some(name) = c.get("name").and_then(|n| n.as_str()) {
                name.to_lowercase()
                    .contains(name_query.to_lowercase().as_str())
            } else {
                false
            };

            let type_matches = if let Some(card_type) = c.get("type_code").and_then(|t| t.as_str())
            {
                // Adjust "type_code" if needed
                card_type
                    .to_lowercase()
                    .contains(type_query.to_lowercase().as_str())
            } else {
                false
            };

            name_matches && type_matches
        })
        .collect();

    let result_count = results.len();
    println!("Search results count: {}", result_count);

    println!("Expand search results? (y/n) > ");
    let expand = utils::get_input();

    match expand.as_str() {
        "y" => {
            let results = serde_json::to_string_pretty(&results).unwrap();
            println!("Search results: {}", results);
        }
        _ => {}
    }
}
