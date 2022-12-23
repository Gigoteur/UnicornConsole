use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FMWaveform {
    Sine,
    InverseSine,
    HalfSine,
    InverseHalfSine,
    AlternatingSine,
    InverseAlternatingSine,
    CamelSine,
    InveseCamelSine,
}
