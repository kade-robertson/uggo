// Credit to https://github.com/pradishb/ugg-parser for figuring out the
// structure of the champ overview stats data.

use serde::Serialize;
use serde::de::{Deserialize, Deserializer, IgnoredAny, SeqAccess, Visitor};
use std::fmt;

use crate::overview::handle_unknown;

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

#[derive(Debug, Clone, Serialize)]
pub struct Runes {
    pub matches: i64,
    pub wins: i64,
    pub primary_style_id: i64,
    pub secondary_style_id: i64,
    pub rune_ids: Vec<i64>,
}

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

#[derive(Debug, Clone, Serialize)]
pub struct SummonerSpells {
    pub matches: i64,
    pub wins: i64,
    pub spell_ids: Vec<i64>,
}

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

#[derive(Debug, Clone, Serialize)]
pub struct Items {
    pub matches: i64,
    pub wins: i64,
    pub item_ids: Vec<i64>,
}

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

#[derive(Debug, Clone, Serialize)]
pub struct Abilities {
    pub matches: i64,
    pub wins: i64,
    pub ability_order: Vec<char>,
    pub ability_max_order: String,
}

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

#[derive(Debug, Clone, Serialize)]
pub struct LateItem {
    pub matches: i64,
    pub wins: i64,
    pub id: i64,
}

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

#[derive(Debug, Clone, Serialize, Default)]
pub struct Shards {
    pub matches: i64,
    pub wins: i64,
    pub shard_ids: Vec<i64>,
}

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
                let runes = visitor
                    .next_element::<Runes>()?
                    .ok_or(serde::de::Error::custom("Could not parse runes."))?;
                let summoner_spells = visitor
                    .next_element::<SummonerSpells>()?
                    .ok_or(serde::de::Error::custom("Could not parse summoner spells."))?;
                let starting_items = visitor
                    .next_element::<Items>()?
                    .ok_or(serde::de::Error::custom("Could not parse starting items."))?;
                let core_items = visitor
                    .next_element::<Items>()?
                    .ok_or(serde::de::Error::custom("Could not parse core items."))?;
                let abilities = visitor
                    .next_element::<Abilities>()?
                    .ok_or(serde::de::Error::custom("Could not parse abilities."))?;
                let late_items = visitor
                    .next_element::<Vec<Vec<LateItem>>>()?
                    .ok_or(serde::de::Error::custom("Could not parse late items."))?;
                let match_info = visitor
                    .next_element::<Vec<i64>>()
                    .unwrap_or_default()
                    .unwrap_or_default();
                let low_sample_size = match_info.get(1).is_some_and(|games| *games < 1000);

                // this is the original low sample size value, it's always false though, so ignore.
                let _ = visitor.next_element::<IgnoredAny>().is_ok();

                let shards = visitor
                    .next_element::<Shards>()
                    .unwrap_or_default()
                    .unwrap_or_default();

                // Don't know what this is yet
                while let Some(IgnoredAny) = visitor.next_element()? {}

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
