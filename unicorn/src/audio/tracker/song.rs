use serde::{Deserialize, Serialize};

use audio::tracker::chain::{Chain, ChainId};

use crate::audio::consts::{DEFAULT_BPM, PHRASE_STEPS_PER_BEAT, SONG_TRACK_CHANNELS};

#[derive(Debug, Clone, Copy)]
pub struct SongId(pub usize);

/// A song is a series of chains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub bpm: f32,
    pub tracks: Box<[[Option<ChainId>; SONG_TRACK_CHANNELS]]>,
}

impl Default for Song {
    fn default() -> Self {
        Self {
            bpm: DEFAULT_BPM,
            tracks: vec![std::array::from_fn(|_| None)].into_boxed_slice(),
        }
    }
}

impl Song {
    pub fn song_length_seconds(&self, chains: &[Option<Chain>]) -> f32 {
        let mut sum = 0.0;
        let empty_pattern_length = (60.0 / self.bpm) * PHRASE_STEPS_PER_BEAT as f32;

        for row in self.tracks.iter() {
            let row_max = row
                .iter()
                .map(|lane| {
                    lane.and_then(|chain| {
                        chains.get(chain.0).and_then(|chain| {
                            chain
                                .as_ref()
                                .map(|chain| chain.chain_length_seconds(self.bpm))
                        })
                    })
                    .unwrap_or(empty_pattern_length)
                })
                .reduce(f32::max);

            if let Some(row_max) = row_max {
                sum += row_max
            } else {
                break;
            }
        }

        sum
    }
}
