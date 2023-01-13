mod sprite_view;
pub mod sprite_sheet_list;
mod palette_view;

use eframe::egui::{TextEdit, TextureId, Ui, Vec2, Color32, ImageButton};

use sprite_view::SpriteViewer;
use palette_view::PaletteViewer;
use sprite_sheet_list::SpriteSheetList;

#[derive(Debug, Clone, Default)]
pub struct SpriteEditor {
    sprite_viewer: SpriteViewer,
    palette_viewer: PaletteViewer,
    sprite_sheet_list: SpriteSheetList
}

impl SpriteEditor {
    pub fn draw(&mut self, ui: &mut Ui, rom: &mut unicorn::core::Unicorn, scale: f32, texture_id: TextureId) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        self.sprite_viewer.draw(ui, rom, scale, texture_id, self.sprite_sheet_list.selected_idx, self.palette_viewer.selected_color);
                        self.palette_viewer.draw(ui, rom, scale, texture_id);
                    });
                });
                self.sprite_sheet_list.draw(ui, rom, scale, texture_id);
            });
        });
    }
}