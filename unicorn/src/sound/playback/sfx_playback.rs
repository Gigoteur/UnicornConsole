use std::sync::Arc;

use gamercade_audio::ChainId;

use crate::{
    ChainPlayback, InstrumentInstance, Sfx, SoundRomInstance, TrackerFlow, TrackerOscillator,
    TrackerOscillatorFlow,
};

#[derive(Debug, Clone)]
pub struct SfxPlayback {
    pub(crate) oscillator: TrackerOscillator,
    pub(crate) chain_playback: ChainPlayback,
}

impl SfxPlayback {
    pub fn new(
        chain: Option<ChainId>,
        rom: &Arc<SoundRomInstance>,
        instrument: InstrumentInstance,
        output_sample_rate: usize,
    ) -> Self {
        Self {
            oscillator: TrackerOscillator::new(output_sample_rate),
            chain_playback: ChainPlayback::new(chain, rom, instrument),
        }
    }

    pub fn set_sfx_id(&mut self, sfx: Option<Sfx>) {
        if let Some(sfx) = sfx {
            self.chain_playback.set_chain_id(Some(sfx.chain));
            self.oscillator.reset_bpm(sfx.bpm);
        } else {
            self.chain_playback.set_chain_id(None);
        }
    }

    pub fn tick(&mut self) -> f32 {
        match self.oscillator.tick() {
            TrackerOscillatorFlow::Continue => (),
            TrackerOscillatorFlow::UpdateTracker => match self.chain_playback.update_tracker() {
                TrackerFlow::Advance => (),
                TrackerFlow::Finished => {
                    self.oscillator.stop();
                    self.chain_playback.set_chain_id(None);
                }
            },
        };

        self.chain_playback.phrase_playback.instrument.tick()
    }

    pub fn replace_sound_rom_instance(&mut self, new_rom: &Arc<SoundRomInstance>) {
        self.chain_playback.replace_sound_rom_instance(new_rom)
    }
}
