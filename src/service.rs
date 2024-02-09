use crate::{handlers, types::cards::CardType, utils};
use serde_json::Value;
use std::{env, fs, fs::File, io::Write, path::PathBuf};
// use std::error::Error;
// use crate::models::cards::*;

fn get_card_handler(card_code: CardType) -> Box<dyn handlers::CardHandler> {
    match card_code {
        CardType::Act => Box::new(handlers::ActHandler {}),
        CardType::Agenda => Box::new(handlers::AgendaHandler {}),
        CardType::Asset => Box::new(handlers::AssetHandler {}),
        CardType::Enemy => Box::new(handlers::EnemyHandler {}),
        CardType::Event => Box::new(handlers::EventHandler {}),
        CardType::Investigator => Box::new(handlers::InvestigatorHandler {}),
        CardType::Key => Box::new(handlers::KeyHandler {}),
        CardType::Location => Box::new(handlers::LocationHandler {}),
        CardType::Scenario => Box::new(handlers::ScenarioHandler {}),
        CardType::Skill => Box::new(handlers::SkillHandler {}),
        CardType::Story => Box::new(handlers::StoryHandler {}),
        CardType::Treachery => Box::new(handlers::TreacheryHandler {}),
        _ => Box::new(handlers::DefaultHander),
    }
}

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
                let handler = get_card_handler(type_code);
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
    let files: Vec<Value> = utils::get_files();

    let mut all_cards = File::create("output.json").unwrap();

    writeln!(all_cards, "[").unwrap();

    // Iterate over each JSON value in the files vector
    for (index, file) in files.iter().enumerate() {
        // Serialize the JSON value to a string
        let json_string = serde_json::to_string_pretty(file).unwrap();
        // Write the JSON string to the output file
        writeln!(all_cards, "{}", json_string).unwrap();
        // Write a comma after each JSON value except for the last one
        if index < files.len() - 1 {
            writeln!(all_cards, ",").unwrap();
        }
    }

    writeln!(all_cards, "]").unwrap();
}

