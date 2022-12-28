use std::sync::Arc;

use crate::audio::tracker::phrase::PhraseId;
use crate::audio::consts::PHRASE_MAX_ENTRIES;

use crate::sound::instruments::instrument_instance::InstrumentInstance;
use crate::sound::sound_rom_instance::SoundRomInstance;
use crate::sound::playback::tracker_flow::TrackerFlow;
use crate::sound::instruments::instrument_instance::new_instrument_channel_message;

#[derive(Debug, Clone)]
pub struct PhrasePlayback {
    pub(crate) rom: Arc<SoundRomInstance>,
    pub(crate) step_index: usize,
    pub(crate) phrase: Option<PhraseId>,
    pub(crate) instrument: InstrumentInstance,
}

impl PhrasePlayback {
    pub(crate) fn new(
        phrase: Option<PhraseId>,
        rom: &Arc<SoundRomInstance>,
        instrument: InstrumentInstance,
    ) -> Self {
        let mut out = Self {
            step_index: 0,
            phrase,
            rom: rom.clone(),
            instrument,
        };
        out.update_instrument();
        out
    }

    /// Sets the active phrase ID for this playback
    /// and notifies the sound thread.
    pub(crate) fn set_phrase_id(&mut self, phrase: Option<PhraseId>) {
        self.phrase = phrase;
        self.step_index = 0;

        self.update_instrument();
    }

    /// Updates the instrument with new frequency, effects, id etc
    fn update_instrument(&mut self) -> Option<()> {
        let phrase_id = self.phrase?;
        let next_entry = self.rom[phrase_id].as_ref()?;
        let next_entry = next_entry.entries[self.step_index].as_ref()?;
        let msg = new_instrument_channel_message(next_entry, &self.rom)?;
        self.instrument.update_from_tracker(&msg);
        Some(())
    }

    /// Increments the index and notifies the sound thread
    pub(crate) fn update_tracker(&mut self) -> TrackerFlow {
        self.step_index += 1;

        if self.step_index >= PHRASE_MAX_ENTRIES {
            self.step_index = 0;
            TrackerFlow::Finished
        } else {
            self.update_instrument();
            TrackerFlow::Advance
        }
    }

    pub(crate) fn replace_sound_rom_instance(&mut self, new_rom: &Arc<SoundRomInstance>) {
        self.rom = new_rom.clone();
        self.update_instrument();
    }
}
