#[derive(Debug)]
pub enum CardType {
    Act,
    Agenda,
    Asset,
    Enemy,
    Event,
    Investigator,
    Key,
    Location,
    Scenario,
    Skill,
    Story,
    Treachery,
}

impl CardType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "act" => Some(Self::Act),
            "agenda" => Some(Self::Agenda),
            "asset" => Some(Self::Asset),
            "enemy" => Some(Self::Enemy),
            "event" => Some(Self::Event),
            "investigator" => Some(Self::Investigator),
            "key" => Some(Self::Key),
            "location" => Some(Self::Location),
            "scenario" => Some(Self::Scenario),
            "skill" => Some(Self::Skill),
            "story" => Some(Self::Story),
            "treachery" => Some(Self::Treachery),
            _ => None,
        }
    }
}

impl CardType {
    pub fn all_types() -> Vec<&'static str> {
        let types: Vec<&str> = vec![
            "Act",
            "Agenda",
            "Asset",
            "Enemy",
            "Event",
            "Investigator",
            "Key",
            "Location",
            "Scenario",
            "Skill",
            "Story",
            "Treachery",
        ];
        types
    }
}
