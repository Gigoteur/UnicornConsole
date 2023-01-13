
use eframe::egui::{TextEdit, TextureId, Ui, Vec2, Color32, ImageButton};

#[derive(Clone, Default, Debug)]
pub struct MapViewer {
    pub(crate) selected_color: usize,
}

impl MapViewer {
    pub(crate) fn draw(&mut self, ui: &mut Ui, rom: &mut unicorn::core::Unicorn, scale: f32, texture_id: TextureId, idx: usize) {
        ui.label("Map Viewer");
    }
}