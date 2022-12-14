use egui::{ComboBox, Slider, Ui};
use gilrs::Gilrs;

use crate::console::{InputMode, LocalInputManager, LocalKeyboardId};

pub struct ControllerGui {
    pub local_player_count: usize,
}

impl Default for ControllerGui {
    fn default() -> Self {
        Self {
            local_player_count: 1,
        }
    }
}

impl ControllerGui {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        can_adjust_player_count: bool,
        input: &mut LocalInputManager,
        gilrs: &Gilrs,
    ) {
        ui.group(|ui| {
            ui.label("Controller Settings:");

            if ui
                .add_enabled(
                    can_adjust_player_count,
                    Slider::new(&mut self.local_player_count, 1..=4).text("Local Player Count"),
                )
                .changed()
            {
                input.player_bindings.resize(
                    self.local_player_count,
                    InputMode::Emulated(LocalKeyboardId(0)),
                );
            };

            input
                .player_bindings
                .iter_mut()
                .enumerate()
                .for_each(|(player_id, input_mode)| {
                    let combo_text = match &input_mode {
                        InputMode::Emulated(keyboard_index) => {
                            format!("Keyboard {}", keyboard_index.0)
                        }
                        InputMode::Gamepad(gamepad_id) => {
                            format!("{} [{}]", gilrs.gamepad(*gamepad_id).name(), gamepad_id)
                        }
                    };

                    ComboBox::from_label(format!("Player {} Settings:", player_id))
                        .selected_text(combo_text)
                        .show_ui(ui, |ui| {
                            (0..input.keyboard_bindings.buttons.len()).for_each(|keyboard_index| {
                                ui.selectable_value(
                                    input_mode,
                                    InputMode::Emulated(LocalKeyboardId(keyboard_index)),
                                    format!("Keyboard {}", keyboard_index),
                                );
                            });

                            gilrs.gamepads().for_each(|(id, gamepad)| {
                                ui.selectable_value(
                                    input_mode,
                                    InputMode::Gamepad(id),
                                    format!("{} [{}]", gamepad.name(), id),
                                );
                            });
                        });
                });
        });
    }
}