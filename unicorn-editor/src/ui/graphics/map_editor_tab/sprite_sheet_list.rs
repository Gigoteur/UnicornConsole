
use eframe::egui::{ComboBox, ImageButton, TextureId, Ui, Vec2, Color32, ScrollArea, Grid};

use unicorn::gfx::sprite::Sprite;

#[derive(Clone, Default, Debug)]
pub struct SpriteSheetList {
    pub(crate) selected_idx: usize,
    pub int_page: usize,
}

impl SpriteSheetList {
    pub(crate) fn draw(&mut self, ui: &mut Ui, rom: &mut unicorn::core::Unicorn, scale: f32, texture_id: TextureId) {       
            ui.group(|ui| {
                ui.label(format!("Sprite Sheet List"));

                ScrollArea::horizontal().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = Vec2 { x: 0.0, y: 0.0 };

                        Grid::new("map_sprite_sheet_editor_grid").show(ui, |ui| {

                            (0..4).for_each(|y| {
                                (0..16).for_each(|x| {
                                    ui.vertical(|ui| {
                                        self.draw_sprite_preview(ui, rom, y*16+(self.int_page*16)+x, scale/4.0, texture_id);
                                    });
                                });
                                ui.end_row();
                            });
                        });
                    });
                });
            });
            ui.label(format!("Page"));
            ComboBox::from_id_source(0)
            .selected_text(self.int_page.to_string())
            .show_ui(ui, |ui| {
                for page in 0..4 {
                    ui.selectable_value(
                        &mut self.int_page,
                        page,
                        (page as u32).to_string(),
                    );
                }
            });
    }

    pub fn draw_sprite_preview(&mut self, ui: &mut Ui, rom: &mut unicorn::core::Unicorn, idx: usize, scale: f32, texture_id: TextureId) {
        let mut screen = rom.screen.lock().unwrap();
        ui.spacing_mut().item_spacing = Vec2 { x: 0.0, y: 0.0 };
    

        if screen.sprites.len() == 0 {
            return;
        }
    
        ui.spacing_mut().button_padding = Vec2::splat(0.0);

        ui.horizontal(|ui| {
    
            (0..8).for_each(|x| {
                ui.vertical(|ui| {
                    (0..8).for_each(|y| {
                        let index = x + (y * 8);
    
                        let sprite = screen.sprites.get(idx).unwrap();
    
                        let idx_color = sprite.data[index];
                        let color = screen.palette.get_rgb(idx_color as u32);
    
                        let image_button =
                        ImageButton::new(texture_id, Vec2 { x: 16.0 * scale, y: 16.0 * scale })
                                .tint(Color32::from_rgba_unmultiplied(
                                    color.r, color.g, color.b, color.a,
                                ));
                        if ui.add(image_button).clicked() {
                            println!("HERE !!");
                            self.selected_idx = idx;
                        }
                    });
                });
            })
        });
    }
}


