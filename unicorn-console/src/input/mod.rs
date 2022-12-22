
mod gamepad_bindings;
mod key_bindings;
mod key_types;
mod local_input_manager;

use gilrs::GamepadId;
use key_bindings::*;
pub use local_input_manager::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub enum InputMode {
    Emulated(LocalKeyboardId),
    Gamepad(GamepadId),
}

use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct LocalKeyboardId(pub usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct LocalPlayerId(pub usize);