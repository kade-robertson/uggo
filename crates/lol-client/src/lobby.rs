use serde::{Deserialize, Serialize};
use ugg_types::mappings::Mode;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lobby {
    pub game_config: GameConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QueueID {
    Unrecognized = -999,
    Normal = 400,
    RankedSoloDuo = 420,
    NormalBlind = 430,
    RankedFlex = 440,
    ARAM = 450,
    NormalQuickplay = 490,
    Clash = 700,
    ClashAlt = 701,
    ARAMClash = 720,
    ARAMClashAlt = 721,
    IntroBot = 830,
    BeginnerBot = 840,
    IntermediateBot = 850,
    ARAMBots = 860,
    ARURF = 900,
    OneForAll = 1020,
    NexusBlitz = 1300,
    UltimateSpellbook = 1400,
    URF = 1900,
}

impl Default for QueueID {
    fn default() -> Self {
        Self::Unrecognized
    }
}

impl QueueID {
    pub fn into_mode(&self) -> Option<Mode> {
        use QueueID::*;

        match self {
            Normal | RankedSoloDuo | RankedFlex | NormalBlind | NormalQuickplay | Clash
            | ClashAlt | IntroBot | BeginnerBot | IntermediateBot => Some(Mode::Normal),
            ARAM | ARAMClash | ARAMClashAlt | ARAMBots => Some(Mode::ARAM),
            OneForAll => Some(Mode::OneForAll),
            URF => Some(Mode::URF),
            ARURF => Some(Mode::ARURF),
            NexusBlitz => Some(Mode::NexusBlitz),
            _ => None,
        }
    }

    pub fn matches(&self, mode: Mode) -> bool {
        self.into_mode() == Some(mode)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameConfig {
    #[serde(default)]
    pub queue_id: QueueID,
}
