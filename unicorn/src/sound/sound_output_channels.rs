use crate::audio::consts::{SFX_CHANNELS, SONG_TRACK_CHANNELS};

#[derive(Clone)]
pub struct SoundOutputChannels {
    pub sfx_output: [f32; SFX_CHANNELS],
    pub bgm_output: [f32; SONG_TRACK_CHANNELS],
}

impl SoundOutputChannels {
    pub fn get_sfx_output(&self) -> f32 {
        self.sfx_output.iter().sum()
    }

    pub fn get_bgm_output(&self) -> f32 {
        self.bgm_output.iter().sum()
    }
}
