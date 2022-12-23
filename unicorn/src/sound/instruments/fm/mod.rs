pub mod fm_waveform;
pub mod operator_instance;
pub mod patch_instance;

pub(crate) const LUT_QUARTER_LEN: usize = 256;
pub(crate) const LUT_FULL_LEN: usize = LUT_QUARTER_LEN * 4;
pub(crate) const FM_MODULATION: f32 = 8.0;
