use eframe::{
    egui::{Label, RichText, Sense, Ui},
    epaint::Color32,
};
use tinystr::TinyAsciiStr;

pub(crate) const TRACKER_TEXT_FONT_SIZE: f32 = 32.0;

pub(crate) struct TrackerText<const N: usize> {
    text: TinyAsciiStr<N>,
    text_color: Color32,
    bg_color: Option<Color32>,
}

impl<const N: usize> TrackerText<N> {
    pub fn new(text: &str, text_color: Color32, bg_color: Option<Color32>) -> Self {
        Self {
            text: TinyAsciiStr::from_str(text).unwrap(),
            text_color,
            bg_color,
        }
    }

    pub fn draw(&self, ui: &mut Ui) -> bool {
        let mut text = RichText::new(self.text.as_str())
            .color(self.text_color)
            .monospace()
            .size(TRACKER_TEXT_FONT_SIZE);
        if let Some(bg_color) = self.bg_color {
            text = text.background_color(bg_color)
        };
        ui.add(Label::new(text).sense(Sense::click())).clicked()
    }

    pub fn separator(bg_color: Option<Color32>) -> Self {
        Self {
            text: TinyAsciiStr::from_bytes(&[u8::try_from(' ').unwrap(); N]).unwrap(),
            text_color: Color32::DARK_GRAY,
            bg_color,
        }
    }

    pub fn new_empty(bg_color: Option<Color32>) -> Self {
        Self {
            text: TinyAsciiStr::from_bytes(&[u8::try_from('-').unwrap(); N]).unwrap(),
            text_color: Color32::DARK_GRAY,
            bg_color,
        }
    }
}
