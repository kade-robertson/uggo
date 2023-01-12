#![allow(clippy::cast_precision_loss)]

// Credit to https://github.com/pradishb/ugg-parser for figuring out the
// structure of the champ overview stats data.

use crate::mappings;
use crate::nested_data::GroupedData;
use serde::de::{Deserialize, Deserializer, IgnoredAny, SeqAccess, Visitor};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

pub type GroupedMatchupData = HashMap<mappings::Role, WrappedMatchupData>;

pub type Matchups = HashMap<mappings::Region, HashMap<mappings::Rank, Value>>;

impl GroupedData<WrappedMatchupData> for GroupedMatchupData {
    fn is_role_valid(&self, role: &mappings::Role) -> bool {
        self.contains_key(role)
    }

    fn get_most_popular_role(&self) -> Option<mappings::Role> {
        self.iter()
            .max_by(|a, b| a.1.data.total_matches.cmp(&b.1.data.total_matches))
            .map(|(r, _)| *r)
    }

    fn get_wrapped_data(&self, role: &mappings::Role) -> Option<WrappedMatchupData> {
        self.get(role).cloned()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct WrappedMatchupData {
    pub data: MatchupData,
}

impl<'de> Deserialize<'de> for WrappedMatchupData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WrappedMatchupDataVisitor;

        impl<'de> Visitor<'de> for WrappedMatchupDataVisitor {
            type Value = WrappedMatchupData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("waa")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<WrappedMatchupData, V::Error>
            where
                V: SeqAccess<'de>,
            {
                match visitor.next_element::<MatchupData>() {
                    Ok(Some(data)) => {
                        while let Some(IgnoredAny) = visitor.next_element()? {}
                        Ok(WrappedMatchupData { data })
                    }
                    _ => Err(serde::de::Error::missing_field("top-level element")),
                }
            }
        }

        deserializer.deserialize_seq(WrappedMatchupDataVisitor)
    }
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[derive(Debug, Clone, Serialize)]
pub struct MatchupData {
    pub best_matchups: Vec<Matchup>,
    pub worst_matchups: Vec<Matchup>,
    pub total_matches: i64,
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[derive(Debug, Clone, Serialize)]
pub struct Matchup {
    pub champion_id: i64,
    pub wins: i64,
    pub matches: i64,
    pub winrate: f64,
}

#[cfg(not(feature = "client"))]
impl<'de> Deserialize<'de> for MatchupData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MatchupDataVisitor;

        impl<'de> Visitor<'de> for MatchupDataVisitor {
            type Value = MatchupData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("overview data")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<MatchupData, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut all_matchups: Vec<Matchup> = vec![];
                let mut total_matches: i64 = 0;

                while let Ok(data_opt) = visitor.next_element::<Vec<i64>>() {
                    match data_opt {
                        Some(data) => {
                            let wins = data[2] - data[1];
                            let winrate = wins as f64 / data[2] as f64;
                            all_matchups.push(Matchup {
                                champion_id: data[0],
                                wins,
                                matches: data[2],
                                winrate,
                            });
                            total_matches += data[2];
                        }
                        None => {
                            break;
                        }
                    }
                }

                // Only consider matchups that represent at least a 0.5% possibility of showing up
                all_matchups = all_matchups
                    .into_iter()
                    .filter(|a| a.matches as f64 >= (total_matches as f64 / 200.0))
                    .collect::<Vec<Matchup>>();
                all_matchups.sort_by(|a, b| b.winrate.partial_cmp(&a.winrate).unwrap());

                if all_matchups.len() >= 5 {
                    let best_matchups: Vec<Matchup> = all_matchups.clone()[..5].to_vec();
                    let mut worst_matchups: Vec<Matchup> =
                        all_matchups.clone()[all_matchups.len() - 5..].to_vec();
                    worst_matchups.reverse();

                    let matchup_data = MatchupData {
                        best_matchups,
                        worst_matchups,
                        total_matches,
                    };
                    Ok(matchup_data)
                } else {
                    Ok(MatchupData {
                        best_matchups: vec![],
                        worst_matchups: vec![],
                        total_matches: 0,
                    })
                }
            }
        }

        deserializer.deserialize_seq(MatchupDataVisitor)
    }
}
