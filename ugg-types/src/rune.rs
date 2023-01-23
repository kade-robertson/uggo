#[derive(Clone)]
pub struct RuneExtended<T: Clone> {
    pub slot: i64,
    pub index: i64,
    pub siblings: i64,
    pub parent: String,
    pub parent_id: i64,
    pub rune: T,
}
