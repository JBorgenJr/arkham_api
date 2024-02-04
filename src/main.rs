mod api;
mod handlers;
mod models;
mod service;
mod types;

use std::io;

use crate::types::cards::CardType;

fn main() {
    println!("Enter a command <init|search>");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    match input {
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

    match service::search() {
        Ok(_) => println!("Search complete"),
        Err(e) => println!("Error: {:?}", e),
    }
}
