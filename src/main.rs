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
    println!("What type of card are you searching for?");
    let types_list = CardType::variants().join("|");
    println!("Enter a type <{}>", types_list);

    let input = utils::get_input();

    let test = CardType::from_str(&input);

    println!("{:?}", test);

    match service::search() {
        Ok(_) => println!("Search complete"),
        Err(e) => println!("Error: {:?}", e),
    }
}
