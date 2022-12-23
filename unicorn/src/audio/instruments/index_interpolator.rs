use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
/// Determines how to interpolate fractional values when doing any
/// kind of sample lookup. Since the output may require an index at point
/// 1.5, we need to know how to convert this value into a non-fractional integer.
pub enum IndexInterpolator {
    /// Linearly interpolates between the values
    /// A value of 1.2 will take the samples at 1 and 2,
    /// and combine the result which is (20% of 1) plus (80% of 2).
    /// A value of 10.5 will take 50% of the values, summed, at samples 10 and 11.
    #[default]
    Linear,

    /// Uses only the integral part of the number
    /// 1.2 will be truncated 1
    /// 1.5 will be truncated to 1
    /// 1.9 will be truncated to 1
    Truncate,

    /// Rounds to the nearest neighbor.
    /// 1.2 will round to 1
    /// 1.5 will round to 2
    /// 1.7 will round to 2
    NearestNeighbor,
}

pub enum IndexInterpolatorResult {
    Single(usize),
    Multiple(ArrayVec<(usize, f32), 2>),
}

impl IndexInterpolator {
    pub fn get_indices(self, index: f32, table_length: usize) -> IndexInterpolatorResult {
        match self {
            IndexInterpolator::Linear => {
                let fract = index.fract();
                let index = index as usize;
                let first = (index % table_length, 1.0 - fract);
                let second = ((index + 1) % table_length, fract);
                IndexInterpolatorResult::Multiple(ArrayVec::from([first, second]))
            }
            IndexInterpolator::Truncate => {
                IndexInterpolatorResult::Single(index.trunc() as usize % table_length)
            }
            IndexInterpolator::NearestNeighbor => {
                IndexInterpolatorResult::Single(index.round() as usize % table_length)
            }
        }
    }
}
