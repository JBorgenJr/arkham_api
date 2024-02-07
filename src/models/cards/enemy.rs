use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Enemy {
    pub pack_code: String,
    pub pack_name: String,
    pub type_code: String,
    pub type_name: String,
    pub subtype_code: Option<String>,
    pub subtype_name: Option<String>,
    pub faction_code: String,
    pub faction_name: String,
    pub position: u32,
    pub exceptional: bool,
    pub myriad: bool,
    pub code: String,
    pub name: String,
    pub real_name: String,
    pub text: Option<String>,
    pub real_text: Option<String>,
    pub quantity: u32,
    pub health: Option<i32>,
    pub health_per_investigator: bool,
    pub enemy_damage: Option<u32>,
    pub enemy_horror: Option<u32>,
    pub enemy_fight: Option<i32>,
    pub enemy_evade: Option<i32>,
    pub deck_limit: Option<u32>,
    pub real_slot: Option<String>,
    pub traits: String,
    pub real_traits: String,
    pub flavor: Option<String>,
    pub illustrator: String,
    pub is_unique: bool,
    pub permanent: bool,
    pub double_sided: bool,
    pub octgn_id: Option<String>,
    pub url: String,
    pub imagesrc: Option<String>,
    pub duplicated_by: Option<Vec<String>>,
}

impl Enemy {
    pub fn to_string_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }
}
