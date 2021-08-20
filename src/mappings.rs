use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Display, EnumString, EnumIter, PartialEq)]
pub enum Rank {
    Challenger = 1,
    Master,
    Diamond,
    Platinum,
    Gold,
    Silver,
    Bronze,
    Overall,
    PlatinumPlus,
    DiamondPlus,
    Iron,
    Grandmaster,
    MasterPlus,
    Diamond2Plus,
}

#[derive(Display, EnumString, EnumIter, PartialEq)]
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

#[derive(Display, EnumString, EnumIter, PartialEq)]
pub enum Role {
    Jungle = 1,
    Support,
    ADCarry,
    Top,
    Mid,
    None,
    Automatic,
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
