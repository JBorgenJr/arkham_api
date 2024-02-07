// use std::fmt;

// #[derive(Debug)]
// pub enum Cycle {
//     Core,
//     TheDunwichLegacy,
//     ThePathToCarcosa,
//     TheForgottenAge,
//     TheCircleUndone,
//     TheDreamEaters,
//     TheInnsMouthConspiracy,
//     EdgeOfTheEarth,
//     TheScarletKeys,
//     TheFeastOfHemlockVale,
//     ReturnTo,
//     SideStories,
//     Unknown,
// }

// impl Cycle {
//     pub fn from_str(s: &str) -> Option<Self> {
//         match s.to_lowercase().as_str() {
//             "core" => Some(Self::Core),
//             "dwl" => Some(Self::TheDunwichLegacy),
//             "ptc" => Some(Self::ThePathToCarcosa),
//             "tfa" => Some(Self::TheForgottenAge),
//             "tcu" => Some(Self::TheCircleUndone),
//             "tde" => Some(Self::TheDreamEaters),
//             "tic" => Some(Self::TheInnsMouthConspiracy),
//             "eoe" => Some(Self::EdgeOfTheEarth),
//             "tsk" => Some(Self::TheScarletKeys),
//             "fhv" => Some(Self::TheFeastOfHemlockVale),
//             "return" => Some(Self::ReturnTo),
//             "side_stories" => Some(Self::SideStories),
//             _ => Some(Self::Unknown),
//         }
//     }
// }

// impl fmt::Display for Cycle {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 Cycle::Core => "core",
//                 Cycle::TheDunwichLegacy => "dwl",
//                 Cycle::ThePathToCarcosa => "ptc",
//                 Cycle::TheForgottenAge => "tfa",
//                 Cycle::TheCircleUndone => "tcu",
//                 Cycle::TheDreamEaters => "tde",
//                 Cycle::TheInnsMouthConspiracy => "tic",
//                 Cycle::EdgeOfTheEarth => "eoe",
//                 Cycle::TheScarletKeys => "tsk",
//                 Cycle::TheFeastOfHemlockVale => "fhv",
//                 Cycle::ReturnTo => "return",
//                 Cycle::SideStories => "side_stories",
//                 Cycle::Unknown => "",
//             }
//         )
//     }
// }

// impl Cycle {
//     pub fn variants() -> Vec<&'static str> {
//         vec![
//             "core",
//             "dwl",
//             "ptc",
//             "tfa",
//             "tcu",
//             "tde",
//             "tic",
//             "eoe",
//             "tsk",
//             "fhv",
//             "return",
//             "side_stories",
//         ]
//     }
// }
