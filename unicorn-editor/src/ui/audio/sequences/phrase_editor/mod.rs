use eframe::egui::{Grid, InputState, Key, Slider, Ui};

use gamercade_audio::{InstrumentId, NoteId, Phrase, PhraseEntry, DEFAULT_BPM, PHRASE_MAX_ENTRIES};

use super::{
    HandleTrackerEditEntryCommand, TrackerEditCommand, TrackerEditEntryCommand,
    TrackerEditRowCommand, TRACKER_TEXT_FONT_SIZE,
};

use crate::ui::{AudioList, AudioSyncHelper};
use gamercade_fs::EditorSoundData;

mod phrase_list;
mod phrase_row;

use phrase_list::*;
use phrase_row::*;

#[derive(Debug)]
pub(crate) struct PhraseEditor {
    phrase_list: PhraseList,
    selected_entry: SelectedEntry,

    target_bpm: f32,
}

impl Default for PhraseEditor {
    fn default() -> Self {
        Self {
            phrase_list: Default::default(),
            selected_entry: Default::default(),
            target_bpm: DEFAULT_BPM,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct SelectedEntry {
    index: usize,
    mode: SelectedEntryMode,
}

impl SelectedEntry {
    fn up(&mut self) {
        self.index = self.index.saturating_sub(1);
    }

    fn down(&mut self) {
        self.index += 1;
        if self.index == PHRASE_MAX_ENTRIES {
            self.index = PHRASE_MAX_ENTRIES - 1
        };
    }

    fn left(&mut self) {
        self.mode.left()
    }

    fn right(&mut self) {
        self.mode.right()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum SelectedEntryMode {
    #[default]
    None,
    Note,
    Volume,
    Instrument,
}

impl SelectedEntryMode {
    fn right(&mut self) {
        match self {
            SelectedEntryMode::None => *self = SelectedEntryMode::Note,
            SelectedEntryMode::Note => *self = SelectedEntryMode::Volume,
            SelectedEntryMode::Volume => *self = SelectedEntryMode::Instrument,
            SelectedEntryMode::Instrument => *self = SelectedEntryMode::Note,
        }
    }

    fn left(&mut self) {
        match self {
            SelectedEntryMode::None => *self = SelectedEntryMode::Volume,
            SelectedEntryMode::Note => *self = SelectedEntryMode::Instrument,
            SelectedEntryMode::Volume => *self = SelectedEntryMode::Note,
            SelectedEntryMode::Instrument => *self = SelectedEntryMode::Volume,
        }
    }
}

type PhraseEntryType = PhraseEntry<NoteId, InstrumentId>;

impl PhraseEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        self.phrase_list.draw(ui, data, sync);

        let selected_phrase = &mut data.phrases[self.phrase_list.selected_phrase];

        ui.label("Phrase Name: ");
        ui.text_edit_singleline(&mut selected_phrase.name);

        ui.label("Bpm: ");
        ui.add(Slider::new(&mut self.target_bpm, 0.0..=500.0));

        if ui.button("Play").clicked() || ui.input().key_pressed(Key::Space) {
            sync.play_phrase(self.phrase_list.selected_phrase, self.target_bpm);
        }

        if ui.button("Stop").clicked() {
            sync.stop_sfx()
        }

        if let Some(phrase) = &mut selected_phrase.data {
            self.phrase_editor_inner(ui, phrase);

            let input = ui.input();

            if input.modifiers.shift {
                self.handle_shift_input(&input, phrase, sync);
            } else {
                self.handle_input(&input)
            }
        }
    }

    fn handle_shift_input(
        &mut self,
        input_state: &InputState,
        phrase: &mut Phrase,
        sync: &mut AudioSyncHelper,
    ) {
        let mut command = None;

        let big_adjust = match self.selected_entry.mode {
            SelectedEntryMode::None => 1,
            SelectedEntryMode::Note => 12,
            SelectedEntryMode::Volume => 16,
            SelectedEntryMode::Instrument => 16,
        };

        if input_state.key_pressed(Key::ArrowUp) {
            command = Some(TrackerEditCommand::EditEntry(TrackerEditEntryCommand::Add(
                1,
            )))
        } else if input_state.key_pressed(Key::ArrowRight) {
            command = Some(TrackerEditCommand::EditEntry(TrackerEditEntryCommand::Add(
                big_adjust,
            )))
        } else if input_state.key_pressed(Key::ArrowDown) {
            command = Some(TrackerEditCommand::EditEntry(TrackerEditEntryCommand::Sub(
                1,
            )))
        } else if input_state.key_pressed(Key::ArrowLeft) {
            command = Some(TrackerEditCommand::EditEntry(TrackerEditEntryCommand::Sub(
                big_adjust,
            )))
        } else if input_state.key_pressed(Key::Z) {
            command = Some(TrackerEditCommand::EditRow(
                TrackerEditRowCommand::InsertOrDelete,
            ))
        };

        match command {
            Some(TrackerEditCommand::EditEntry(entry)) => {
                self.handle_edit_entry(entry, phrase, sync)
            }
            Some(TrackerEditCommand::EditRow(row)) => self.handle_edit_row(row, phrase, sync),
            None => (),
        }
    }

    fn handle_edit_entry(
        &self,
        command: TrackerEditEntryCommand,
        phrase: &mut Phrase,
        sync: &mut AudioSyncHelper,
    ) {
        if let Some(phrase) = &mut phrase.entries[self.selected_entry.index] {
            let should_sync = match self.selected_entry.mode {
                SelectedEntryMode::None => false,
                SelectedEntryMode::Note => {
                    phrase.note.handle_command(command);
                    true
                }
                SelectedEntryMode::Volume => {
                    phrase.volume.handle_command(command);
                    true
                }
                SelectedEntryMode::Instrument => {
                    phrase.instrument.handle_command(command);
                    true
                }
            };

            if should_sync {
                sync.notify_rom_changed()
            }
        }
    }

    fn handle_edit_row(
        &self,
        command: TrackerEditRowCommand,
        phrase: &mut Phrase,
        sync: &mut AudioSyncHelper,
    ) {
        let phrase_row = &mut phrase.entries[self.selected_entry.index];
        match (command, &phrase_row) {
            (TrackerEditRowCommand::InsertOrDelete, Some(_)) => {
                *phrase_row = None;
                sync.notify_rom_changed();
            }
            (TrackerEditRowCommand::InsertOrDelete, None) => {
                *phrase_row = Some(PhraseEntry::default());
                sync.notify_rom_changed();
            }
        }
    }

    fn handle_input(&mut self, input_state: &InputState) {
        if input_state.key_pressed(Key::ArrowUp) {
            self.selected_entry.up()
        }

        if input_state.key_pressed(Key::ArrowDown) {
            self.selected_entry.down()
        }

        if input_state.key_pressed(Key::ArrowLeft) {
            self.selected_entry.left()
        }

        if input_state.key_pressed(Key::ArrowRight) {
            self.selected_entry.right()
        }
    }

    fn phrase_editor_inner(&mut self, ui: &mut Ui, phrase: &mut Phrase) {
        Grid::new("phase_editor_grid")
            .min_row_height(TRACKER_TEXT_FONT_SIZE)
            .striped(true)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;

                // Draw the header row
                ui.horizontal_centered(|ui| {
                    let header = PhraseRow::header();
                    header.draw(ui);
                });
                ui.end_row();

                // Draw the individual entries
                phrase
                    .entries
                    .iter_mut()
                    .enumerate()
                    .for_each(|(row, entry)| {
                        ui.horizontal_centered(|ui| {
                            let phrase_row = PhraseRow::new(row, entry, self.selected_entry);
                            if let Some(selected) = phrase_row.draw(ui) {
                                self.selected_entry.index = row;
                                self.selected_entry.mode = selected;
                            }
                        });
                        ui.end_row();
                    });
            });
    }
}
