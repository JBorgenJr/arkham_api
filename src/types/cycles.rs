#[derive(Debug)]
pub enum Cycle {
    Core,
    TheDunwichLegacy,
    ThePathToCarcosa,
    TheForgottenAge,
    TheCircleUndone,
    TheDreamEaters,
    TheInnsmouthConspiracy,
    EdgeOfTheEarth,
    TheScarletKeys,
    TheFeastOfHemlockVale,
    Return,
    InvestigatorStarterDecks,
    SideStories,
    Promotional,
    Parallel,
}

// impl Cycle {
//     pub fn from_str(s: &str) -> Option<Self> {
//         match s.to_lowercase().as_str() {
//             "core" => Some(Self::Core),
//             "dwl" => Some(Self::TheDunwichLegacy),
//             "ptc" => Some(Self::ThePathToCarcosa),
//             "tfa" => Some(Self::TheForgottenAge),
//             "tcu" => Some(Self::TheCircleUndone),
//             "tde" => Some(Self::TheDreamEaters),
//             "tic" => Some(Self::TheInnsmouthConspiracy),
//             "eoe" => Some(Self::EdgeOfTheEarth),
//             "tsk" => Some(Self::TheScarletKeys),
//             "fhv" => Some(Self::TheFeastOfHemlockVale),
//             "return" => Some(Self::Return),
//             "investigator" => Some(Self::InvestigatorStarterDecks),
//             "side_stories" => Some(Self::SideStories),
//             "promotional" => Some(Self::Promotional),
//             "parallel" => Some(Self::Parallel),
//             _ => None,
//         }
//     }
// }

impl Cycle {
    pub fn all_cycles() -> Vec<&'static str> {
        let cycles: Vec<&str> = vec![
            "Core",
            "The Dunwich Legacy",
            "The Path to Carcosa",
            "The Forgotten Age",
            "The Circle Undone",
            "The Dream-Eaters",
            "The Innsmouth Conspiracy",
            "Edge of the Earth",
            "The Scarlet Keys",
            "The Feast of Hemlock Vale",
            "Return to...",
            "Investigator Starter Decks",
            "Side Stories",
            "Promotional",
            "Parallel",
        ];
        cycles
    }
}
