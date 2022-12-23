use gamercade_audio::{Song, SONGS_MAX_COUNT};

use gamercade_fs::{EditorAudioDataEntry, EditorSoundData};

use crate::ui::AudioList;

#[derive(Default)]
pub(super) struct SongList {
    pub(super) selected_song: usize,
}

impl AudioList<Song> for SongList {
    const NAME: &'static str = "Song";
    const MAX_ENTRY_COUNT: usize = SONGS_MAX_COUNT;

    fn target_data_mut(data: &mut EditorSoundData) -> &mut Vec<EditorAudioDataEntry<Song>> {
        &mut data.songs
    }

    fn selected_index(&mut self) -> &mut usize {
        &mut self.selected_song
    }

    fn on_add() -> Song {
        Song::default()
    }

    fn on_clear(&mut self, data: &mut Vec<EditorAudioDataEntry<Song>>) {
        data.remove(self.selected_song);
    }
}
