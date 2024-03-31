// =================================
// Main Application Logic
// =================================

mod api;
mod config;
mod handlers;
mod models;
mod service;
mod types;
mod utils;
mod schema;

fn main() {
    loop {
        println!("Enter a command: <init|search|exit>");

        let input: String = utils::get_input();

        match input.as_str() {
            "init" => init(),
            "search" => service::search(),
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
