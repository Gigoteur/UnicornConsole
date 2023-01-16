mod sprite_editor_tab;
mod map_editor_tab;
mod code_editor_tab;

use eframe::egui::{
    ColorImage, Slider, TextureFilter, TextureHandle, Ui
};

use sprite_editor_tab::SpriteEditor;
use map_editor_tab::MapEditor;
use code_editor_tab::CodeEditor;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GraphicsEditorMode {
    Palette,
    Map,
    Sprite,
    Code,
}

impl Default for GraphicsEditor {
    fn default() -> Self {
        Self {
            mode: GraphicsEditorMode::Sprite,
            sprite_editor: SpriteEditor::default(),
            map_editor: MapEditor::default(),
            code_editor: CodeEditor::default(),

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
    pub code_editor: CodeEditor,

    pub scale: f32,
    default_palette_texture: Option<TextureHandle>,
}

impl GraphicsEditor {
    pub fn draw_selector(&mut self, ui: &mut Ui) {
        ui.selectable_value(&mut self.mode, GraphicsEditorMode::Code, "Code");
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
            GraphicsEditorMode::Code => self.code_editor.draw(ui, rom, self.scale, texture_id),
            GraphicsEditorMode::Palette => (),
            GraphicsEditorMode::Map => self.map_editor.draw(ui, rom, self.scale, texture_id),
            GraphicsEditorMode::Sprite => self.sprite_editor.draw(ui, rom, self.scale, texture_id),
        };
    }

    pub fn draw_bottom_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Scaling:");
            ui.add(Slider::new(&mut self.scale, 0.1..=3.0));
        });
    }
}