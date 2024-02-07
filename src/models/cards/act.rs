use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Act {
    pub back_flavor: Option<String>,
    pub back_name: Option<String>,
    pub back_text: Option<String>,
    pub backimagesrc: Option<String>,
    pub code: String,
    pub doom: Option<u32>,
    pub double_sided: bool,
    pub encounter_code: String,
    pub encounter_name: String,
    pub encounter_position: u32,
    pub exceptional: bool,
    pub faction_code: String,
    pub faction_name: String,
    pub flavor: Option<String>,
    pub health_per_investigator: bool,
    pub illustrator: Option<String>,
    pub imagesrc: Option<String>,
    pub is_unique: bool,
    pub myriad: bool,
    pub name: String,
    pub octgn_id: Option<String>,
    pub pack_code: String,
    pub pack_name: String,
    pub permanent: bool,
    pub position: u32,
    pub quantity: u32,
    pub real_name: String,
    pub real_slot: Option<String>,
    pub real_text: Option<String>,
    pub spoiler: u32,
    pub stage: u32,
    pub text: Option<String>,
    pub type_code: String,
    pub type_name: String,
    pub url: String,
    pub clues: Option<i32>,
}

impl Act {
    pub fn to_string_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
