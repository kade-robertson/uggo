use std::{convert::TryFrom, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Rank {
    #[serde(rename = "1")]
    Challenger = 1,

    #[serde(rename = "2")]
    Master = 2,

    #[serde(rename = "3")]
    Diamond = 3,

    #[serde(rename = "4")]
    Platinum = 4,

    #[serde(rename = "5")]
    Gold = 5,

    #[serde(rename = "6")]
    Silver = 6,

    #[serde(rename = "7")]
    Bronze = 7,

    #[serde(rename = "8")]
    Overall = 8,

    #[serde(rename = "10")]
    PlatinumPlus = 10,

    #[serde(rename = "11")]
    DiamondPlus = 11,

    #[serde(rename = "12")]
    Iron = 12,

    #[serde(rename = "13")]
    Grandmaster = 13,

    #[serde(rename = "14")]
    MasterPlus = 14,

    #[serde(rename = "15")]
    Diamond2Plus = 15,

    #[serde(rename = "16")]
    Emerald = 16,

    #[serde(rename = "17")]
    EmeraldPlus = 17,
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rank_str = match self {
            Rank::Challenger => "Challenger",
            Rank::Master => "Master",
            Rank::Diamond => "Diamond",
            Rank::Platinum => "Platinum",
            Rank::Gold => "Gold",
            Rank::Silver => "Silver",
            Rank::Bronze => "Bronze",
            Rank::Overall => "Overall",
            Rank::PlatinumPlus => "PlatinumPlus",
            Rank::DiamondPlus => "DiamondPlus",
            Rank::Iron => "Iron",
            Rank::Grandmaster => "Grandmaster",
            Rank::MasterPlus => "MasterPlus",
            Rank::Diamond2Plus => "Diamond2Plus",
            Rank::Emerald => "Emerald",
            Rank::EmeraldPlus => "EmeraldPlus",
        };
        write!(f, "{rank_str}")
    }
}

impl Rank {
    pub fn preferred_order() -> [Rank; 16] {
        // Prefer Platinum+, then overall, the plus ranks from lowest to highest,
        // and finally individual ranks.
        [
            Rank::PlatinumPlus,
            Rank::Overall,
            Rank::EmeraldPlus,
            Rank::DiamondPlus,
            Rank::Diamond2Plus,
            Rank::MasterPlus,
            Rank::Iron,
            Rank::Bronze,
            Rank::Silver,
            Rank::Gold,
            Rank::Platinum,
            Rank::Emerald,
            Rank::Diamond,
            Rank::Master,
            Rank::Grandmaster,
            Rank::Challenger,
        ]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Region {
    #[serde(rename = "1")]
    NA1 = 1,

    #[serde(rename = "2")]
    EUW1,

    #[serde(rename = "3")]
    KR,

    #[serde(rename = "4")]
    EUN1,

    #[serde(rename = "5")]
    BR1,

    #[serde(rename = "6")]
    LA1,

    #[serde(rename = "7")]
    LA2,

    #[serde(rename = "8")]
    OC1,

    #[serde(rename = "9")]
    RU,

    #[serde(rename = "10")]
    TR1,

    #[serde(rename = "11")]
    JP1,

    #[serde(rename = "12")]
    World,

    #[serde(rename = "13")]
    PH2,

    #[serde(rename = "14")]
    SG2,

    #[serde(rename = "15")]
    TH2,

    #[serde(rename = "16")]
    TW2,

    #[serde(rename = "17")]
    VN2,
}

impl Region {
    pub fn to_api_string(self) -> String {
        (match self {
            Region::NA1 => 1,
            Region::EUW1 => 2,
            Region::KR => 3,
            Region::EUN1 => 4,
            Region::BR1 => 5,
            Region::LA1 => 6,
            Region::LA2 => 7,
            Region::OC1 => 8,
            Region::RU => 9,
            Region::TR1 => 10,
            Region::JP1 => 11,
            Region::World => 12,
            Region::PH2 => 13,
            Region::SG2 => 14,
            Region::TH2 => 15,
            Region::TW2 => 16,
            Region::VN2 => 17,
        })
        .to_string()
    }
}

impl Default for Region {
    fn default() -> Self {
        Self::World
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let region_str = match self {
            Region::NA1 => "NA1",
            Region::EUW1 => "EUW1",
            Region::KR => "KR",
            Region::EUN1 => "EUN1",
            Region::BR1 => "BR1",
            Region::LA1 => "LA1",
            Region::LA2 => "LA2",
            Region::OC1 => "OC1",
            Region::RU => "RU",
            Region::TR1 => "TR1",
            Region::JP1 => "JP1",
            Region::World => "World",
            Region::PH2 => "PH2",
            Region::SG2 => "SG2",
            Region::TH2 => "TH2",
            Region::TW2 => "TW2",
            Region::VN2 => "VN2",
        };
        write!(f, "{region_str}")
    }
}

pub fn get_region(region: &str) -> Region {
    for enum_region in &[
        Region::NA1,
        Region::EUW1,
        Region::KR,
        Region::EUN1,
        Region::BR1,
        Region::LA1,
        Region::LA2,
        Region::OC1,
        Region::RU,
        Region::TR1,
        Region::JP1,
        Region::World,
        Region::PH2,
        Region::SG2,
        Region::TH2,
        Region::TW2,
        Region::VN2,
    ] {
        let region_str = enum_region.to_string().to_lowercase();
        if region.to_lowercase() == region_str
            || region_str.contains(&region.to_lowercase()[..region.len() - 1])
        {
            return *enum_region;
        }
    }
    Region::World
}

impl FromStr for Region {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(get_region(s))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Role {
    #[serde(rename = "1")]
    Jungle = 1,

    #[serde(rename = "2")]
    Support,

    #[serde(rename = "3")]
    ADCarry,

    #[serde(rename = "4")]
    Top,

    #[serde(rename = "5")]
    Mid,

    #[serde(rename = "6")]
    None,

    #[serde(rename = "7")]
    Automatic,
}

impl Default for Role {
    fn default() -> Self {
        Self::Automatic
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role_str = match self {
            Role::Jungle => "Jungle",
            Role::Support => "Support",
            Role::ADCarry => "ADCarry",
            Role::Top => "Top",
            Role::Mid => "Mid",
            Role::None => "None",
            Role::Automatic => "Automatic",
        };
        write!(f, "{role_str}")
    }
}

impl TryFrom<i32> for Role {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Jungle),
            2 => Ok(Self::Support),
            3 => Ok(Self::ADCarry),
            4 => Ok(Self::Top),
            5 => Ok(Self::Mid),
            6 => Ok(Self::None),
            _ => Err(()),
        }
    }
}

pub fn get_role(role: &str) -> Role {
    for enum_role in &[
        Role::Jungle,
        Role::Support,
        Role::ADCarry,
        Role::Top,
        Role::Mid,
        Role::None,
        Role::Automatic,
    ] {
        let role_str = enum_role.to_string().to_lowercase();
        if role_str.contains(&role.to_lowercase()) {
            return *enum_role;
        }
    }
    Role::Automatic
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(get_role(s))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Mode {
    Normal,
    ARAM,
    OneForAll,
    URF,
    ARURF,
    NexusBlitz,
}

impl Mode {
    pub fn to_api_string(self) -> String {
        (match self {
            Self::Normal => "ranked_solo_5x5",
            Self::ARAM => "normal_aram",
            Self::OneForAll => "one_for_all",
            Self::URF => "pick_urf",
            Self::ARURF => "urf",
            Self::NexusBlitz => "nexus_blitz",
        })
        .to_string()
    }

    pub fn from_api_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "normal_aram" => Self::ARAM,
            "one_for_all" => Self::OneForAll,
            "pick_urf" => Self::URF,
            "urf" => Self::ARURF,
            "nexus_blitz" => Self::NexusBlitz,
            _ => Self::Normal,
        }
    }

    pub fn all() -> &'static [Mode; 6] {
        &[
            Mode::Normal,
            Mode::ARAM,
            Mode::OneForAll,
            Mode::URF,
            Mode::ARURF,
            Mode::NexusBlitz,
        ]
    }
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        (match &self {
            Self::Normal => "Normal",
            Self::ARAM => "ARAM",
            Self::OneForAll => "OneForAll",
            Self::URF => "URF",
            Self::ARURF => "ARURF",
            Self::NexusBlitz => "NexusBlitz",
        })
        .to_string()
    }
}

impl From<&str> for Mode {
    fn from(mode_str: &str) -> Self {
        match mode_str.to_lowercase().as_str() {
            "aram" | "all_random_all_mid" | "normal_aram" => Self::ARAM,
            "oneforall" | "one_for_all" => Self::OneForAll,
            "urf" | "ultra_rapid_fire" => Self::URF,
            "arurf" => Self::ARURF,
            "nexus_blitz" | "nexusblitz" => Self::NexusBlitz,
            _ => Self::Normal,
        }
    }
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Mode::from(s))
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_role() {
        assert_eq!(get_role("top"), Role::Top);
        assert_eq!(get_role("mid"), Role::Mid);
        assert_eq!(get_role("sup"), Role::Support);
        assert_eq!(get_role("Adc"), Role::ADCarry);
        assert_eq!(get_role("jungle"), Role::Jungle);
    }
}
