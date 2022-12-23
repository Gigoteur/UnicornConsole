#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrackerFlow {
    /// Move to the next index
    Advance,

    /// Nothing else left to play
    Finished,
}
