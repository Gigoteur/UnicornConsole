use strum::{EnumCount, EnumIter};
use tinystr::TinyAsciiStr;

/// A list of valid octaves for the sound engine.
#[derive(Debug, Clone, Copy, EnumIter, EnumCount, PartialEq, Eq)]
pub enum Octave {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Octave {
    /// Value as an inedx multipler. Used to search for notes.
    pub(crate) fn as_index_multiplier(self) -> usize {
        match self {
            Octave::One => 0,
            Octave::Two => 1,
            Octave::Three => 2,
            Octave::Four => 3,
            Octave::Five => 4,
            Octave::Six => 5,
            Octave::Seven => 6,
            Octave::Eight => 7,
            Octave::Nine => 8,
        }
    }

    /// Value as a single digit character. Used during note generation.
    pub(crate) fn as_str(self) -> TinyAsciiStr<1> {
        TinyAsciiStr::from_str(match self {
            Octave::One => "1",
            Octave::Two => "2",
            Octave::Three => "3",
            Octave::Four => "4",
            Octave::Five => "5",
            Octave::Six => "6",
            Octave::Seven => "7",
            Octave::Eight => "8",
            Octave::Nine => "9",
        })
        .unwrap()
    }
}
