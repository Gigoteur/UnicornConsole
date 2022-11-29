pub mod pico8;
pub mod bbc;
pub mod cbmii;
pub mod appleii;
pub mod trollmini;

pub struct Font {
    // Width of glyph in pixels
    glyph_width: i32,
    // Height of glyph in pixels
    glyph_height: i32,
    // Number of x pixels before glyph
    left_bearing: i32,
    // Number of y pixels before glyph
    top_bearing: i32,
    // Horizontal distance to next character
    advance_width: i32,
    // Vertical distance between lines
    line_height: i32,
    // Glyph bitmap data - one byte per row, first bit in MSB
    glyph_data: &'static [u8],
    name: &'static str,
}
