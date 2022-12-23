use gamercade_audio::{
    get_note, to_scaled_value, InstrumentId, NoteId, PhraseEntry, PhraseStorageType,
    PhraseVolumeType,
};

use crate::{
    InstrumentDefinition, InstrumentDefinitionKind, PatchInstance, SamplerInstance,
    SoundRomInstance, WavetableInstance,
};

#[derive(Debug, Clone)]
pub struct InstrumentInstance {
    id: usize,
    kind: InstrumentInstanceKind,
    volume: PhraseVolumeType,
}

#[derive(Debug, Clone)]
pub enum InstrumentInstanceKind {
    Wavetable(WavetableInstance),
    FMSynth(Box<PatchInstance>),
    Sampler(SamplerInstance),
}

pub type InstrumentChannelType = PhraseEntry<f32, InstrumentDefinition>;

pub fn new_instrument_channel_message(
    entry: &PhraseStorageType,
    rom: &SoundRomInstance,
) -> Option<InstrumentChannelType> {
    if let Some(instrument) = &rom[entry.instrument] {
        let note = get_note(entry.note).frequency;
        let instrument = instrument.clone();

        Some(InstrumentChannelType {
            note,
            volume: entry.volume,
            instrument,
            effects: entry.effects.clone(),
        })
    } else {
        None
    }
}

impl InstrumentInstance {
    pub(crate) fn no_sound(output_sample_rate: usize) -> Self {
        Self {
            id: usize::MAX,
            kind: InstrumentInstanceKind::Wavetable(WavetableInstance::no_sound(
                output_sample_rate,
            )),
            volume: 0,
        }
    }

    pub fn force_refresh_instrument(&mut self, rom: &SoundRomInstance) {
        if let Some(instrument) = &rom[InstrumentId(self.id)] {
            self.update_from_instrument(instrument)
        }
    }

    pub(crate) fn new_from_instrument(
        source: &InstrumentDefinition,
        output_sample_rate: usize,
    ) -> Self {
        let kind = match &source.kind {
            InstrumentDefinitionKind::Wavetable(wavetable) => InstrumentInstanceKind::Wavetable(
                WavetableInstance::new(wavetable.clone(), output_sample_rate),
            ),
            InstrumentDefinitionKind::FMSynth(fm_synth) => InstrumentInstanceKind::FMSynth(
                Box::new(PatchInstance::new(fm_synth.clone(), output_sample_rate)),
            ),
            InstrumentDefinitionKind::Sampler(sample) => {
                InstrumentInstanceKind::Sampler(SamplerInstance::new(sample, output_sample_rate))
            }
        };

        Self {
            id: source.id,
            kind,
            volume: PhraseVolumeType::MAX,
        }
    }

    pub(crate) fn update_from_instrument(&mut self, instrument: &InstrumentDefinition) {
        let output_sample_rate = match &self.kind {
            InstrumentInstanceKind::Wavetable(wv) => wv.oscillator.output_sample_rate,
            InstrumentInstanceKind::FMSynth(fm) => fm.output_sample_rate(),
            InstrumentInstanceKind::Sampler(sm) => sm.oscillator.output_sample_rate,
        };

        *self = Self::new_from_instrument(instrument, output_sample_rate)
    }

    pub(crate) fn update_from_tracker(&mut self, entry: &InstrumentChannelType) {
        if self.id != entry.instrument.id {
            self.update_from_instrument(&entry.instrument)
        }

        self.volume = entry.volume;

        match &mut self.kind {
            InstrumentInstanceKind::Wavetable(wave) => {
                wave.set_frequency(entry.note);
                wave.trigger();
            }
            InstrumentInstanceKind::FMSynth(fm) => {
                fm.set_frequency(entry.note);
                fm.trigger();
            }
            InstrumentInstanceKind::Sampler(sampler) => {
                sampler.set_frequency(entry.note);
                sampler.trigger();
            }
        }
    }

    pub(crate) fn tick(&mut self) -> f32 {
        let raw_output = match &mut self.kind {
            InstrumentInstanceKind::Wavetable(wv) => wv.tick(),
            InstrumentInstanceKind::FMSynth(fm) => fm.tick(),
            InstrumentInstanceKind::Sampler(sm) => sm.tick(),
        };

        raw_output * to_scaled_value(self.volume)
    }

    pub(crate) fn set_active(&mut self, active: bool) {
        match &mut self.kind {
            InstrumentInstanceKind::Wavetable(wv) => wv.set_active(active),
            InstrumentInstanceKind::FMSynth(fm) => fm.set_active(active),
            InstrumentInstanceKind::Sampler(sm) => sm.set_active(active),
        }
    }

    pub(crate) fn trigger(&mut self) {
        match &mut self.kind {
            InstrumentInstanceKind::Wavetable(wv) => wv.trigger(),
            InstrumentInstanceKind::FMSynth(fm) => fm.trigger(),
            InstrumentInstanceKind::Sampler(sm) => sm.trigger(),
        }
    }

    pub(crate) fn set_note(&mut self, note_id: i32) {
        if let Ok(note) = NoteId::try_from(note_id) {
            let frequency = gamercade_audio::get_note(note).frequency;

            match &mut self.kind {
                InstrumentInstanceKind::Wavetable(wv) => wv.set_frequency(frequency),
                InstrumentInstanceKind::FMSynth(fm) => fm.set_frequency(frequency),
                InstrumentInstanceKind::Sampler(sm) => sm.set_frequency(frequency),
            }
        }
    }

    pub(crate) fn set_frequency(&mut self, frequency: f32) {
        match &mut self.kind {
            InstrumentInstanceKind::Wavetable(wv) => wv.set_frequency(frequency),
            InstrumentInstanceKind::FMSynth(fm) => fm.set_frequency(frequency),
            InstrumentInstanceKind::Sampler(sm) => sm.set_frequency(frequency),
        }
    }
}
