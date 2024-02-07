use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Scenario {
    pub pack_code: String,
    pub pack_name: String,
    pub type_code: String,
    pub type_name: String,
    pub faction_code: String,
    pub faction_name: String,
    pub encounter_code: String,
    pub encounter_name: String,
    pub position: Option<u32>,
    pub exceptional: bool,
    pub myriad: bool,
    pub encounter_position: Option<u32>,
    pub code: String,
    pub name: String,
    pub real_name: String,
    pub text: String,
    pub real_text: String,
    pub quantity: u32,
    pub health_per_investigator: bool,
    pub real_slot: Option<String>,
    pub is_unique: bool,
    pub permanent: bool,
    pub double_sided: bool,
    pub back_text: Option<String>,
    pub octgn_id: String,
    pub url: String,
    pub imagesrc: Option<String>,
    pub spoiler: u32,
    pub backimagesrc: Option<String>,
}

impl Scenario {
    pub fn to_string_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }
}
