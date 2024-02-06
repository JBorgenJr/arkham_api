mod api;
mod config;
mod handlers;
mod models;
mod service;
mod types;
mod utils;

use crate::{types::cards::CardType, utils::get_input};

fn main() {
    println!("Enter a command <init|search>");

    let input = utils::get_input();

    match input.as_str() {
        "init" => init(),
        "search" => search(),
        _ => println!("Invalid command"),
    }
}

fn init() {
    println!("Initializing...");
    match api::init() {
        Ok(resp) => service::categorize_cards(resp),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn search() {
    // Search Scope
    println!("What is the scope of the search?");
    println!("Enter <global|set|type>"); // TODO: use enum to get list of types ex: global, set, type?
    let search_type = get_input();

    // Search Card Types
    println!("What type of card are you searching for?");
    let types_list = CardType::variants().join("|"); // TODO: use enum to get list of types instead of ::variants()
    println!("Enter a type <{}>", types_list);
    let card_type = utils::get_input();

    // Execute Search
    match service::search(search_type, card_type) {
        Ok(value) => println!(
            "Search complete: {}",
            serde_json::to_string_pretty(&value).unwrap()
        ),
        Err(e) => println!("Error: {:?}", e),
    }
}
