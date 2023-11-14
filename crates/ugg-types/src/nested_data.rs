use std::collections::HashMap;

use serde_json::Value;

use crate::mappings;

pub trait NestedData<T> {
    fn is_region_valid(&self, region: &mappings::Region) -> bool;
    fn is_rank_valid(&self, region: &mappings::Region, rank: &mappings::Rank) -> bool;
    fn get_wrapped_data(&self, region: &mappings::Region, rank: &mappings::Rank) -> Option<T>;
}

impl NestedData<Value> for HashMap<mappings::Region, HashMap<mappings::Rank, Value>> {
    fn is_region_valid(&self, region: &mappings::Region) -> bool {
        self.contains_key(region)
    }

    fn is_rank_valid(&self, region: &mappings::Region, rank: &mappings::Rank) -> bool {
        self.get(region).map_or(false, |rd| rd.contains_key(rank))
    }

    fn get_wrapped_data(&self, region: &mappings::Region, rank: &mappings::Rank) -> Option<Value> {
        self.get(region).and_then(|rg| rg.get(rank).cloned())
    }
}

pub trait GroupedData<T> {
    fn is_role_valid(&self, role: &mappings::Role) -> bool;
    fn get_most_popular_role(&self) -> Option<mappings::Role>;
    fn get_wrapped_data(&self, role: &mappings::Role) -> Option<T>;
}
