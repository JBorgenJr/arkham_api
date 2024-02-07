mod api;
mod config;
mod handlers;
mod models;
mod service;
mod types;
mod utils;

use crate::{
    types::{cards::CardType, cycles::Cycle},
    utils::get_input,
};

fn main() {
    loop {
        println!("Enter a command <init|search|exit>");

        let input = utils::get_input();

        match input.as_str() {
            "init" => init(),
            "search" => search(),
            "exit" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid command"),
        }
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
    println!("Enter <global|cycle|type>"); // TODO: use enum to get list of types ex: global, set, type?
    let search_type = get_input();

    match search_type.as_str() {
        "global" => {}
        "cycle" => {
            // retrieve list of cycles

            let cycles_list = Cycle::variants().join("|"); // TODO: use enum to get list of types instead of ::variants()
            println!("Cycles: {:?}", cycles_list)
        }
        "type" => {}
        _ => println!("Invalid search scope"),
    }

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
