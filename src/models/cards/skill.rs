use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
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
    pub text: String,
    pub real_text: String,
    pub quantity: u32,
    pub skill_combat: Option<u32>,
    pub xp: Option<u32>,
    pub health_per_investigator: bool,
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

impl Skill {
    pub fn to_string_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }
}
