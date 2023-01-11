use crate::mappings;

pub trait NestedData<T> {
    fn is_region_valid(&self, region: &mappings::Region) -> bool;
    fn is_rank_valid(&self, region: &mappings::Region, rank: &mappings::Rank) -> bool;
    fn is_role_valid(
        &self,
        region: &mappings::Region,
        rank: &mappings::Rank,
        role: &mappings::Role,
    ) -> bool;
    fn get_most_popular_role(
        &self,
        region: &mappings::Region,
        rank: &mappings::Rank,
    ) -> Option<mappings::Role>;
    fn get_wrapped_data(
        &self,
        region: &mappings::Region,
        rank: &mappings::Rank,
        role: &mappings::Role,
    ) -> Option<T>;
}
