pub mod fm;
pub mod instrument_instance;
pub mod sampler;
pub mod wavetable;

/// The Trigger or Key state for the sound source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveState {
    Off,
    On,
    Trigger,
}
