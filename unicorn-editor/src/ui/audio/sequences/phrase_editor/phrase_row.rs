use eframe::{egui::Ui, epaint::Color32};

use unicorn::audio::notes::note::get_note;

use crate::ui::audio::sequences::{
    TrackerText, DEFAULT_TEXT_COLOR, EDITING_BG_COLOR, SELECTED_BG_COLOR,
};

use super::{PhraseEntryType, SelectedEntry, SelectedEntryMode};

pub(super) struct PhraseRow {
    row_index: TrackerText<3>,
    note: TrackerText<3>,
    volume: TrackerText<2>,
    instrument: TrackerText<2>,
    separator: TrackerText<2>,
}

impl PhraseRow {
    pub(super) fn header() -> Self {
        Self {
            row_index: TrackerText::new("# ", Color32::GRAY, None),
            note: TrackerText::new("N  ", Color32::GRAY, None),
            volume: TrackerText::new("V ", Color32::GRAY, None),
            instrument: TrackerText::new("I ", Color32::GRAY, None),
            separator: TrackerText::separator(None),
        }
    }

    pub(crate) fn new(
        row: usize,
        entry: &Option<PhraseEntryType>,
        selected: SelectedEntry,
    ) -> Self {
        let bg_color = if selected.index == row {
            Some(SELECTED_BG_COLOR)
        } else {
            None
        };

        let row = TrackerText::new(&format!("{:X}:", row), DEFAULT_TEXT_COLOR, bg_color);
        let separator = TrackerText::separator(bg_color);

        if let Some(entry) = entry {
            Self {
                row_index: row,
                note: TrackerText::new(
                    &get_note(entry.note).name,
                    DEFAULT_TEXT_COLOR,
                    if selected.mode == SelectedEntryMode::Note && bg_color.is_some() {
                        Some(EDITING_BG_COLOR)
                    } else {
                        bg_color
                    },
                ),
                volume: TrackerText::new(
                    &format!("{:02X}", entry.volume),
                    DEFAULT_TEXT_COLOR,
                    if selected.mode == SelectedEntryMode::Volume && bg_color.is_some() {
                        Some(EDITING_BG_COLOR)
                    } else {
                        bg_color
                    },
                ),
                instrument: TrackerText::new(
                    &format!("{:02X}", entry.instrument.0),
                    DEFAULT_TEXT_COLOR,
                    if selected.mode == SelectedEntryMode::Instrument && bg_color.is_some() {
                        Some(EDITING_BG_COLOR)
                    } else {
                        bg_color
                    },
                ),
                separator,
            }
        } else {
            Self {
                row_index: row,
                note: TrackerText::new_empty(bg_color),
                volume: TrackerText::new_empty(bg_color),
                instrument: TrackerText::new_empty(bg_color),
                separator,
            }
        }
    }

    pub(crate) fn draw(&self, ui: &mut Ui) -> Option<SelectedEntryMode> {
        let results = [
            self.row_index.draw(ui),
            self.separator.draw(ui),
            self.note.draw(ui),
            self.separator.draw(ui),
            self.volume.draw(ui),
            self.separator.draw(ui),
            self.instrument.draw(ui),
        ];

        if let Some((index, _)) = results.into_iter().enumerate().find(|(_, result)| *result) {
            match index {
                2 => Some(SelectedEntryMode::Note),
                4 => Some(SelectedEntryMode::Volume),
                6 => Some(SelectedEntryMode::Instrument),
                _ => Some(SelectedEntryMode::None),
            }
        } else {
            None
        }
    }
}
