use strum::{EnumCount, EnumIter};
use tinystr::TinyAsciiStr;

#[derive(Debug, Clone, Copy, EnumIter, EnumCount, PartialEq, Eq)]
pub enum NoteName {
    A,
    ASharp,
    B,
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NoteColor {
    White,
    Black,
}

impl NoteName {
    pub(crate) fn as_index_offset(self) -> usize {
        match self {
            NoteName::A => 0,
            NoteName::ASharp => 1,
            NoteName::B => 2,
            NoteName::C => 3,
            NoteName::CSharp => 4,
            NoteName::D => 5,
            NoteName::DSharp => 6,
            NoteName::E => 7,
            NoteName::F => 8,
            NoteName::FSharp => 9,
            NoteName::G => 10,
            NoteName::GSharp => 11,
        }
    }

    pub(crate) fn as_str(self) -> TinyAsciiStr<2> {
        TinyAsciiStr::from_str(match self {
            NoteName::A => "A-",
            NoteName::ASharp => "A#",
            NoteName::B => "B-",
            NoteName::C => "C-",
            NoteName::CSharp => "C#",
            NoteName::D => "D-",
            NoteName::DSharp => "D#",
            NoteName::E => "E-",
            NoteName::F => "F-",
            NoteName::FSharp => "F#",
            NoteName::G => "G-",
            NoteName::GSharp => "G#",
        })
        .unwrap()
    }

    pub fn get_key_color(self) -> NoteColor {
        use self::NoteColor::*;
        match self {
            NoteName::A => White,
            NoteName::ASharp => Black,
            NoteName::B => White,
            NoteName::C => White,
            NoteName::CSharp => Black,
            NoteName::D => White,
            NoteName::DSharp => Black,
            NoteName::E => White,
            NoteName::F => White,
            NoteName::FSharp => Black,
            NoteName::G => White,
            NoteName::GSharp => Black,
        }
    }
}
