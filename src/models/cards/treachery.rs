use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Treachery {
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
    pub text: String,
    pub real_text: String,
    pub quantity: u32,
    pub health_per_investigator: bool,
    pub deck_limit: Option<u32>,
    pub real_slot: Option<String>,
    pub traits: Option<String>,
    pub real_traits: Option<String>,
    pub restrictions: Option<Restrictions>,
    pub illustrator: Option<String>,
    pub is_unique: bool,
    pub permanent: bool,
    pub double_sided: bool,
    pub octgn_id: Option<String>,
    pub url: String,
    pub imagesrc: Option<String>,
    pub duplicated_by: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Restrictions {
    pub investigator: HashMap<String, String>,
}

impl Treachery {
    pub fn to_string_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
