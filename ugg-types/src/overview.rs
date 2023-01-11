// Credit to https://github.com/pradishb/ugg-parser for figuring out the
// structure of the champ overview stats data.

use crate::mappings;
use serde::de::{Deserialize, Deserializer, IgnoredAny, SeqAccess, Visitor};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;

pub type ChampOverview = HashMap<
    mappings::Region,
    HashMap<mappings::Rank, HashMap<mappings::Role, WrappedOverviewData>>,
>;

#[cfg(not(feature = "client"))]
fn handle_unknown<T: Default, E>(result: Result<Option<T>, E>) -> T {
    result
        .unwrap_or_else(|_| Some(T::default()))
        .unwrap_or_default()
}

#[derive(Debug, Clone)]
pub struct WrappedOverviewData {
    pub data: OverviewData,
}

impl<'de> Deserialize<'de> for WrappedOverviewData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WrappedOverviewDataVisitor;

        impl<'de> Visitor<'de> for WrappedOverviewDataVisitor {
            type Value = WrappedOverviewData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("waa")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<WrappedOverviewData, V::Error>
            where
                V: SeqAccess<'de>,
            {
                match visitor.next_element::<OverviewData>() {
                    Ok(Some(data)) => {
                        while let Some(IgnoredAny) = visitor.next_element()? {}
                        Ok(WrappedOverviewData { data })
                    }
                    _ => Err(serde::de::Error::missing_field("top-level element")),
                }
            }
        }

        deserializer.deserialize_seq(WrappedOverviewDataVisitor)
    }
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[derive(Debug, Clone, Serialize)]
pub struct OverviewData {
    pub runes: Runes,
    pub summoner_spells: SummonerSpells,
    pub starting_items: Items,
    pub core_items: Items,
    pub abilities: Abilities,
    pub item_4_options: Vec<LateItem>,
    pub item_5_options: Vec<LateItem>,
    pub item_6_options: Vec<LateItem>,
    pub wins: i64,
    pub matches: i64,
    pub low_sample_size: bool,
    pub shards: Shards,
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[derive(Debug, Clone, Serialize)]
pub struct Runes {
    pub matches: i64,
    pub wins: i64,
    pub primary_style_id: i64,
    pub secondary_style_id: i64,
    pub rune_ids: Vec<i64>,
}

#[cfg(not(feature = "client"))]
impl<'de> Deserialize<'de> for Runes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RunesVisitor;

        impl<'de> Visitor<'de> for RunesVisitor {
            type Value = Runes;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("rune stats")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<Runes, V::Error>
            where
                V: SeqAccess<'de>,
            {
                Ok(Runes {
                    matches: handle_unknown(visitor.next_element::<i64>()),
                    wins: handle_unknown(visitor.next_element::<i64>()),
                    primary_style_id: handle_unknown(visitor.next_element::<i64>()),
                    secondary_style_id: handle_unknown(visitor.next_element::<i64>()),
                    rune_ids: handle_unknown(visitor.next_element::<Vec<i64>>()),
                })
            }
        }

        deserializer.deserialize_seq(RunesVisitor)
    }
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[derive(Debug, Clone, Serialize)]
pub struct SummonerSpells {
    pub matches: i64,
    pub wins: i64,
    pub spell_ids: Vec<i64>,
}

#[cfg(not(feature = "client"))]
impl<'de> Deserialize<'de> for SummonerSpells {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SummonerSpellsVisitor;

        impl<'de> Visitor<'de> for SummonerSpellsVisitor {
            type Value = SummonerSpells;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("summoner spells")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<SummonerSpells, V::Error>
            where
                V: SeqAccess<'de>,
            {
                Ok(SummonerSpells {
                    matches: handle_unknown(visitor.next_element::<i64>()),
                    wins: handle_unknown(visitor.next_element::<i64>()),
                    spell_ids: handle_unknown(visitor.next_element::<Vec<i64>>()),
                })
            }
        }

        deserializer.deserialize_seq(SummonerSpellsVisitor)
    }
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[derive(Debug, Clone, Serialize)]
pub struct Items {
    pub matches: i64,
    pub wins: i64,
    pub item_ids: Vec<i64>,
}

#[cfg(not(feature = "client"))]
impl<'de> Deserialize<'de> for Items {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ItemsVisitor;

        impl<'de> Visitor<'de> for ItemsVisitor {
            type Value = Items;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("items")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<Items, V::Error>
            where
                V: SeqAccess<'de>,
            {
                Ok(Items {
                    matches: handle_unknown(visitor.next_element::<i64>()),
                    wins: handle_unknown(visitor.next_element::<i64>()),
                    item_ids: handle_unknown(visitor.next_element::<Vec<i64>>()),
                })
            }
        }

        deserializer.deserialize_seq(ItemsVisitor)
    }
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[derive(Debug, Clone, Serialize)]
pub struct Abilities {
    pub matches: i64,
    pub wins: i64,
    pub ability_order: Vec<char>,
    pub ability_max_order: String,
}

#[cfg(not(feature = "client"))]
impl<'de> Deserialize<'de> for Abilities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AbilitiesVisitor;

        impl<'de> Visitor<'de> for AbilitiesVisitor {
            type Value = Abilities;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("abilities")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<Abilities, V::Error>
            where
                V: SeqAccess<'de>,
            {
                Ok(Abilities {
                    matches: handle_unknown(visitor.next_element::<i64>()),
                    wins: handle_unknown(visitor.next_element::<i64>()),
                    ability_order: handle_unknown(visitor.next_element::<Vec<char>>()),
                    ability_max_order: handle_unknown(visitor.next_element::<String>()),
                })
            }
        }

        deserializer.deserialize_seq(AbilitiesVisitor)
    }
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[derive(Debug, Clone, Serialize)]
pub struct LateItem {
    pub matches: i64,
    pub wins: i64,
    pub id: i64,
}

#[cfg(not(feature = "client"))]
impl<'de> Deserialize<'de> for LateItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LateItemVisitor;

        impl<'de> Visitor<'de> for LateItemVisitor {
            type Value = LateItem;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("summoner spells")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<LateItem, V::Error>
            where
                V: SeqAccess<'de>,
            {
                Ok(LateItem {
                    id: handle_unknown(visitor.next_element::<i64>()),
                    wins: handle_unknown(visitor.next_element::<i64>()),
                    matches: handle_unknown(visitor.next_element::<i64>()),
                })
            }
        }

        deserializer.deserialize_seq(LateItemVisitor)
    }
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[derive(Debug, Clone, Serialize)]
pub struct Shards {
    pub matches: i64,
    pub wins: i64,
    pub shard_ids: Vec<i64>,
}

#[cfg(not(feature = "client"))]
impl<'de> Deserialize<'de> for Shards {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AbilitiesVisitor;

        impl<'de> Visitor<'de> for AbilitiesVisitor {
            type Value = Shards;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("shards")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<Shards, V::Error>
            where
                V: SeqAccess<'de>,
            {
                Ok(Shards {
                    matches: handle_unknown(visitor.next_element::<i64>()),
                    wins: handle_unknown(visitor.next_element::<i64>()),
                    shard_ids: handle_unknown(visitor.next_element::<Vec<String>>())
                        .iter()
                        .map(|x| x.parse::<i64>().unwrap_or_default())
                        .collect::<Vec<i64>>(),
                })
            }
        }

        deserializer.deserialize_seq(AbilitiesVisitor)
    }
}

#[cfg(not(feature = "client"))]
impl<'de> Deserialize<'de> for OverviewData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OverviewDataVisitor;

        impl<'de> Visitor<'de> for OverviewDataVisitor {
            type Value = OverviewData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("overview data")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<OverviewData, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let runes = visitor.next_element::<Runes>().unwrap().unwrap();
                let summoner_spells = visitor.next_element::<SummonerSpells>().unwrap().unwrap();

                let starting_items = visitor.next_element::<Items>().unwrap().unwrap();
                let core_items = visitor.next_element::<Items>().unwrap().unwrap();
                let abilities = visitor.next_element::<Abilities>().unwrap().unwrap();
                let late_items = visitor
                    .next_element::<Vec<Vec<LateItem>>>()
                    .unwrap()
                    .unwrap();
                let match_info = handle_unknown(visitor.next_element::<Vec<i64>>());
                let low_sample_size = match_info[1] < 1000;

                // this is the original low sample size value, it's always false though, so ignore.
                if visitor.next_element::<serde_json::Value>().is_ok() {}

                let shards = visitor.next_element::<Shards>().ok().unwrap().unwrap();

                // this array is never used?
                if visitor.next_element::<serde_json::Value>().is_ok() {}

                let overview_data = OverviewData {
                    runes,
                    summoner_spells,
                    starting_items,
                    core_items,
                    abilities,
                    item_4_options: late_items[0].clone(),
                    item_5_options: late_items[1].clone(),
                    item_6_options: late_items[2].clone(),
                    wins: match_info[0],
                    matches: match_info[1],
                    low_sample_size,
                    shards,
                };
                Ok(overview_data)
            }
        }

        deserializer.deserialize_seq(OverviewDataVisitor)
    }
}
