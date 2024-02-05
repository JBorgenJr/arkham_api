use serde_json::Value;
use crate::config::API_BASE_URL;

#[tokio::main]
pub async fn init() -> Result<String, Box<dyn std::error::Error>> {
    println!("Fetching data from ArkhamDB...");

    let url = format!("{}{}", API_BASE_URL, "cards/");

    let client = reqwest::Client::new();

    let response = client.get(&url).query(&[("encounter", "1")]).send().await?;

    let mut json_string = String::new();

    if response.status().is_success() {
        let cards: Value = response.json().await?;

        json_string = serde_json::to_string_pretty(&cards)?;
    } else {
        println!("Request failed with status: {}", response.status());
    }

    println!("Data fetched successfully.");

    Ok(json_string)
}
