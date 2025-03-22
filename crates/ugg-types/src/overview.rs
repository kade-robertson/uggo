// Credit to https://github.com/pradishb/ugg-parser for figuring out the
// structure of the champ overview stats data.

use crate::arena_overview::ArenaOverviewData;
use crate::default_overview::{Abilities, OverviewData};
use crate::mappings;
use serde::de::{Deserialize, Deserializer, IgnoredAny, SeqAccess, Visitor};
use serde::{Deserialize as DeserializeDerive, Serialize};
use std::collections::HashMap;
use std::fmt;

pub type ChampOverview = HashMap<
    mappings::Region,
    HashMap<mappings::Rank, HashMap<mappings::Role, WrappedOverviewData>>,
>;

pub fn handle_unknown<T: Default, E>(result: Result<Option<T>, E>) -> T {
    result.ok().flatten().unwrap_or_default()
}

#[derive(Debug, Clone, Serialize, DeserializeDerive)]
#[serde(untagged)]
pub enum Overview {
    Default(OverviewData),
    Arena(ArenaOverviewData),
}

impl Overview {
    #[must_use]
    pub fn matches(&self) -> i64 {
        match self {
            Overview::Arena(a) => a.matches,
            Overview::Default(d) => d.matches,
        }
    }

    #[must_use]
    pub fn abilities(&self) -> Abilities {
        match self {
            Overview::Arena(a) => a.abilities.clone(),
            Overview::Default(d) => d.abilities.clone(),
        }
    }

    #[must_use]
    pub fn low_sample_size(&self) -> bool {
        match self {
            Overview::Arena(a) => a.low_sample_size,
            Overview::Default(d) => d.low_sample_size,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct WrappedOverviewData {
    pub data: Overview,
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
                match visitor.next_element::<Overview>() {
                    Ok(Some(data)) => {
                        while let Some(IgnoredAny) = visitor.next_element()? {}
                        Ok(WrappedOverviewData { data })
                    }
                    Err(e) => Err(e),
                    _ => Err(serde::de::Error::custom("No more data left.")),
                }
            }
        }

        deserializer.deserialize_seq(WrappedOverviewDataVisitor)
    }
}
