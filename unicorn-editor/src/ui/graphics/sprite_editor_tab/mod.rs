mod sprite_view;
mod sprite_sheet_list;

use eframe::egui::{TextEdit, TextureId, Ui, Vec2, Color32, ImageButton};

use sprite_view::SpriteViewer;
use sprite_sheet_list::SpriteSheetList;

#[derive(Debug, Clone, Default)]
pub struct SpriteEditor {
    sprite_viewer: SpriteViewer,
    sprite_sheet_list: SpriteSheetList
}

impl SpriteEditor {
    pub fn draw(&mut self, ui: &mut Ui, rom: &mut unicorn::core::Unicorn, scale: f32, texture_id: TextureId) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                self.sprite_viewer.draw(ui, rom, scale, texture_id, self.sprite_sheet_list.selected_idx);
                self.sprite_sheet_list.draw(ui, rom, scale, texture_id);
            });
        });
    }
}