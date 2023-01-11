use eframe::egui::Ui;

use unicorn::audio::instruments::instrument_data_definition::InstrumentDataDefinition;
use unicorn::audio::consts::INSTRUMENTS_MAX_COUNT;

use crate::editor::editor_sounds_data::{EditorAudioDataEntry, EditorSoundData};

use crate::ui::{AudioList, AudioSyncHelper};

#[derive(Default)]
pub struct InstrumentList {
    pub selected_instrument: usize,
}

impl AudioList<Option<InstrumentDataDefinition>> for InstrumentList {
    const NAME: &'static str = "Instrument";
    const MAX_ENTRY_COUNT: usize = INSTRUMENTS_MAX_COUNT;

    fn draw_buttons(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        ui.horizontal(|ui| {
            if ui.button("New").clicked() && data.instruments.len() < Self::MAX_ENTRY_COUNT {
                data.instruments.push(EditorAudioDataEntry::default());
                sync.notify_rom_changed()
            }

            if ui.button("Clear Instrument").clicked() {
                data.instruments[self.selected_instrument] = EditorAudioDataEntry::default();
                sync.notify_rom_changed()
            }

            if ui.button("Clean Up Instruments").clicked() {
                // TODO: Clean up unused instruments
                // Have to iterate through the list and find the highest "non-none" value
                // Then we can remove all of those after it.
                println!("TODO: Clean Up Instruments")
            }
        });
    }

    fn target_data_mut(
        data: &mut EditorSoundData,
    ) -> &mut Vec<EditorAudioDataEntry<Option<InstrumentDataDefinition>>> {
        &mut data.instruments
    }

    fn selected_index(&mut self) -> &mut usize {
        &mut self.selected_instrument
    }

    fn on_add() -> Option<InstrumentDataDefinition> {
        unreachable!()
    }

    fn on_clear(
        &mut self,
        _data: &mut Vec<EditorAudioDataEntry<Option<InstrumentDataDefinition>>>,
    ) {
        unreachable!()
    }
}
