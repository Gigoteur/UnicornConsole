use gamercade_audio::{Phrase, PHRASES_MAX_COUNT};
use gamercade_fs::{EditorAudioDataEntry, EditorSoundData};

use crate::ui::AudioList;

#[derive(Debug, Default)]
pub(super) struct PhraseList {
    pub(super) selected_phrase: usize,
}

impl AudioList<Option<Phrase>> for PhraseList {
    const MAX_ENTRY_COUNT: usize = PHRASES_MAX_COUNT;
    const NAME: &'static str = "Phrase";

    fn target_data_mut(
        data: &mut EditorSoundData,
    ) -> &mut Vec<EditorAudioDataEntry<Option<Phrase>>> {
        &mut data.phrases
    }

    fn selected_index(&mut self) -> &mut usize {
        &mut self.selected_phrase
    }

    fn on_add() -> Option<Phrase> {
        Some(Phrase::default())
    }

    fn on_clear(&mut self, data: &mut Vec<EditorAudioDataEntry<Option<Phrase>>>) {
        data[self.selected_phrase].data = Some(Phrase::default());
    }
}
