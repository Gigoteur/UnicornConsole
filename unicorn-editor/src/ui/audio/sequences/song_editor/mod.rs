use eframe::egui::{Grid, InputState, Key, Slider, Ui};

mod song_list;
mod song_row;
use gamercade_audio::{Chain, ChainId, Song, PHRASE_STEPS_PER_BEAT, SONG_TRACK_CHANNELS};
use song_list::*;
use song_row::*;

use gamercade_fs::{EditorAudioDataEntry, EditorSoundData};

use crate::ui::{AudioList, AudioSyncHelper};

use super::{
    HandleTrackerEditEntryCommand, TrackerEditCommand, TrackerEditEntryCommand,
    TrackerEditRowCommand, TRACKER_TEXT_FONT_SIZE,
};

#[derive(Default)]
pub(crate) struct SongEditor {
    song_list: SongList,
    selected_entry: SelectedEntry,
}

#[derive(Default, Clone, Debug)]
struct SelectedEntry {
    selected_row: usize,
    selected_channel: Option<usize>,
}

impl SongEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        self.song_list.draw(ui, data, sync);

        if let Some(song) = data.songs.get_mut(self.song_list.selected_song) {
            ui.label("Song Name: ");
            ui.text_edit_singleline(&mut song.name);

            let song = &mut song.data;

            if ui.add(Slider::new(&mut song.bpm, 0.0..=999.9)).changed() {
                sync.notify_rom_changed();
            }

            ui.label(format!(
                "Song Length (secs): {}",
                song_length_seconds(song, &data.chains)
            ));

            if ui.button("Play").clicked() || ui.input().key_pressed(Key::Space) {
                sync.play_bgm(self.song_list.selected_song);
            }

            if ui.button("Stop").clicked() {
                sync.stop_bgm();
            }

            self.song_editor_inner(ui, song);

            let tracks = &mut song.tracks;

            if ui.button("Add Row").clicked() {
                let mut new_tracks = tracks.to_vec();
                new_tracks.insert(
                    self.selected_entry.selected_row + 1,
                    std::array::from_fn(|_| None),
                );
                self.selected_entry.selected_row += 1;
                *tracks = new_tracks.into_boxed_slice();
                sync.notify_rom_changed();
            }

            if ui.button("Delete Row").clicked() && tracks.len() > 1 {
                let mut new_tracks = tracks.to_vec();
                new_tracks.remove(self.selected_entry.selected_row);
                *tracks = new_tracks.into_boxed_slice();

                self.selected_entry.selected_row =
                    self.selected_entry.selected_row.min(tracks.len() - 1);
                sync.notify_rom_changed();
            }

            let input = ui.input();
            if input.modifiers.shift {
                let song_channels = &mut song.tracks[self.selected_entry.selected_row];
                if let Some(selected_channel) = self.selected_entry.selected_channel {
                    let chain = song_channels.get_mut(selected_channel).unwrap();
                    self.handle_shift_input(&input, chain, sync);
                }
            } else {
                self.handle_input(&input, song);
            }
        } else {
            ui.label("No Songs exist! Please create one.");
        }
    }

    fn song_editor_inner(&mut self, ui: &mut Ui, song: &mut Song) {
        Grid::new("song_editor_grid")
            .min_row_height(TRACKER_TEXT_FONT_SIZE)
            .striped(true)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;

                // Draw the header row
                ui.horizontal_centered(|ui| {
                    let header = SongRow::header();
                    header.draw(ui);
                });
                ui.end_row();

                // Draw the individual entries
                song.tracks.iter_mut().enumerate().for_each(|(row, entry)| {
                    ui.horizontal_centered(|ui| {
                        let song_row = SongRow::new(row, entry, self.selected_entry.clone());
                        match song_row.draw(ui) {
                            Some(Some(channel)) => {
                                self.selected_entry.selected_row = row;
                                self.selected_entry.selected_channel = Some(channel);
                            }
                            Some(None) => {
                                self.selected_entry.selected_row = row;
                                self.selected_entry.selected_channel = None;
                            }
                            None => (),
                        }
                    });
                    ui.end_row();
                });
            });
    }

    fn handle_shift_input(
        &mut self,
        input_state: &InputState,
        chain: &mut Option<ChainId>,
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
                self.handle_edit_chain_entry(entry, chain, sync)
            }
            Some(TrackerEditCommand::EditRow(row)) => {
                self.handle_edit_chain_option(row, chain, sync)
            }
            None => (),
        }
    }

    fn handle_edit_chain_entry(
        &self,
        command: TrackerEditEntryCommand,
        chain: &mut Option<ChainId>,
        sync: &mut AudioSyncHelper,
    ) {
        if let Some(chain) = chain {
            chain.handle_command(command);
            sync.notify_rom_changed()
        }
    }

    fn handle_edit_chain_option(
        &self,
        command: TrackerEditRowCommand,
        chain: &mut Option<ChainId>,
        sync: &mut AudioSyncHelper,
    ) {
        match (command, &chain) {
            (TrackerEditRowCommand::InsertOrDelete, None) => {
                *chain = Some(ChainId::default());
                sync.notify_rom_changed()
            }
            (TrackerEditRowCommand::InsertOrDelete, Some(_)) => {
                *chain = None;
                sync.notify_rom_changed()
            }
        }
    }

    fn handle_input(&mut self, input_state: &InputState, song: &Song) {
        if input_state.key_pressed(Key::ArrowUp) {
            self.selected_entry.selected_row = self.selected_entry.selected_row.saturating_sub(1)
        }

        if input_state.key_pressed(Key::ArrowDown) {
            self.selected_entry.selected_row =
                (self.selected_entry.selected_row + 1).min(song.tracks.len() - 1)
        }

        if input_state.key_pressed(Key::ArrowRight) {
            if let Some(index) = &mut self.selected_entry.selected_channel {
                *index += 1;
                if *index == SONG_TRACK_CHANNELS {
                    *index = 0;
                }
            } else {
                self.selected_entry.selected_channel = Some(0)
            }
        }

        if input_state.key_pressed(Key::ArrowLeft) {
            if let Some(index) = &mut self.selected_entry.selected_channel {
                if *index == 0 {
                    *index = SONG_TRACK_CHANNELS;
                }
                *index -= 1;
            } else {
                self.selected_entry.selected_channel = Some(SONG_TRACK_CHANNELS - 1)
            }
        }
    }
}

// This is copied & pasted from gamercade_audio's song.rs
// with slight modifications
fn song_length_seconds(song: &Song, chains: &[EditorAudioDataEntry<Option<Chain>>]) -> f32 {
    let mut sum = 0.0;
    let empty_pattern_length = (60.0 / song.bpm) * PHRASE_STEPS_PER_BEAT as f32;

    for row in song.tracks.iter() {
        let row_max = row
            .iter()
            .map(|lane| {
                lane.and_then(|chain| {
                    chains.get(chain.0).and_then(|chain| {
                        chain
                            .data
                            .as_ref()
                            .map(|chain| chain.chain_length_seconds(song.bpm))
                    })
                })
                .unwrap_or(empty_pattern_length)
            })
            .reduce(f32::max);

        if let Some(row_max) = row_max {
            sum += row_max
        } else {
            break;
        }
    }

    sum
}
