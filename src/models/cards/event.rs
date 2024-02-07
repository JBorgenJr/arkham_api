use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub pack_code: String,
    pub pack_name: String,
    pub type_code: String,
    pub type_name: String,
    pub faction_code: String,
    pub faction_name: String,
    pub position: u32,
    pub exceptional: bool,
    pub myriad: bool,
    pub code: String,
    pub name: String,
    pub real_name: String,
    pub cost: Option<i32>,
    pub text: String,
    pub real_text: String,
    pub errata_date: Option<ErrataDate>,
    pub quantity: u32,
    pub skill_intellect: Option<u32>,
    pub skill_agility: Option<u32>,
    pub skill_wild: Option<u32>,
    pub health_per_investigator: bool,
    pub deck_limit: Option<u32>,
    pub real_slot: Option<String>,
    pub traits: Option<String>,
    pub real_traits: Option<String>,
    pub restrictions: Option<Restrictions>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrataDate {
    pub date: String,
    pub timezone_type: u32,
    pub timezone: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Restrictions {
    pub investigator: HashMap<String, String>,
}

impl Event {
    pub fn to_string_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
