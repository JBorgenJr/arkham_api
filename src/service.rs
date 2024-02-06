use crate::handlers;
use crate::models::*;
use crate::types::cards::CardType;
use serde_json::Value;
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

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

pub fn search(
    search_type: String,
    card_type: String,
) -> Result<Vec<investigator::Investigator /* TODO: Update types based on input */>, Box<dyn Error>>
{
    println!("Searching...");
    println!("Type: {}", card_type);

    let path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
        .join("data")
        .join(search_type) // TODO: Pass in which set to search
        .join(&card_type);

    let mut investigators: Vec<investigator::Investigator> = Vec::new(); // TODO: Update types
                                                                         // based on input

    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let contents = fs::read_to_string(entry.path())?;

        let investigator: investigator::Investigator = serde_json::from_str(contents.as_str())?; // TODO:
                                                                                                 // Update types based on input
        investigators.push(investigator);
    }

    println!("Search complete");

    Ok(investigators)
}
