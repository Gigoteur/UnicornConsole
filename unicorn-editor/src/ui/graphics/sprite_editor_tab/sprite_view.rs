
use eframe::egui::{TextEdit, TextureId, Ui, Vec2, Color32, ImageButton};

#[derive(Clone, Default, Debug)]
pub struct SpriteViewer {
    pub(crate) selected_color: usize,
}

impl SpriteViewer {
    pub(crate) fn draw(&mut self, ui: &mut Ui, rom: &mut unicorn::core::Unicorn, scale: f32, texture_id: TextureId, idx: usize) {
        ui.label("Sprite Viewer");
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2 { x: 0.0, y: 0.0 };

            ui.horizontal(|ui| {
                (0..8).for_each(|x| {
                    ui.vertical(|ui| {
                        (0..8).for_each(|y| {
                            let mut screen = rom.screen.lock().unwrap();
                            if screen.sprites.len() > 0 {
                                let sprite = screen.sprites.get(idx).unwrap();

                                let index = x + (y * 8);
                                let selected = index == self.selected_color;
                                
                                let idx_color = sprite.data[index];
                                let color = screen.palette.get_rgb(idx_color as u32);

                                let image_button =
                                    ImageButton::new(texture_id, Vec2 { x: 16.0 * scale, y: 16.0 * scale })
                                        .selected(selected)
                                        .tint(Color32::from_rgba_unmultiplied(
                                            color.r, color.g, color.b, color.a,
                                        ));
                                if ui.add(image_button).clicked() {
                                    self.selected_color = index
                                };
                            }
                        });
                    });
                })
            });
        });
    }

}