use crate::input::{Buttons, InputState, MouseState};

#[derive(Debug, Default, Clone)]
pub struct PlayerInputEntry {
    pub previous: Buttons,
    pub current: InputState,

    pub previous_mouse: MouseState,
    pub current_mouse: MouseState,
}