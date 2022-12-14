pub mod input_code;
pub mod input_state;
pub mod mouse_state;

pub use self::input_code::*;
pub use self::input_state::*;
pub use self::mouse_state::*;

pub trait AsApiCode: Sized {
    fn to_api_code(&self) -> u8;
    fn from_api_code(code: u8) -> Option<Self>;
}