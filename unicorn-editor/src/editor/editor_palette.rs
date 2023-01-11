use unicorn::gfx::palette::Palette;

#[derive(Clone, Debug)]
pub struct EditorPalette {
    pub name: String,
    pub palette: Palette,
}