use serde::{Deserialize, Serialize};

use std::ops::Range;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoopMode {
    Oneshot,
    Loop,
    LoopRange(Range<usize>),
}
