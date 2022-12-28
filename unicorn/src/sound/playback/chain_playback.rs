use std::sync::Arc;

use crate::audio::tracker::chain::ChainId;

use crate::sound::instruments::instrument_instance::InstrumentInstance;
use crate::sound::playback::phrase_playback::PhrasePlayback;
use crate::sound::sound_rom_instance::SoundRomInstance;
use crate::sound::playback::tracker_flow::TrackerFlow;

#[derive(Debug, Clone)]
pub struct ChainPlayback {
    pub(crate) rom: Arc<SoundRomInstance>,
    pub(crate) phrase_index: usize,
    pub(crate) chain: Option<ChainId>,
    pub(crate) phrase_playback: PhrasePlayback,
}

impl ChainPlayback {
    pub fn new(
        chain: Option<ChainId>,
        rom: &Arc<SoundRomInstance>,
        instrument: InstrumentInstance,
    ) -> Self {
        let mut out = Self {
            rom: rom.clone(),
            phrase_index: 0,
            chain,
            phrase_playback: PhrasePlayback::new(None, rom, instrument),
        };

        out.set_chain_id(chain);
        out
    }

    /// Sets the active chain ID for this playback
    /// and potentially affect the nested instrument.
    /// This will additionally set the reset phrase index to zero.
    pub fn set_chain_id(&mut self, chain: Option<ChainId>) {
        self.chain = chain;
        self.phrase_index = 0;

        let phrase_id =
            chain.and_then(|chain| self.rom[chain].as_ref().and_then(|chain| chain.entries[0]));

        self.phrase_playback.set_phrase_id(phrase_id);
    }

    /// Calls update_tracker on the phrase playback,
    /// if its done, will increment our current phrase index
    /// within the chain
    pub fn update_tracker(&mut self) -> TrackerFlow {
        match self.phrase_playback.update_tracker() {
            TrackerFlow::Advance => TrackerFlow::Advance,
            TrackerFlow::Finished => self.next_step(),
        }
    }

    /// Advances the chain to the next phrase within the chain.
    fn next_step(&mut self) -> TrackerFlow {
        if let Some(chain) = self.chain {
            self.phrase_index += 1;

            let next_phrase = self.rom[chain]
                .as_ref()
                .and_then(|chain| chain.entries.get(self.phrase_index).and_then(|x| *x));

            if next_phrase.is_some() {
                self.phrase_playback.set_phrase_id(next_phrase);
                TrackerFlow::Advance
            } else {
                TrackerFlow::Finished
            }
        } else {
            TrackerFlow::Finished
        }
    }

    pub fn replace_sound_rom_instance(&mut self, new_rom: &Arc<SoundRomInstance>) {
        self.rom = new_rom.clone();

        self.phrase_playback.replace_sound_rom_instance(new_rom);
    }
}
