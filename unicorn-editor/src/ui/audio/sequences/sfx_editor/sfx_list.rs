use unicorn::audio::sound_rom::Sfx;
use unicorn::audio::consts::SFX_MAX_COUNT;


use crate::editor::editor_sounds_data::{EditorSoundData, EditorAudioDataEntry};

use crate::ui::AudioList;

#[derive(Default)]
pub(super) struct SfxList {
    pub(super) selected_sfx: usize,
}

impl AudioList<Sfx> for SfxList {
    const NAME: &'static str = "Sfx";
    const MAX_ENTRY_COUNT: usize = SFX_MAX_COUNT;

    fn target_data_mut(data: &mut EditorSoundData) -> &mut Vec<EditorAudioDataEntry<Sfx>> {
        &mut data.sfx
    }

    fn selected_index(&mut self) -> &mut usize {
        &mut self.selected_sfx
    }

    fn on_add() -> Sfx {
        Sfx::default()
    }

    fn on_clear(&mut self, data: &mut Vec<EditorAudioDataEntry<Sfx>>) {
        data.remove(self.selected_sfx);
    }
}
