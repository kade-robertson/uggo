use std::convert::TryFrom;

use clap::{builder::PossibleValue, ValueEnum};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(
    Copy, Clone, Display, EnumString, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug,
)]
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

#[derive(
    Copy, Clone, Display, EnumString, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug,
)]
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

pub fn get_region(region: &str) -> Region {
    for enum_region in Region::iter() {
        let region_str = enum_region.to_string().to_lowercase();
        if region.to_lowercase() == region_str
            || region_str.contains(&region.to_lowercase()[..region.len() - 1])
        {
            return enum_region;
        }
    }
    Region::NA1
}

impl ValueEnum for Region {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::NA1,
            Self::EUW1,
            Self::KR,
            Self::EUN1,
            Self::BR1,
            Self::LA1,
            Self::LA2,
            Self::OC1,
            Self::RU,
            Self::TR1,
            Self::JP1,
            Self::World,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::NA1 => PossibleValue::new("na1").alias("NA1"),
            Self::EUW1 => PossibleValue::new("euw1").alias("EUW1"),
            Self::KR => PossibleValue::new("kr").alias("KR"),
            Self::EUN1 => PossibleValue::new("eun1").alias("EUN1"),
            Self::BR1 => PossibleValue::new("br1").alias("BR1"),
            Self::LA1 => PossibleValue::new("la1").alias("LA1"),
            Self::LA2 => PossibleValue::new("la2").alias("LA2"),
            Self::OC1 => PossibleValue::new("oc1").alias("OC1"),
            Self::RU => PossibleValue::new("run").alias("RU"),
            Self::TR1 => PossibleValue::new("tr1").alias("TR1"),
            Self::JP1 => PossibleValue::new("jp1").alias("JP1"),
            Self::World => PossibleValue::new("world").alias("World"),
        })
    }
}

#[derive(
    Copy, Clone, Display, EnumString, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug,
)]
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
    for enum_role in Role::iter() {
        let role_str = enum_role.to_string().to_lowercase();
        if role_str.contains(&role.to_lowercase()) {
            return enum_role;
        }
    }
    Role::Automatic
}

impl ValueEnum for Role {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Jungle,
            Self::Support,
            Self::ADCarry,
            Self::Top,
            Self::Mid,
            Self::None,
            Self::Automatic,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Jungle => PossibleValue::new("jungle").alias("Jungle"),
            Self::Support => PossibleValue::new("support").alias("Support"),
            Self::ADCarry => PossibleValue::new("ad-carry")
                .alias("adcarry")
                .alias("ADCarry"),
            Self::Top => PossibleValue::new("top").alias("Top"),
            Self::Mid => PossibleValue::new("mid").alias("Mid"),
            Self::None => PossibleValue::new("none").alias("None"),
            Self::Automatic => PossibleValue::new("automatic").alias("Automatic"),
        })
    }
}

#[derive(Clone, Copy, Debug, EnumIter)]
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

impl ValueEnum for Mode {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Normal, Self::ARAM, Self::OneForAll, Self::URF]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Normal => PossibleValue::new("normal").alias("Normal"),
            Self::ARAM => PossibleValue::new("aram").alias("ARAM"),
            Self::OneForAll => PossibleValue::new("one-for-all").alias("OneForAll"),
            Self::URF => PossibleValue::new("urf")
                .alias("URF")
                .alias("ultra-rapid-fire"),
        })
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
