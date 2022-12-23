/// How many channels are available for a song.
pub const SONG_TRACK_CHANNELS: usize = 8;

/// How many channels are available for sfx.
pub const SFX_CHANNELS: usize = 8;

/// How many effects are available.
pub const EFFECT_COUNT: usize = 3;

/// Maximum allowed phrases in a chain
pub const CHAIN_MAX_PHRASE_COUNT: usize = 16;

/// Maximum allowed entries (or steps) in a phrase
pub const PHRASE_MAX_ENTRIES: usize = 16;

// I'm not sure why this is correct but it is
pub const PHRASE_STEPS_PER_BEAT: usize = 4;

use strum::EnumCount;

use crate::audio::notes::note_name::NoteName;
use crate::audio::notes::octave::Octave;

/// The total number of valid notes. 96 notes from C1 -> B9
pub const TOTAL_NOTES_COUNT: usize = (Octave::COUNT - 1) * NoteName::COUNT;

/// The maximum number of songs in the SoundRom
pub const SONGS_MAX_COUNT: usize = 256;

/// The maximum number of chain lists in a Song
pub const SONG_MAX_CHAIN_LENGTH: usize = 256;

/// The maximum number of chains in the SoundRom
pub const CHAINS_MAX_COUNT: usize = 256;

/// The maximum number of phrases in the SoundRom
pub const PHRASES_MAX_COUNT: usize = 256;

/// The maximum number of instruments in the SoundRom
pub const INSTRUMENTS_MAX_COUNT: usize = 256;

/// The maximum number of sfx in the SoundRom
pub const SFX_MAX_COUNT: usize = 256;

/// Maximum length of a wavetable
pub const WAVETABLE_MAX_LENGTH: usize = 2048;

pub const DEFAULT_BPM: f32 = 120.0;
