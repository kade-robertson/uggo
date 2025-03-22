use std::fmt;

use serde::Serialize;
use serde::de::{Deserialize, Deserializer, IgnoredAny, SeqAccess, Visitor};

use crate::default_overview::{Abilities, Items, LateItem};
use crate::overview::handle_unknown;

#[derive(Debug, Clone, Serialize)]
pub struct ArenaOverviewData {
    pub starting_items: Items,
    pub core_items: Items,
    pub abilities: Abilities,
    pub item_4_options: Vec<LateItem>,
    pub item_5_options: Vec<LateItem>,
    pub item_6_options: Vec<LateItem>,
    pub consumables: Vec<LateItem>,
    pub prismatic_items: Vec<PrismaticItem>,
    pub wins: i64,
    pub matches: i64,
    pub low_sample_size: bool,
    pub champion_synergies: Vec<ChampionSynergy>,
    pub augments: Vec<Augment>,
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct PrismaticItem {
    pub id: i64,
    pub matches: i64,
    pub wins: i64,
}

impl PrismaticItem {
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn winrate(&self) -> f64 {
        if self.matches <= 0 {
            return 0f64;
        }
        (self.wins as f64) / (self.matches as f64)
    }
}

impl PartialOrd for PrismaticItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PrismaticItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.winrate().total_cmp(&other.winrate())
    }
}

impl<'de> Deserialize<'de> for PrismaticItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PrismaticItemVisitor;

        impl<'de> Visitor<'de> for PrismaticItemVisitor {
            type Value = PrismaticItem;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("prismatic items")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<PrismaticItem, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let id = handle_unknown(visitor.next_element::<i64>());

                // no clue what these two are
                let _ = visitor.next_element::<IgnoredAny>().is_ok();
                let _ = visitor.next_element::<IgnoredAny>().is_ok();

                let wins = handle_unknown(visitor.next_element::<i64>());
                let matches = handle_unknown(visitor.next_element::<i64>());
                Ok(PrismaticItem { id, matches, wins })
            }
        }

        deserializer.deserialize_seq(PrismaticItemVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct ChampionSynergy {
    id: i64,
    top_four: i64,
    picked: i64,
    first: i64,
    sum_of_placements: i64,
}

impl ChampionSynergy {
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn top_four_rate(&self) -> f64 {
        if self.picked <= 0 {
            return 0f64;
        }
        (self.top_four as f64) / (self.picked as f64)
    }
}

impl PartialOrd for ChampionSynergy {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ChampionSynergy {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.top_four_rate().total_cmp(&other.top_four_rate())
    }
}

impl<'de> Deserialize<'de> for ChampionSynergy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChampionSynergiesVisitor;

        impl<'de> Visitor<'de> for ChampionSynergiesVisitor {
            type Value = ChampionSynergy;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("champion synergy")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<ChampionSynergy, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let id = handle_unknown(visitor.next_element::<i64>());
                let top_four = handle_unknown(visitor.next_element::<i64>());
                let picked = handle_unknown(visitor.next_element::<i64>());
                let first = handle_unknown(visitor.next_element::<i64>());
                let sum_of_placements = handle_unknown(visitor.next_element::<i64>());

                Ok(ChampionSynergy {
                    id,
                    top_four,
                    picked,
                    first,
                    sum_of_placements,
                })
            }
        }

        deserializer.deserialize_seq(ChampionSynergiesVisitor)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Augment {
    id: i64,
    wins: i64,
    matches: i64,
}

impl Augment {
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn winrate(&self) -> f64 {
        if self.matches <= 0 {
            return 0f64;
        }
        (self.wins as f64) / (self.matches as f64)
    }
}

impl PartialOrd for Augment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Augment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.winrate().total_cmp(&other.winrate())
    }
}

impl<'de> Deserialize<'de> for Augment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AugmentVisitor;

        impl<'de> Visitor<'de> for AugmentVisitor {
            type Value = Augment;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("augment")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<Augment, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let id = handle_unknown(visitor.next_element::<i64>());
                let wins = handle_unknown(visitor.next_element::<i64>());
                let matches = handle_unknown(visitor.next_element::<i64>());

                Ok(Augment { id, wins, matches })
            }
        }

        deserializer.deserialize_seq(AugmentVisitor)
    }
}

impl<'de> Deserialize<'de> for ArenaOverviewData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArenaOverviewDataVisitor;

        impl<'de> Visitor<'de> for ArenaOverviewDataVisitor {
            type Value = ArenaOverviewData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("overview data")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<ArenaOverviewData, V::Error>
            where
                V: SeqAccess<'de>,
            {
                // winrate + extra stuff?
                let _ = visitor.next_element::<IgnoredAny>().is_ok();

                // winrate + extra stuff?
                let _ = visitor.next_element::<IgnoredAny>().is_ok();

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
                    .next_element::<(
                        Vec<LateItem>,
                        Vec<LateItem>,
                        Vec<LateItem>,
                        Vec<LateItem>,
                        Vec<PrismaticItem>,
                        // just a random array?
                        IgnoredAny,
                    )>()?
                    .ok_or(serde::de::Error::custom(
                        "Could not parse late / prismatic items.",
                    ))?;

                // Prismatic items are ordered by pickrate, not winrate
                // reorder them
                let mut prismatic_items = late_items.4;
                prismatic_items.sort_by(|a, b| b.cmp(a));

                let match_info = visitor.next_element::<(i64, i64)>()?.unwrap_or_default();
                let low_sample_size = match_info.1 < 1000;

                // this is the original low sample size value, it's always false though, so ignore.
                let _ = visitor.next_element::<IgnoredAny>().is_ok();

                // I think this is where shards would end up? It's junk data.
                let _ = visitor.next_element::<IgnoredAny>().is_ok();

                // empty array
                let _ = visitor.next_element::<IgnoredAny>().is_ok();

                // These are ordered by pick rate which seems like a bad default
                // reorder them by their top 4 rate instead.
                let mut champion_synergies = visitor
                    .next_element::<Vec<ChampionSynergy>>()?
                    .unwrap_or_default();
                champion_synergies.sort_by(|a, b| b.cmp(a));

                // Augments are ordered by pickrate in the UI, but winrate
                // in the data. Pick rate is presumably influenced by the tier
                // of the augment. Sort by winrate, but consider bucketing these
                // by tier if the information is available.
                let mut augments = visitor.next_element::<Vec<Augment>>()?.unwrap_or_default();
                augments.sort_by(|a, b| b.cmp(a));

                // Don't know what this is yet
                while let Some(IgnoredAny) = visitor.next_element()? {}

                let arena_overview_data = ArenaOverviewData {
                    starting_items,
                    core_items,
                    abilities,
                    item_4_options: late_items.0,
                    item_5_options: late_items.1,
                    item_6_options: late_items.2,
                    consumables: late_items.3,
                    prismatic_items,
                    wins: match_info.0,
                    matches: match_info.1,
                    low_sample_size,
                    champion_synergies,
                    augments,
                };
                Ok(arena_overview_data)
            }
        }

        deserializer.deserialize_seq(ArenaOverviewDataVisitor)
    }
}
