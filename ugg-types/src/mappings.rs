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
        };
        write!(f, "{rank_str}")
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

#[derive(Clone, Copy, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Mode {
    Normal,
    ARAM,
    OneForAll,
    URF,
}

impl Mode {
    pub fn to_api_string(self) -> String {
        (match self {
            Self::Normal => "ranked_solo_5x5",
            Self::ARAM => "normal_aram",
            Self::OneForAll => "one_for_all",
            Self::URF => "pick_urf",
        })
        .to_string()
    }

    pub fn all() -> &'static [Mode; 4] {
        &[Mode::Normal, Mode::ARAM, Mode::OneForAll, Mode::URF]
    }
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        (match &self {
            Self::Normal => "Normal",
            Self::ARAM => "ARAM",
            Self::OneForAll => "OneForAll",
            Self::URF => "URF",
        })
        .to_string()
    }
}

impl From<&str> for Mode {
    fn from(mode_str: &str) -> Self {
        match mode_str.to_lowercase().as_str() {
            "aram" | "all_random_all_mid" | "ranked_aram" => Self::ARAM,
            "oneforall" | "one_for_all" => Self::OneForAll,
            "urf" | "ultra_rapid_fire" => Self::URF,
            _ => Self::Normal,
        }
    }
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "aram" | "all_random_all_mid" | "ranked_aram" => Ok(Self::ARAM),
            "oneforall" | "one_for_all" => Ok(Self::OneForAll),
            "urf" | "ultra_rapid_fire" => Ok(Self::URF),
            _ => Ok(Self::Normal),
        }
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
