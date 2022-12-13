use eframe::egui::{
    Color32, ColorImage, Image, Slider, TextureFilter, TextureHandle, TextureId, Ui, Vec2,
};

use super::{PaletteEditor, SpriteEditor, SpriteSheetEditor};
use gamercade_fs::EditorGraphicsData;

use gamercade_core::{Palette, PALETTE_COLORS};

const ROWS_PER_PALETTE_PREVIEW: usize = 8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GraphicsEditorMode {
    Palette,
    SpriteSheet,
    Sprite,
}

impl Default for GraphicsEditor {
    fn default() -> Self {
        Self {
            mode: GraphicsEditorMode::Palette,
            palette_editor: PaletteEditor::default(),
            sprite_sheet_editor: SpriteSheetEditor::default(),
            sprite_editor: SpriteEditor::default(),

            scale: 16.0,
            default_palette_texture: None,
        }
    }
}

#[derive(Clone)]
pub struct GraphicsEditor {
    pub mode: GraphicsEditorMode,
    pub palette_editor: PaletteEditor,
    pub sprite_sheet_editor: SpriteSheetEditor,
    pub sprite_editor: SpriteEditor,

    pub scale: f32,
    default_palette_texture: Option<TextureHandle>,
}

impl GraphicsEditor {
    pub fn draw_selector(&mut self, ui: &mut Ui) {
        ui.selectable_value(&mut self.mode, GraphicsEditorMode::Palette, "Palettes");
        ui.selectable_value(
            &mut self.mode,
            GraphicsEditorMode::SpriteSheet,
            "Sprite Sheets",
        );
        ui.selectable_value(&mut self.mode, GraphicsEditorMode::Sprite, "Sprite Editor");
    }

    pub fn draw_contents(&mut self, ui: &mut Ui, data: &mut EditorGraphicsData) {
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
            GraphicsEditorMode::Palette => self.palette_editor.draw(
                ui,
                data,
                &self.sprite_sheet_editor,
                self.scale,
                texture_id,
            ),
            GraphicsEditorMode::SpriteSheet => self.sprite_sheet_editor.draw(
                ui,
                data,
                &mut self.palette_editor,
                self.scale,
                texture_id,
            ),
            GraphicsEditorMode::Sprite => self.sprite_editor.draw(ui),
        };
    }

    pub fn draw_bottom_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Sprite Scaling:");
            ui.add(Slider::new(&mut self.scale, 1.0..=16.0));
        });
    }
}

pub(crate) fn draw_palette_preview(ui: &mut Ui, palette: &Palette, texture_id: TextureId) {
    ui.spacing_mut().item_spacing = Vec2 { x: 0.0, y: 0.0 };
    ui.horizontal(|ui| {
        (0..PALETTE_COLORS / ROWS_PER_PALETTE_PREVIEW).for_each(|x| {
            ui.vertical(|ui| {
                (0..ROWS_PER_PALETTE_PREVIEW).for_each(|y| {
                    let color = palette.colors[x + (y * ROWS_PER_PALETTE_PREVIEW)];
                    let image = Image::new(texture_id, Vec2 { x: 10.0, y: 10.0 }).tint(
                        Color32::from_rgba_unmultiplied(color.r, color.g, color.b, color.a),
                    );
                    ui.add(image);
                });
            });
        })
    });
}