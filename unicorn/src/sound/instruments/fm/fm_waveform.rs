use std::mem::MaybeUninit;

use crate::audio::instruments::fm::fm_waveform::FMWaveform;

use crate::sound::instruments::fm::{LUT_FULL_LEN, LUT_QUARTER_LEN};

static mut SIN_LUT: MaybeUninit<[f32; LUT_QUARTER_LEN]> = MaybeUninit::uninit();

pub(crate) fn init_fm_lut() {
    use std::f32::consts::{PI, TAU};
    unsafe {
        SIN_LUT.write(std::array::from_fn(|index| {
            let phase = (TAU * index as f32) / LUT_FULL_LEN as f32;
            let phase = phase + (PI / LUT_FULL_LEN as f32); //Offset it slightly to break symmetry

            phase.sin()
        }));
    }
}

#[derive(Clone, Copy)]
enum Quadrant {
    First,
    Second,
    Third,
    Fourth,
}

impl Quadrant {
    pub fn from_index(index: usize) -> Self {
        if index < LUT_QUARTER_LEN {
            Quadrant::First
        } else if index < LUT_QUARTER_LEN * 2 {
            Quadrant::Second
        } else if index < LUT_QUARTER_LEN * 3 {
            Quadrant::Third
        } else if index < LUT_FULL_LEN {
            Quadrant::Fourth
        } else {
            unreachable!()
        }
    }
}

/// Returns the actual f32 value of the waveform from the table
pub fn lookup(waveform: FMWaveform, index: usize) -> f32 {
    match waveform {
        FMWaveform::Sine => sine_lut(index),
        FMWaveform::InverseSine => inverse_sine_lut(index),
        FMWaveform::HalfSine => half_sine_lut(index),
        FMWaveform::InverseHalfSine => inverse_half_sine_lut(index),
        FMWaveform::AlternatingSine => alternating_sine_lut(index),
        FMWaveform::InverseAlternatingSine => inverse_alternating_sine_lut(index),
        FMWaveform::CamelSine => camel_sine_lut(index),
        FMWaveform::InveseCamelSine => invese_camel_sine_lut(index),
    }
}

fn sine_lut(index: usize) -> f32 {
    let lut = unsafe { SIN_LUT.assume_init_ref() };
    let index_mod = index % LUT_QUARTER_LEN;

    match Quadrant::from_index(index) {
        Quadrant::First => lut[index_mod],
        Quadrant::Second => lut[LUT_QUARTER_LEN - index_mod - 1],
        Quadrant::Third => -lut[index_mod],
        Quadrant::Fourth => -lut[LUT_QUARTER_LEN - index_mod - 1],
    }
}

fn inverse_sine_lut(index: usize) -> f32 {
    let lut = unsafe { SIN_LUT.assume_init_ref() };
    let index_mod = index % LUT_QUARTER_LEN;

    match Quadrant::from_index(index) {
        Quadrant::First => 1.0 - lut[LUT_QUARTER_LEN - index_mod - 1],
        Quadrant::Second => 1.0 - lut[index_mod],
        Quadrant::Third => -1.0 + lut[LUT_QUARTER_LEN - index_mod - 1],
        Quadrant::Fourth => -1.0 + lut[index_mod],
    }
}

fn half_sine_lut(index: usize) -> f32 {
    let lut = unsafe { SIN_LUT.assume_init_ref() };
    let index_mod = index % LUT_QUARTER_LEN;

    match Quadrant::from_index(index) {
        Quadrant::First => lut[index_mod],
        Quadrant::Second => lut[LUT_QUARTER_LEN - index_mod - 1],
        Quadrant::Third | Quadrant::Fourth => 0.0,
    }
}

fn inverse_half_sine_lut(index: usize) -> f32 {
    let lut = unsafe { SIN_LUT.assume_init_ref() };
    let index_mod = index % LUT_QUARTER_LEN;

    match Quadrant::from_index(index) {
        Quadrant::First => 1.0 - lut[LUT_QUARTER_LEN - index_mod - 1],
        Quadrant::Second => 1.0 - lut[index_mod],
        Quadrant::Third | Quadrant::Fourth => 0.0,
    }
}

fn alternating_sine_lut(index: usize) -> f32 {
    let lut = unsafe { SIN_LUT.assume_init_ref() };

    let index = index * 2;

    if index > LUT_FULL_LEN - 1 {
        return 0.0;
    }

    let index_mod = index % LUT_QUARTER_LEN;
    match Quadrant::from_index(index) {
        Quadrant::First => lut[index_mod],
        Quadrant::Second => lut[LUT_QUARTER_LEN - index_mod - 1],
        Quadrant::Third => -lut[index_mod],
        Quadrant::Fourth => -lut[LUT_QUARTER_LEN - index_mod - 1],
    }
}

fn inverse_alternating_sine_lut(index: usize) -> f32 {
    let lut = unsafe { SIN_LUT.assume_init_ref() };

    let index = index * 2;

    if index > LUT_FULL_LEN - 1 {
        return 0.0;
    }

    let index_mod = index % LUT_QUARTER_LEN;
    match Quadrant::from_index(index) {
        Quadrant::First => 1.0 - lut[LUT_QUARTER_LEN - index_mod - 1],
        Quadrant::Second => 1.0 - lut[index_mod],
        Quadrant::Third => -1.0 + lut[LUT_QUARTER_LEN - index_mod - 1],
        Quadrant::Fourth => -1.0 + lut[index_mod],
    }
}

fn camel_sine_lut(index: usize) -> f32 {
    let lut = unsafe { SIN_LUT.assume_init_ref() };

    let index = index * 2;

    if index > LUT_FULL_LEN - 1 {
        return 0.0;
    }

    let index_mod = index % LUT_QUARTER_LEN;
    match Quadrant::from_index(index) {
        Quadrant::First | Quadrant::Third => lut[index_mod],
        Quadrant::Second | Quadrant::Fourth => lut[LUT_QUARTER_LEN - index_mod - 1],
    }
}

fn invese_camel_sine_lut(index: usize) -> f32 {
    let lut = unsafe { SIN_LUT.assume_init_ref() };

    let index = index * 2;

    if index > LUT_FULL_LEN - 1 {
        return 0.0;
    }

    let index_mod = index % LUT_QUARTER_LEN;
    match Quadrant::from_index(index) {
        Quadrant::First | Quadrant::Third => 1.0 - lut[LUT_QUARTER_LEN - index_mod - 1],
        Quadrant::Second | Quadrant::Fourth => 1.0 - lut[index_mod],
    }
}
