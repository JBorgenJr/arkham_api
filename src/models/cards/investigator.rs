use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Investigator {
    alternated_by: Option<Vec<String>>,
    back_flavor: Option<String>,
    back_text: Option<String>,
    backimagesrc: Option<String>,
    code: String,
    deck_limit: Option<i32>,
    deck_options: Option<Vec<DeckOption>>,
    deck_requirements: Option<DeckRequirements>,
    double_sided: bool,
    duplicated_by: Option<Vec<String>>,
    exceptional: bool,
    faction_code: String,
    faction_name: String,
    flavor: Option<String>,
    health: i32,
    health_per_investigator: bool,
    illustrator: String,
    imagesrc: Option<String>,
    is_unique: bool,
    myriad: bool,
    name: String,
    octgn_id: Option<String>,
    pack_code: String,
    pack_name: String,
    permanent: bool,
    position: i32,
    quantity: i32,
    real_name: String,
    real_slot: Option<String>,
    real_text: String,
    real_traits: String,
    sanity: i32,
    skill_agility: i32,
    skill_combat: i32,
    skill_intellect: i32,
    skill_willpower: i32,
    subname: String,
    text: String,
    traits: String,
    type_code: String,
    type_name: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeckOption {
    faction: Option<Vec<String>>,
    level: Option<Level>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Level {
    max: i32,
    min: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeckRequirements {
    card: std::collections::HashMap<String, std::collections::HashMap<String, String>>,
    random: Vec<RandomRequirement>,
    size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct RandomRequirement {
    target: String,
    value: String,
}

impl Investigator {
    pub fn to_string_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
