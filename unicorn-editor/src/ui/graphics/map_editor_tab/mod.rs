mod map_view;
mod sprite_sheet_list;

use eframe::egui::{TextureId, Ui};


#[derive(Debug, Clone, Default)]
pub struct MapEditor {
    map_viewer: map_view::MapViewer,
    sprite_sheet_list: sprite_sheet_list::SpriteSheetList,
}

impl MapEditor {
    pub fn draw(&mut self, ui: &mut Ui, rom: &mut unicorn::core::Unicorn, scale: f32, texture_id: TextureId) {        ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    self.map_viewer.draw(ui, rom, scale, texture_id, self.sprite_sheet_list.selected_idx);
                });
            });
            self.sprite_sheet_list.draw(ui, rom, scale, texture_id);
        });
    });
    }
}