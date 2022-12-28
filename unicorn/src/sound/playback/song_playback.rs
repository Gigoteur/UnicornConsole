use std::sync::Arc;

use crate::audio::tracker::song::SongId;
use crate::audio::consts::SONG_TRACK_CHANNELS;

use crate::sound::playback::chain_playback::ChainPlayback;
use crate::sound::sound_rom_instance::SoundRomInstance;
use crate::sound::playback::tracker_flow::TrackerFlow;
use crate::sound::playback::tracker_oscillator::{TrackerOscillator, TrackerOscillatorFlow};

#[derive(Debug, Clone)]
pub struct SongPlayback {
    pub song: Option<SongId>,
    pub(crate) chain_index: usize, // The current location in the song
    pub tracks: [ChainPlayback; SONG_TRACK_CHANNELS],
    pub(crate) chain_states: [TrackerFlow; SONG_TRACK_CHANNELS],
    pub(crate) rom: Arc<SoundRomInstance>,

    oscillator: TrackerOscillator,
}

fn default_chain_states() -> [TrackerFlow; SONG_TRACK_CHANNELS] {
    std::array::from_fn(|_| TrackerFlow::Advance)
}

impl SongPlayback {
    pub(crate) fn new(
        song: Option<SongId>,
        tracks: [ChainPlayback; SONG_TRACK_CHANNELS],
        rom: &Arc<SoundRomInstance>,
        output_sample_rate: usize,
    ) -> Self {
        let mut out = Self {
            song,
            chain_index: 0,
            tracks,
            rom: rom.clone(),
            chain_states: default_chain_states(),
            oscillator: TrackerOscillator::new(output_sample_rate),
        };

        out.set_song_id(song);
        out
    }

    pub(crate) fn tick(&mut self) -> [f32; SONG_TRACK_CHANNELS] {
        match self.oscillator.tick() {
            TrackerOscillatorFlow::Continue => (),
            TrackerOscillatorFlow::UpdateTracker => {
                self.update_tracker(); //TODO: Should we handle this output?
            }
        };

        let mut iter = self.tracks.iter_mut();

        std::array::from_fn(|_| iter.next().unwrap().phrase_playback.instrument.tick())
    }

    /// Sets this playback to play specified Song Id.
    /// Passing in None will mute the playback.
    pub(crate) fn set_song_id(&mut self, song: Option<SongId>) {
        self.song = song;
        self.chain_index = 0;

        // If the song is valid, update all chains to
        // use the correct indices and data
        if let Some(song) = song {
            let song = &self.rom[song];
            self.oscillator.reset_bpm(song.bpm);
            let next_chain = song.tracks[0];
            self.chain_states = default_chain_states();
            self.tracks
                .iter_mut()
                .zip(next_chain.iter())
                .for_each(|(track, next)| {
                    track.set_chain_id(*next);
                });
        } else {
            // Otherwise, just stop all of the playbacks
            self.tracks.iter_mut().for_each(|track| {
                track.set_chain_id(None);
            });
        }
    }

    /// Calls update_tracker on each chain playback,
    /// if all are done, will increment our current chain index
    /// within the song
    pub(crate) fn update_tracker(&mut self) -> TrackerFlow {
        // Call update on each of the chains, but
        // only if they should continue playing
        self.tracks
            .iter_mut()
            .zip(self.chain_states.iter_mut())
            .for_each(|(tracker, state)| {
                if TrackerFlow::Advance == *state {
                    *state = tracker.update_tracker()
                }
            });

        if self
            .chain_states
            .iter()
            .all(|state| *state == TrackerFlow::Finished)
        {
            self.next_step()
        } else {
            TrackerFlow::Advance
        }
    }

    /// Advances the tracks to the next chain within the song.
    pub(crate) fn next_step(&mut self) -> TrackerFlow {
        // Song doesn't exist, so we're done
        if self.song.is_none() {
            return TrackerFlow::Finished;
        };
        let song = self.song.unwrap();

        self.chain_index += 1;

        // Song doesn't have any more entries, so we're done
        let next_chain = self.rom[song].tracks.get(self.chain_index);
        if next_chain.is_none() {
            return TrackerFlow::Finished;
        }

        let next_chain = next_chain.unwrap();

        self.tracks
            .iter_mut()
            .zip(self.chain_states.iter_mut().zip(next_chain.iter()))
            .for_each(|(track, (state, next))| {
                *state = TrackerFlow::Advance;
                track.set_chain_id(*next);
            });

        TrackerFlow::Advance
    }

    pub(crate) fn replace_sound_rom_instance(&mut self, new_rom: &Arc<SoundRomInstance>) {
        self.rom = new_rom.clone();

        self.tracks
            .iter_mut()
            .for_each(|track| track.replace_sound_rom_instance(new_rom));
    }
}
