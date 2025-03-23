// Credit to https://github.com/pradishb/ugg-parser for figuring out the
// structure of the champ overview stats data.

use crate::mappings;
use serde::Serialize;
use serde::de::{Deserialize, Deserializer, IgnoredAny, SeqAccess, Visitor};
use std::collections::HashMap;
use std::fmt;

pub type Matchups =
    HashMap<mappings::Region, HashMap<mappings::Rank, HashMap<mappings::Role, WrappedMatchupData>>>;

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

#[derive(Debug, Clone, Serialize)]
pub struct MatchupData {
    pub best_matchups: Vec<Matchup>,
    pub worst_matchups: Vec<Matchup>,
    pub total_matches: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct Matchup {
    pub champion_id: i64,
    pub wins: i32,
    pub matches: i32,
    pub winrate: f64,
}

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
                let mut total_matches: i32 = 0;

                while let Ok(data_opt) = visitor.next_element::<InnerData>() {
                    match data_opt {
                        Some(data) => {
                            let wins = data.2 - data.1;
                            let winrate = f64::from(wins) / f64::from(data.2);
                            all_matchups.push(Matchup {
                                champion_id: data.0,
                                wins,
                                matches: data.2,
                                winrate,
                            });
                            total_matches += data.2;
                        }
                        None => {
                            break;
                        }
                    }
                }

                // Only consider matchups that represent at least a 0.5% possibility of showing up
                all_matchups = all_matchups
                    .into_iter()
                    .filter(|a| f64::from(a.matches) >= (f64::from(total_matches) / 200.0))
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

struct InnerData(i64, i32, i32);

impl<'de> Deserialize<'de> for InnerData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InnerSeqVisitor;

        impl<'de> Visitor<'de> for InnerSeqVisitor {
            type Value = InnerData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence with at least 3 elements")
            }

            fn visit_seq<A>(self, mut visitor: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let champion_id = visitor
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                let losses = visitor
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

                let matches = visitor
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;

                while let Some(IgnoredAny) = visitor.next_element()? {}

                Ok(InnerData(champion_id, losses, matches))
            }
        }

        deserializer.deserialize_seq(InnerSeqVisitor)
    }
}
