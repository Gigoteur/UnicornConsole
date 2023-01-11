use eframe::{egui::Ui, epaint::Color32};

use unicorn::audio::consts::SONG_TRACK_CHANNELS;
use unicorn::audio::tracker::chain::ChainId;

use crate::ui::audio::sequences::{
    TrackerText, DEFAULT_TEXT_COLOR, EDITING_BG_COLOR, SELECTED_BG_COLOR,
};

use super::SelectedEntry;

pub struct SongRow {
    row_index: TrackerText<2>,
    separator: TrackerText<2>,
    channels: [TrackerText<2>; SONG_TRACK_CHANNELS],
}

impl SongRow {
    pub(super) fn header() -> Self {
        Self {
            row_index: TrackerText::new("# ", Color32::GRAY, None),
            separator: TrackerText::separator(None),
            channels: std::array::from_fn(|index| {
                TrackerText::new(&format!("c{:X}", index), Color32::GRAY, None)
            }),
        }
    }

    pub(super) fn new(
        row: usize,
        song_entry: &[Option<ChainId>; SONG_TRACK_CHANNELS],
        selected_entry: SelectedEntry,
    ) -> Self {
        let bg_color = if selected_entry.selected_row == row {
            Some(SELECTED_BG_COLOR)
        } else {
            None
        };

        let row_index = TrackerText::new(&format!("{:X}:", row), DEFAULT_TEXT_COLOR, bg_color);
        let separator = TrackerText::separator(bg_color);

        let channels = std::array::from_fn(|index| {
            let mut color = bg_color;

            if let Some(selected_channel) = selected_entry.selected_channel {
                if selected_channel == index && bg_color.is_some() {
                    color = Some(EDITING_BG_COLOR)
                }
            };

            if let Some(chain) = song_entry[index] {
                TrackerText::new(&format!("{:02X}", chain.0), DEFAULT_TEXT_COLOR, color)
            } else {
                TrackerText::new_empty(color)
            }
        });

        Self {
            row_index,
            channels,
            separator,
        }
    }

    pub(crate) fn draw(&self, ui: &mut Ui) -> Option<Option<usize>> {
        let mut output = None;

        let row_name_clicked = self.row_index.draw(ui);
        let separator_clicked = self.separator.draw(ui);

        if row_name_clicked || separator_clicked {
            output = Some(None);
        }

        self.channels
            .iter()
            .enumerate()
            .for_each(|(index, channel)| {
                if self.separator.draw(ui) {
                    output = Some(None);
                }

                if channel.draw(ui) {
                    output = Some(Some(index));
                }
            });

        output
    }
}
