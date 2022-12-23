use std::f32::consts::{FRAC_PI_2, PI, TAU};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WavetableWaveform {
    // Basics
    Sine,
    Square,
    Pulse(f32),
    Saw,
    Triangle,

    // OPL
    HalfSine,
    AbsoluteSine,
    QuarterSine,
    AlternatingSine,
    CamelSine,
    LogarithmicSaw,

    // Tx81z
    InvertedSine,
    InvertedHalfSine,
    InvertedAlternatingSine,
    InvertedCamelSine,

    // Noise
    Noise,
}

impl Default for WavetableWaveform {
    fn default() -> Self {
        Self::Sine
    }
}

impl WavetableWaveform {
    /// Generates a Sine wave oscilator
    pub fn sine() -> Self {
        Self::Sine
    }

    /// Generates a Pulse wave oscilator.
    pub fn pulse(duty: f32) -> Self {
        assert!(duty < 1.0);
        assert!(duty > 0.0);

        if duty == 0.5 {
            Self::Square
        } else {
            Self::Pulse(duty)
        }
    }

    /// Generates a Square wave oscilator. Equal to pulse(0.5)
    pub fn square() -> Self {
        Self::Square
    }

    /// Generates a Sawtooth wave oscilator
    pub fn saw() -> Self {
        Self::Saw
    }

    /// Generates a Triangle wave oscilator
    pub fn triangle() -> Self {
        Self::Triangle
    }

    /// Generates a Half Sine wave oscilator. Produces
    /// a sound if the value is >= 0
    pub fn half_sine() -> Self {
        Self::HalfSine
    }

    /// Generates an Absolute Sine wave oscilator.
    pub fn absolute_sine() -> Self {
        Self::AbsoluteSine
    }

    /// Generates a Quarter Sine wave oscilator. Generates a sound
    /// for the rising part of a sine wave.
    pub fn quarter_sine() -> Self {
        Self::QuarterSine
    }

    /// Generates an Alternating Sine wave oscilator. Produces a wave
    /// with a half-period arch and trough, then silence. Similar to a
    /// sine wave with half of the period.
    pub fn alternating_sine() -> Self {
        Self::AlternatingSine
    }

    /// Generates a Camel Sine wave oscilator. Produces a wave
    /// with two half-period arches, then silence. Similar to
    /// absolute value of an alternating sine oscilator.
    pub fn camel_sine() -> Self {
        Self::CamelSine
    }

    /// Generates a Logarithmic Sawtooth wave oscilator.
    pub fn logarithmic_saw() -> Self {
        Self::LogarithmicSaw
    }

    /// Generates the result of waveform
    pub fn func(self, value: f32) -> f32 {
        match self {
            Self::Sine => value.sin(),
            Self::Pulse(duty) => pulse(value, duty),
            Self::Square => square(value),
            Self::Saw => ((value % TAU) / PI) - 1.0,
            Self::Triangle => value.sin().asin() / FRAC_PI_2,
            Self::HalfSine => half_sine(value),
            Self::AbsoluteSine => value.sin().abs(),
            Self::QuarterSine => quarter_sine(value),
            Self::AlternatingSine => alternating_sine(value),
            Self::CamelSine => camel_sine(value),
            Self::LogarithmicSaw => logarithmic_saw(value),
            Self::InvertedSine => inverted_sine(value),
            Self::InvertedHalfSine => inverted_half_sine(value),
            Self::InvertedAlternatingSine => inverted_alternating_sine(value),
            Self::InvertedCamelSine => inverted_camel_sine(value),
            Self::Noise => (fastrand::f32() * 2.0) - 1.0,
        }
    }
}

fn pulse(value: f32, duty: f32) -> f32 {
    if (value.sin() + 1.0) / 2.0 < duty {
        -1.0
    } else {
        1.0
    }
}

fn square(value: f32) -> f32 {
    1.0_f32.copysign(value.sin())
}

fn half_sine(value: f32) -> f32 {
    if value < TAU * 0.5 {
        value.sin()
    } else {
        0.0
    }
}

fn quarter_sine(value: f32) -> f32 {
    let output = value.sin().abs() * square(value * 2.0);
    if output.is_sign_positive() {
        output
    } else {
        0.0
    }
}

fn alternating_sine(value: f32) -> f32 {
    if value < TAU * 0.5 {
        (value * 2.0).sin()
    } else {
        0.0
    }
}

fn camel_sine(value: f32) -> f32 {
    alternating_sine(value).abs()
}

fn logarithmic_saw(value: f32) -> f32 {
    (((value % TAU) - PI) / PI).asin() / -FRAC_PI_2
}

fn inverted_sine(value: f32) -> f32 {
    let cos = value.cos();
    if value < TAU * 0.25 {
        1.0 - cos
    } else if value < TAU * 0.5 {
        1.0 + cos
    } else if value < TAU * 0.75 {
        -1.0 - cos
    } else {
        -1.0 + cos
    }
}

fn inverted_half_sine(value: f32) -> f32 {
    if value < TAU * 0.5 {
        inverted_sine(value)
    } else {
        0.0
    }
}
fn inverted_alternating_sine(value: f32) -> f32 {
    if value < TAU * 0.5 {
        inverted_sine(value * 2.0)
    } else {
        0.0
    }
}
fn inverted_camel_sine(value: f32) -> f32 {
    inverted_alternating_sine(value).abs()
}
