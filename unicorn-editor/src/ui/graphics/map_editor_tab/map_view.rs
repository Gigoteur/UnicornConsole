
use eframe::egui::{TextureId, Ui};

#[derive(Clone, Default, Debug)]
pub struct MapViewer {
}

impl MapViewer {
    pub(crate) fn draw(&mut self, ui: &mut Ui, _rom: &mut unicorn::core::Unicorn, _scale: f32, _texture_id: TextureId, _idx: usize) {
        ui.label("Map Viewer");
    }
}