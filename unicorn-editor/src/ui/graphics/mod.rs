mod sprite_editor_tab;
mod map_editor_tab;

use eframe::egui::{
    Color32, ColorImage, Image, Slider, TextureFilter, TextureHandle, TextureId, Ui, Vec2,
};

use sprite_editor_tab::SpriteEditor;
use map_editor_tab::MapEditor;

use unicorn::gfx::palette::Palette;

const PALETTE_COLORS: usize = 16;
const ROWS_PER_PALETTE_PREVIEW: usize = 4;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GraphicsEditorMode {
    Palette,
    Map,
    Sprite,
}

impl Default for GraphicsEditor {
    fn default() -> Self {
        Self {
            mode: GraphicsEditorMode::Sprite,
            sprite_editor: SpriteEditor::default(),
            map_editor: MapEditor::default(),

            scale: 1.0,
            default_palette_texture: None,
        }
    }
}

#[derive(Clone)]
pub struct GraphicsEditor {
    pub mode: GraphicsEditorMode,
    pub sprite_editor: SpriteEditor,
    pub map_editor: MapEditor,

    pub scale: f32,
    default_palette_texture: Option<TextureHandle>,
}

impl GraphicsEditor {
    pub fn draw_selector(&mut self, ui: &mut Ui) {
        ui.selectable_value(&mut self.mode, GraphicsEditorMode::Palette, "Palettes");
        ui.selectable_value(&mut self.mode, GraphicsEditorMode::Map, "Map");
        ui.selectable_value(&mut self.mode, GraphicsEditorMode::Sprite, "Sprite");
    }

    pub fn draw_contents(&mut self, ui: &mut Ui, rom: &mut unicorn::core::Unicorn) {
        let texture_id = self
            .default_palette_texture
            .get_or_insert_with(|| {
                ui.ctx().load_texture(
                    "default palette texture",
                    ColorImage::from_rgba_unmultiplied([1, 1], &[255, 255, 255, 255]),
                    TextureFilter::Nearest,
                )
            })
            .id();

        match self.mode {
            GraphicsEditorMode::Palette => (),
            GraphicsEditorMode::Map => self.map_editor.draw(ui, rom, self.scale, texture_id),
            GraphicsEditorMode::Sprite => self.sprite_editor.draw(ui, rom, self.scale, texture_id),
        };
    }

    pub fn draw_bottom_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Sprite Scaling:");
            ui.add(Slider::new(&mut self.scale, 0.1..=3.0));
        });
    }
}

pub(crate) fn draw_palette_preview(ui: &mut Ui, palette: &Palette, texture_id: TextureId) {
    ui.spacing_mut().item_spacing = Vec2 { x: 0.0, y: 0.0 };
    ui.horizontal(|ui| {
        (0..PALETTE_COLORS / ROWS_PER_PALETTE_PREVIEW).for_each(|x| {
            ui.vertical(|ui| {
                (0..ROWS_PER_PALETTE_PREVIEW).for_each(|y| {
                    let mut idx = (x + (y * ROWS_PER_PALETTE_PREVIEW)) as u32;
                    match palette.colors.get(&idx) {
                        Some(rgb_value) => {
                            let image = Image::new(texture_id, Vec2 { x: 10.0, y: 10.0 }).tint(
                                Color32::from_rgba_unmultiplied(rgb_value.r, rgb_value.g, rgb_value.b, rgb_value.a),
                            );
                            ui.add(image);
                        }
                        _ => (),
                    }
                });
            });
        })
    });
}


pub(crate) fn load_buffered_image<'a>(
    ui: &mut eframe::egui::Ui,
    handle: &'a mut Option<eframe::egui::TextureHandle>,
    label: &'a str,
    rgb: eframe::egui::ColorImage,
) -> &'a eframe::egui::TextureHandle {
    match handle {
        Some(handle) => {
            handle.set(rgb, TextureFilter::Nearest);
            handle
        }
        None => {
            *handle = Some(ui.ctx().load_texture(label, rgb, TextureFilter::Nearest));
            handle.as_ref().unwrap()
        }
    }
}