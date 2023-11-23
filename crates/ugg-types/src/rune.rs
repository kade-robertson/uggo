#[derive(Clone)]
pub struct RuneExtended<T: Clone> {
    pub slot: u64,
    pub index: u64,
    pub siblings: u64,
    pub parent: String,
    pub parent_id: i64,
    pub rune: T,
}
