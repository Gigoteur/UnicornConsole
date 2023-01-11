use eframe::egui::Ui;

use unicorn::audio::instruments::instrument_data_definition::InstrumentDataDefinition;

use crate::editor::editor_sounds_data::EditorSoundData;

use super::{AudioList, AudioSyncHelper};

mod envelope_widget;
mod fm_editor;
mod instrument_list;
mod instrument_top_panel;
mod interpolator_widget;
mod piano_roll;
mod sampler_editor;
mod wavetable_editor;

use fm_editor::*;
use instrument_list::*;
use instrument_top_panel::*;
use piano_roll::*;
use sampler_editor::*;
use wavetable_editor::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) enum KeyboardMode {
    Normal,

    #[default]
    PianoRoll,
}

#[derive(Default)]
pub struct InstrumentEditor {
    fm_editor: FMEditor,
    wavetable_editor: WavetableEditor,
    sampler_editor: SamplerEditor,

    instrument_list: InstrumentList,
    instrument_top_panel: InstrumentTopPanel,
    piano_roll: PianoRoll,
    keyboard_mode: KeyboardMode,
}

impl InstrumentEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        self.instrument_list.draw(ui, data, sync);

        let index = self.instrument_list.selected_instrument;

        if let Some(instrument) = data.instruments.get_mut(index) {
            self.instrument_top_panel
                .draw(ui, instrument, sync, &mut self.keyboard_mode);

            // Now we need to determine which instrument kind we are currenty editing
            ui.group(|ui| match &mut instrument.data {
                Some(InstrumentDataDefinition::Wavetable(wv)) => {
                    self.wavetable_editor.draw(ui, wv, sync)
                }
                Some(InstrumentDataDefinition::FMSynth(fm)) => self.fm_editor.draw(ui, fm, sync),
                Some(InstrumentDataDefinition::Sampler(sm)) => {
                    self.sampler_editor.draw(ui, sm, sync)
                }
                None => {
                    ui.label("Instrument is not initialized");
                }
            });
        } else {
            println!("InstrumentEditor: selected_index is invalid")
        }

        self.piano_roll.draw(
            ui,
            sync,
            self.instrument_list.selected_instrument,
            &self.keyboard_mode,
        );
    }
}
