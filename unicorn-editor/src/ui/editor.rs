use std::path::PathBuf;

use eframe::egui::{self, menu, Context};
use rfd::FileDialog;


use super::{AudioEditor, GraphicsEditor};

pub struct Editor {
    pub rom: unicorn::core::Unicorn,
    pub mode: EditorMode,

    graphics_editor: GraphicsEditor,
    audio_editor: AudioEditor,

    wasm_path: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditorMode {
    Graphics,
    Audio,
}

impl Default for Editor {
    fn default() -> Self {
        let rom = unicorn::core::Unicorn::new();
        Self {
            mode: EditorMode::Graphics,
            graphics_editor: GraphicsEditor::default(),
            audio_editor: AudioEditor::new(&rom.sounds),
            wasm_path: None,
            rom,
        }
    }
}

impl eframe::App for Editor {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.draw_menu_panel(ctx);
        self.draw_bottom_panel(ctx);
        self.draw_central_panel(ctx);
    }
}

impl Editor {
    pub fn draw_menu_panel(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("editor_top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.audio_editor.audio_sync_helper.notify_rom_changed();
                        ui.close_menu();
                    }

                    if ui.button("Open").clicked() {
                        if let Some(path) = FileDialog::new()
                        .add_filter("corn (.corn), p8 (.p8)", &["corn", "p8"])
                        .pick_file()
                        {
                            self.rom.load_cartridge(String::from(path.to_string_lossy()));
                        }

                        ui.close_menu();
                    }

                    if ui.button("Save").clicked() {
                        ui.close_menu();
                    }
                });
            });
        });
    }

    pub fn draw_central_panel(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, EditorMode::Graphics, "Graphics Mode");

                ui.separator();

                ui.horizontal(|ui| match &mut self.mode {
                    EditorMode::Graphics => self.graphics_editor.draw_selector(ui),
                    EditorMode::Audio => self.audio_editor.draw_selector(ui),
                });
            });

            match self.mode {
                EditorMode::Graphics => self
                    .graphics_editor
                    .draw_contents(ui, &mut self.rom.graphics),
                EditorMode::Audio => self.audio_editor.draw_contents(ui, &mut self.rom.sounds),
            }
        });
    }

    pub fn draw_bottom_panel(&mut self, ctx: &Context) {
        egui::TopBottomPanel::bottom("editor_bottom_panel").show(ctx, |ui| match self.mode {
            EditorMode::Graphics => self.graphics_editor.draw_bottom_panel(ui),
            EditorMode::Audio => self.audio_editor.draw_bottom_panel(ui),
        });
    }
}