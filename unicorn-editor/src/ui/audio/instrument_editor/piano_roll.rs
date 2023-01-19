use eframe::{
    egui::{ImageButton, Key, TextureFilter, Ui},
    epaint::{Color32, ColorImage, TextureHandle, Vec2},
};

use unicorn::audio::notes::note_name::{NoteColor, NoteName};
use unicorn::audio::notes::note::NotesIter;
use unicorn::audio::notes::note::get_note;
use unicorn::audio::notes::note::NoteId;

use unicorn::audio::consts::TOTAL_NOTES_COUNT;

use crate::ui::AudioSyncHelper;

use super::KeyboardMode;

const BOTTOM_NOTE_INDEX_START: usize = 36;

#[derive(Clone)]
pub struct PianoRoll {
    default_piano_texture: Option<TextureHandle>,

    bottom_note_index: usize,
    key_states: [bool; KEYBOARD_KEY_COUNT],
    key_channels: [Option<usize>; KEYBOARD_KEY_COUNT],
    current_note: String,
}

impl Default for PianoRoll {
    fn default() -> Self {
        Self {
            default_piano_texture: Default::default(),
            bottom_note_index: BOTTOM_NOTE_INDEX_START,
            key_states: Default::default(),
            key_channels: Default::default(),
            current_note: "".to_string(),
        }
    }
}

const KEYBOARD_KEY_COUNT: usize = 24;

const NOTE_SPACING: f32 = 1.0;
const TOP_KEY_SIZE: Vec2 = Vec2::new(12.0, 32.0);
const BOTTOM_KEY_SIZE: Vec2 = Vec2::new(
    (((TOP_KEY_SIZE.x + NOTE_SPACING) * TOTAL_NOTES_COUNT as f32) - (NOTE_SPACING * 56.0)) / 56.0,
    24.0,
);

const KEYS: &[Key; KEYBOARD_KEY_COUNT] = &[
    Key::Z,
    Key::S,
    Key::X,
    Key::D,
    Key::C,
    Key::V,
    Key::G,
    Key::B,
    Key::H,
    Key::N,
    Key::J,
    Key::M,
    Key::Q,
    Key::Num2,
    Key::W,
    Key::Num3,
    Key::E,
    Key::R,
    Key::Num5,
    Key::T,
    Key::Num6,
    Key::Y,
    Key::Num7,
    Key::U,
];

impl PianoRoll {
    fn key_in_keyboard_range(&self, index: usize) -> bool {
        index >= self.bottom_note_index && index < self.bottom_note_index + KEYBOARD_KEY_COUNT
    }

    fn update_key_states(
        &mut self,
        ui: &mut Ui,
        sync: &mut AudioSyncHelper,
        selected_instrument: usize,
    ) {
        let input = ui.input();
        let next_keys = std::array::from_fn(|index| input.key_down(KEYS[index]));

        self.key_states
            .iter()
            .zip(next_keys.iter())
            .enumerate()
            .for_each(|(index, (prev, next))| {
                if prev != next {
                    if *next {
                        self.current_note = get_note(NoteId(index + self.bottom_note_index)).name.to_string();

                        let assigned_channel =
                            sync.play_note(index + self.bottom_note_index, selected_instrument);
                        self.key_channels[index] = Some(assigned_channel);
                    } else if let Some(assigned_channel) = self.key_channels[index] {
                        sync.stop_note(assigned_channel);
                    } else {
                        println!("Err: Released key for an unknown note!")
                    }
                }
            });

        self.key_states = next_keys;
        drop(input);
    }

    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        sync: &mut AudioSyncHelper,
        selected_instrument: usize,
        keyboard_mode: &KeyboardMode,
    ) {
        ui.group(|ui| {
            ui.label("Piano Roll");

            let piano_active = KeyboardMode::PianoRoll == *keyboard_mode;

            if piano_active {
                // Updates the keyboard key states
                self.update_key_states(ui, sync, selected_instrument);
            }

            // Draws the left/right buttons, and handles
            // Arrow keys going left or right
            ui.horizontal(|ui| {
                let go_left = ui.button("<--").clicked()
                    || (piano_active && ui.input().key_pressed(Key::ArrowLeft));
                let go_right = ui.button("-->").clicked()
                    || (piano_active && ui.input().key_pressed(Key::ArrowRight));

                ui.label(self.current_note.clone());
                if go_left && self.bottom_note_index > 0 {
                    self.bottom_note_index -= 12
                } else if go_right
                    && self.bottom_note_index < TOTAL_NOTES_COUNT - KEYBOARD_KEY_COUNT
                {
                    self.bottom_note_index += 12
                }

                self.draw_piano_keys(ui, sync, selected_instrument);
            });
        });
    }

    /// Draws the piano keys, which are clickable
    fn draw_piano_keys(
        &mut self,
        ui: &mut Ui,
        sync: &mut AudioSyncHelper,
        selected_instrument: usize,
    ) {
        let texture_id = self
            .default_piano_texture
            .get_or_insert_with(|| {
                ui.ctx().load_texture(
                    "default piano texture",
                    ColorImage::from_rgba_unmultiplied([1, 1], &[255, 255, 255, 255]),
                    TextureFilter::Nearest,
                )
            })
            .id();
        // Draw the actual piano keys for clicking
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = Vec2 {
                x: NOTE_SPACING,
                y: 0.0,
            };
            ui.spacing_mut().button_padding = Vec2 { x: 0.0, y: 0.0 };

            ui.horizontal(|ui| {
                let all_notes_iter = NotesIter::default().enumerate();

                all_notes_iter.for_each(|(index, (note, _octave))| {
                    let color = self.get_key_texture_tint(note, index);

                    let button_top = ImageButton::new(texture_id, TOP_KEY_SIZE).tint(color);
                    if ui.add(button_top).clicked() {
                        self.current_note = get_note(NoteId(index)).name.to_string();
                        sync.trigger_note(index, selected_instrument);
                    };
                });
            });

            ui.spacing_mut().item_spacing = Vec2 {
                x: NOTE_SPACING,
                y: 0.0,
            };
            ui.horizontal(|ui| {
                let mut white_notes_iter = NotesIter::default().enumerate();

                for (index, (note, _octave)) in white_notes_iter.by_ref() {
                    if note.get_key_color() == NoteColor::White {
                        let tint = self.get_key_texture_tint(note, index);

                        let button_bottom =
                            ImageButton::new(texture_id, BOTTOM_KEY_SIZE).tint(tint);

                        if ui.add(button_bottom).clicked() {
                            self.current_note = get_note(NoteId(index)).name.to_string();
                            sync.trigger_note(index, selected_instrument);
                        };
                    }
                }
            })
        });
    }

    fn get_key_texture_tint(&self, note: NoteName, index: usize) -> Color32 {
        const OUT_OF_RANGE: &[Color32; 2] = &[Color32::GRAY, Color32::BLACK];
        const IN_RANGE: &[Color32; 2] = &[Color32::WHITE, Color32::DARK_GRAY];
        const ACTIVE: &[Color32; 2] = &[Color32::GREEN, Color32::DARK_GREEN];

        let color = match note.get_key_color() {
            NoteColor::White => 0,
            NoteColor::Black => 1,
        };

        let position = if self.key_in_keyboard_range(index) {
            let inner_index = index - self.bottom_note_index;

            if self.key_states[inner_index] {
                ACTIVE
            } else {
                IN_RANGE
            }
        } else {
            OUT_OF_RANGE
        };

        position[color]
    }
}
