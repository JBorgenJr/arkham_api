mod api;
mod config;
mod handlers;
mod models;
mod service;
mod types;
mod utils;

use crate::types::cards::CardType;

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
    println!("What type of search are you performing?");
    // TODO: use enum to get list of types
    println!("What type of card are you searching for?");

    // TODO: use enum to get list of types
    let types_list = CardType::variants().join("|");
    println!("Enter a type <{}>", types_list);

    let input = utils::get_input();

    match service::search(input) {
        Ok(value) => println!("Search complete: {:?}", value), // TODO: map function to use
        // .to_string_pretty()?
        Err(e) => println!("Error: {:?}", e),
    }
}
