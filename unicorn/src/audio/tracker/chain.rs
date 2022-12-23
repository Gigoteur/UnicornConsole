use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

use crate::audio::consts::{CHAIN_MAX_PHRASE_COUNT, PHRASE_STEPS_PER_BEAT};

use super::phrase::PhraseId;

/// Newtype Chain Identifier
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct ChainId(pub usize);

/// A chain is a series of phrases, which when combined together form a song.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub entries: ArrayVec<Option<PhraseId>, CHAIN_MAX_PHRASE_COUNT>,
}

impl Chain {
    pub fn count(&self) -> usize {
        let mut out = 0;
        let mut iter = self.entries.iter();
        while let Some(true) = iter.next().map(Option::is_some) {
            out += 1;
        }
        out
    }

    pub fn is_empty(&self) -> bool {
        self.entries[0].is_some()
    }

    pub fn chain_length_seconds(&self, bpm: f32) -> f32 {
        self.count() as f32 * (60.0 / bpm) * PHRASE_STEPS_PER_BEAT as f32
    }
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            entries: ArrayVec::from(std::array::from_fn(|_| None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Chain;
    use crate::{PhraseId, CHAIN_MAX_PHRASE_COUNT};
    use arrayvec::ArrayVec;

    #[test]
    fn test_chain_count_empty() {
        let chain = Chain::default();

        assert_eq!(chain.count(), 0);
    }

    #[test]
    fn test_chain_count_full() {
        let chain = Chain {
            entries: ArrayVec::from(std::array::from_fn(|_| Some(PhraseId(0)))),
        };

        assert_eq!(chain.count(), CHAIN_MAX_PHRASE_COUNT);
    }

    #[test]
    fn test_chain_count_separated() {
        let chain = Chain {
            entries: ArrayVec::from_iter((0..8_usize).map(|index| {
                if index == 3 || index == 5 {
                    None
                } else {
                    Some(PhraseId(0))
                }
            })),
        };
        assert_eq!(chain.count(), 3);
    }
}
