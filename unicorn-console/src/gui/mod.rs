use egui::{Button, Context};
use std::path::PathBuf;
use gilrs::Gilrs;
use pixels::Pixels;
use rfd::FileDialog;
use winit::{dpi::PhysicalSize, window::Window};

use unicorn;

pub mod framework;

pub struct Gui {
    pub window_open: bool,
    pub game_file: Option<PathBuf>,
}


impl Default for Gui {
    fn default() -> Self {
        Self {
            window_open: true,
            game_file: None,
        }
    }
}

impl Gui {
    fn ui(
        &mut self,
        pixels: &mut Pixels,
        window: &Window,
        session: &mut unicorn::core::Unicorn,
        ctx: &Context,
        gilrs: &mut Gilrs,
    ) {
        let mut is_open = self.window_open;
        egui::Window::new("Main Menu")
            .open(&mut is_open)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Select Game").clicked() {
                            self.game_file = FileDialog::new()
                                .add_filter("corn (.corn), p8 (.p8)", &["corn", "p8"])
                                .pick_file();
                        };

                        if let Some(file) = &self.game_file {
                            let filename = file
                                .file_name()
                                .expect("filename not found")
                                .to_string_lossy()
                                .to_string();
                            ui.label(filename.clone());
                        }
                    });
                });

                // Draw internal content

                let launch_game_text = "Launch Game";
                
                ui.separator();

                ui.horizontal(|ui| {
                    let launch_game = Button::new(launch_game_text);
                    if ui
                        .add_enabled(self.game_file.is_some(), launch_game)
                        .clicked()
                    {
                        // Launch the game !
                        let path = self.game_file.as_ref().unwrap();
                        session.load_cartridge(String::from(path.to_string_lossy()), false);
                        session.init();
                    }

                    let buttons_enabled = self.game_file.is_some();

                    if ui
                        .add_enabled(buttons_enabled, Button::new("Reset Game"))
                        .clicked()
                    {
                    }

                    if ui
                        .add_enabled(buttons_enabled, Button::new("Quit Game"))
                        .clicked()
                    {
                    }
                });
            });
    }


}