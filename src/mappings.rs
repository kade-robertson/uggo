use std::convert::TryFrom;

use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Copy, Clone, Display, EnumString, EnumIter, PartialEq)]
pub enum Rank {
    Challenger = 1,
    Master = 2,
    Diamond = 3,
    Platinum = 4,
    Gold = 5,
    Silver = 6,
    Bronze = 7,
    Overall = 8,
    PlatinumPlus = 10,
    DiamondPlus = 11,
    Iron = 12,
    Grandmaster = 13,
    MasterPlus = 14,
    Diamond2Plus = 15,
}

pub fn rank_to_str(rank: Rank) -> String {
    return (rank as i32).to_string();
}

#[derive(Copy, Clone, Display, EnumString, EnumIter, PartialEq)]
pub enum Region {
    NA1 = 1,
    EUW1,
    KR,
    EUN1,
    BR1,
    LA1,
    LA2,
    OC1,
    RU,
    TR1,
    JP1,
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
    return Region::NA1;
}

#[derive(Copy, Clone, Display, EnumString, EnumIter, PartialEq)]
pub enum Role {
    Jungle = 1,
    Support,
    ADCarry,
    Top,
    Mid,
    None,
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

pub fn role_to_str(role: Role) -> String {
    return (role as i32).to_string();
}

pub fn get_role(role: &str) -> Role {
    for enum_role in Role::iter() {
        let role_str = enum_role.to_string().to_lowercase();
        if role_str.contains(&role.to_lowercase()) {
            return enum_role;
        }
    }
    return Role::Automatic;
}

#[derive(Copy, Clone, Display, EnumString, EnumIter, PartialEq)]
pub enum Mode {
    #[strum(serialize = "ranked_solo_5x5", serialize = "normal")]
    Normal,

    #[strum(serialize = "normal_aram", serialize = "aram")]
    ARAM,

    #[strum(serialize = "one_for_all", serialize = "oneforall")]
    OneForAll,
}
