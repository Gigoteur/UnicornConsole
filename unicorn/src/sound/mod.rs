pub mod envelope;
pub mod instruments;
pub mod playback;
pub mod sound_engine;
pub mod sound_output_channels;
pub mod sound_rom_instance;

use crate::audio::envelope_definition::EnvelopeDefinition;
use crate::audio::instruments::wavetable::wavetable_definition::WavetableDefinition;
use crate::sound::instruments::wavetable::wavetable_instance::NO_SOUND_DEFINITION;

fn initialize_globals() {
    instruments::fm::fm_waveform::init_fm_lut();
    crate::audio::notes::note::initialize_notes();
    unsafe {
        NO_SOUND_DEFINITION.write(std::sync::Arc::new(WavetableDefinition {
            data: Box::new([0, 0]),
            envelope: EnvelopeDefinition::default(),
            interpolator: crate::audio::instruments::index_interpolator::IndexInterpolator::Truncate,
        }));
    }
}
