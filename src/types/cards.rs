use std::fmt;
// use crate::models;

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
    Unknown,
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
            _ => Some(Self::Unknown),
        }
    }
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CardType::Act => "act",
                CardType::Agenda => "agenda",
                CardType::Asset => "asset",
                CardType::Enemy => "enemy",
                CardType::Event => "event",
                CardType::Investigator => "investigator",
                CardType::Key => "key",
                CardType::Location => "location",
                CardType::Scenario => "scenario",
                CardType::Skill => "skill",
                CardType::Story => "story",
                CardType::Treachery => "treachery",
                CardType::Unknown => "",
            }
        )
    }
}

impl CardType {
    pub fn variants() -> Vec<&'static str> {
        vec![
            "act",
            "agenda",
            "asset",
            "enemy",
            "event",
            "investigator",
            "key",
            "location",
            "scenario",
            "skill",
            "story",
            "treachery",
        ]
    }
}

