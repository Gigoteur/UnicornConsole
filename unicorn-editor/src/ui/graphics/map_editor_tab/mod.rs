use eframe::egui::{TextEdit, TextureId, Ui, Vec2, Color32, ImageButton};
#[derive(Debug, Clone, Default)]
pub struct MapEditor {

}

impl MapEditor {
    pub fn draw(&mut self, ui: &mut Ui, rom: &mut unicorn::core::Unicorn, scale: f32, texture_id: TextureId) {

    }
}