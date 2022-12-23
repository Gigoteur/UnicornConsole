use eframe::{egui::Ui, epaint::Color32};
use gamercade_audio::PhraseId;

use crate::ui::audio::sequences::{TrackerText, DEFAULT_TEXT_COLOR, SELECTED_BG_COLOR};
pub(super) struct ChainRow {
    row_index: TrackerText<3>,
    phrase: TrackerText<2>,
    separator: TrackerText<2>,
}

impl ChainRow {
    pub(super) fn header() -> Self {
        Self {
            row_index: TrackerText::new("# ", Color32::GRAY, None),
            separator: TrackerText::separator(None),
            phrase: TrackerText::new("PH", Color32::GRAY, None),
        }
    }

    pub(super) fn new(row: usize, entry: &Option<PhraseId>, selected_index: usize) -> Self {
        let bg_color = if selected_index == row {
            Some(SELECTED_BG_COLOR)
        } else {
            None
        };

        let row_index = TrackerText::new(&format!("{:X}:", row), DEFAULT_TEXT_COLOR, bg_color);
        let separator = TrackerText::separator(bg_color);

        let phrase = if let Some(phrase) = entry {
            TrackerText::new(&format!("{:02X}", phrase.0), DEFAULT_TEXT_COLOR, bg_color)
        } else {
            TrackerText::new_empty(bg_color)
        };

        Self {
            row_index,
            phrase,
            separator,
        }
    }

    pub(super) fn draw(&self, ui: &mut Ui) -> bool {
        let results = [
            self.row_index.draw(ui),
            self.separator.draw(ui),
            self.phrase.draw(ui),
        ];

        results.contains(&true)
    }
}
