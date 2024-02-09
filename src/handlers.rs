use crate::{
    handlers,
    models::cards::*,
    service::{create_card_file, save_card_to_file},
    types::cards::CardType,
};

use serde_json::{Error, Value};

pub fn get_card_handler(card_code: CardType) -> Box<dyn handlers::CardHandler> {
    match card_code {
        CardType::Act => Box::new(handlers::ActHandler {}),
        CardType::Agenda => Box::new(handlers::AgendaHandler {}),
        CardType::Asset => Box::new(handlers::AssetHandler {}),
        CardType::Enemy => Box::new(handlers::EnemyHandler {}),
        CardType::Event => Box::new(handlers::EventHandler {}),
        CardType::Investigator => Box::new(handlers::InvestigatorHandler {}),
        CardType::Key => Box::new(handlers::KeyHandler {}),
        CardType::Location => Box::new(handlers::LocationHandler {}),
        CardType::Scenario => Box::new(handlers::ScenarioHandler {}),
        CardType::Skill => Box::new(handlers::SkillHandler {}),
        CardType::Story => Box::new(handlers::StoryHandler {}),
        CardType::Treachery => Box::new(handlers::TreacheryHandler {}),
        _ => Box::new(handlers::DefaultHander),
    }
}

pub trait CardHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error>;
}

pub struct DefaultHander;
impl CardHandler for DefaultHander {
    fn handle_card(&self, _card: Value) -> Result<(), Error> {
        println!("Unknown card type: {:?}", _card.get("type_code").unwrap());
        Ok(())
    }
}

pub struct ActHandler;
impl CardHandler for ActHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let act: act::Act = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let act_value = act.to_string_pretty();
        save_card_to_file(path, act_value?);
        Ok(())
    }
}

pub struct AgendaHandler;
impl CardHandler for AgendaHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let agenda: agenda::Agenda = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let agenda_value = agenda.to_string_pretty();
        save_card_to_file(path, agenda_value?);
        Ok(())
    }
}

pub struct AssetHandler;
impl CardHandler for AssetHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let asset: asset::Asset = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let asset_value = asset.to_string_pretty();
        save_card_to_file(path, asset_value?);
        Ok(())
    }
}

pub struct EnemyHandler;
impl CardHandler for EnemyHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let enemy: enemy::Enemy = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let enemy_value = enemy.to_string_pretty();
        save_card_to_file(path, enemy_value?);
        Ok(())
    }
}

pub struct EventHandler;
impl CardHandler for EventHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let event: event::Event = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let event_value = event.to_string_pretty();
        save_card_to_file(path, event_value?);
        Ok(())
    }
}

pub struct InvestigatorHandler;
impl CardHandler for InvestigatorHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let investigator: investigator::Investigator = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let investigator_value = investigator.to_string_pretty();
        save_card_to_file(path, investigator_value?);
        Ok(())
    }
}

pub struct KeyHandler;
impl CardHandler for KeyHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let key: key::Key = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let key_value = key.to_string_pretty();
        save_card_to_file(path, key_value?);
        Ok(())
    }
}

pub struct LocationHandler;
impl CardHandler for LocationHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let location: location::Location = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let location_value = location.to_string_pretty();
        save_card_to_file(path, location_value?);
        Ok(())
    }
}

pub struct ScenarioHandler;
impl CardHandler for ScenarioHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let scenario: scenario::Scenario = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let scenario_value = scenario.to_string_pretty();
        save_card_to_file(path, scenario_value?);
        Ok(())
    }
}

pub struct SkillHandler;
impl CardHandler for SkillHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let skill: skill::Skill = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let skill_value = skill.to_string_pretty();
        save_card_to_file(path, skill_value?);
        Ok(())
    }
}

pub struct StoryHandler;
impl CardHandler for StoryHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let story: story::Story = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let story_value = story.to_string_pretty();
        save_card_to_file(path, story_value?);
        Ok(())
    }
}

pub struct TreacheryHandler;
impl CardHandler for TreacheryHandler {
    fn handle_card(&self, card: Value) -> Result<(), Error> {
        let treachery: treachery::Treachery = serde_json::from_value(card.clone())?;
        let path = create_card_file(card);
        let treachery_value = treachery.to_string_pretty();
        save_card_to_file(path, treachery_value?);
        Ok(())
    }
}
