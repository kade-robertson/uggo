use std::{convert::TryFrom, str::FromStr};

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
            1 => Ok(Role::Jungle),
            2 => Ok(Role::Support),
            3 => Ok(Role::ADCarry),
            4 => Ok(Role::Top),
            5 => Ok(Role::Mid),
            6 => Ok(Role::None),
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

#[derive(Copy, Clone, Display, EnumString, EnumIter, PartialEq, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Mode {
    #[strum(serialize = "ranked_solo_5x5", serialize = "normal")]
    Normal,

    #[strum(serialize = "normal_aram", serialize = "aram")]
    ARAM,

    #[strum(serialize = "one_for_all", serialize = "oneforall")]
    OneForAll,

    #[strum(serialize = "urf")]
    URF,
}

pub fn get_mode(mode: &str) -> Mode {
    match Mode::from_str(mode) {
        Ok(variant) => variant,
        Err(_) => match Mode::from_str(&mode.to_lowercase()) {
            Ok(variant) => variant,
            Err(_) => Mode::Normal,
        },
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
