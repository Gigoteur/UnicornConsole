use eframe::egui::{Grid, InputState, Key, Slider, Ui};
use gamercade_audio::{Chain, PhraseId, CHAIN_MAX_PHRASE_COUNT, DEFAULT_BPM};
use gamercade_fs::EditorSoundData;

use crate::ui::{AudioList, AudioSyncHelper};

mod chain_list;
mod chain_row;

use chain_list::*;
use chain_row::*;

use super::{
    HandleTrackerEditEntryCommand, TrackerEditCommand, TrackerEditEntryCommand,
    TrackerEditRowCommand, TRACKER_TEXT_FONT_SIZE,
};

pub(crate) struct ChainEditor {
    chain_list: ChainList,

    selected_index: usize,
    target_bpm: f32,
}

impl Default for ChainEditor {
    fn default() -> Self {
        Self {
            chain_list: Default::default(),
            selected_index: Default::default(),
            target_bpm: DEFAULT_BPM,
        }
    }
}

impl ChainEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        self.chain_list.draw(ui, data, sync);

        let selected_chain = &mut data.chains[self.chain_list.selected_chain];

        ui.label("Chain Name: ");
        ui.text_edit_singleline(&mut selected_chain.name);

        ui.label("Bpm: ");
        ui.add(Slider::new(&mut self.target_bpm, 0.0..=500.0));

        if ui.button("Play").clicked() || ui.input().key_pressed(Key::Space) {
            sync.play_chain(self.chain_list.selected_chain, self.target_bpm);
        }

        if ui.button("Stop").clicked() {
            sync.stop_sfx()
        }

        if let Some(chain) = &mut selected_chain.data {
            self.chain_editor_inner(ui, chain);

            let input = ui.input();

            if input.modifiers.shift {
                self.handle_shift_input(&input, chain, sync);
            } else {
                self.handle_input(&input)
            }
        };
    }

    fn handle_shift_input(
        &mut self,
        input_state: &InputState,
        chain: &mut Chain,
        sync: &mut AudioSyncHelper,
    ) {
        let mut command = None;

        if input_state.key_pressed(Key::ArrowUp) {
            command = Some(TrackerEditCommand::EditEntry(TrackerEditEntryCommand::Add(
                1,
            )))
        } else if input_state.key_pressed(Key::ArrowRight) {
            command = Some(TrackerEditCommand::EditEntry(TrackerEditEntryCommand::Add(
                16,
            )))
        } else if input_state.key_pressed(Key::ArrowDown) {
            command = Some(TrackerEditCommand::EditEntry(TrackerEditEntryCommand::Sub(
                1,
            )))
        } else if input_state.key_pressed(Key::ArrowLeft) {
            command = Some(TrackerEditCommand::EditEntry(TrackerEditEntryCommand::Sub(
                16,
            )))
        } else if input_state.key_pressed(Key::Z) {
            command = Some(TrackerEditCommand::EditRow(
                TrackerEditRowCommand::InsertOrDelete,
            ))
        };

        match command {
            Some(TrackerEditCommand::EditEntry(entry)) => {
                self.handle_edit_entry(entry, chain, sync)
            }
            Some(TrackerEditCommand::EditRow(row)) => self.handle_edit_row(row, chain, sync),
            None => (),
        }
    }

    fn handle_edit_entry(
        &self,
        command: TrackerEditEntryCommand,
        chain: &mut Chain,
        sync: &mut AudioSyncHelper,
    ) {
        if let Some(chain) = &mut chain.entries[self.selected_index] {
            chain.handle_command(command);
            sync.notify_rom_changed()
        }
    }

    fn handle_edit_row(
        &self,
        command: TrackerEditRowCommand,
        chain: &mut Chain,
        sync: &mut AudioSyncHelper,
    ) {
        let chain_row = &mut chain.entries[self.selected_index];

        match (command, &chain_row) {
            (TrackerEditRowCommand::InsertOrDelete, None) => {
                *chain_row = Some(PhraseId::default());
                sync.notify_rom_changed()
            }
            (TrackerEditRowCommand::InsertOrDelete, Some(_)) => {
                *chain_row = None;
                sync.notify_rom_changed()
            }
        }
    }

    fn handle_input(&mut self, input_state: &InputState) {
        if input_state.key_pressed(Key::ArrowUp) {
            self.selected_index = self.selected_index.saturating_sub(1)
        }

        if input_state.key_pressed(Key::ArrowDown) {
            self.selected_index = (self.selected_index + 1).min(CHAIN_MAX_PHRASE_COUNT)
        }
    }

    fn chain_editor_inner(&mut self, ui: &mut Ui, chain: &mut Chain) {
        Grid::new("chain_editor_grid")
            .min_row_height(TRACKER_TEXT_FONT_SIZE)
            .striped(true)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;

                // Draw the header row
                ui.horizontal_centered(|ui| {
                    let header = ChainRow::header();
                    header.draw(ui);
                });
                ui.end_row();

                // Draw the individual entries
                chain
                    .entries
                    .iter_mut()
                    .enumerate()
                    .for_each(|(row, entry)| {
                        ui.horizontal_centered(|ui| {
                            let phrase_row = ChainRow::new(row, entry, self.selected_index);
                            if phrase_row.draw(ui) {
                                self.selected_index = row;
                            }
                        });
                        ui.end_row();
                    });
            });
    }
}
