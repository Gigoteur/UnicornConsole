use unicorn::input::{Buttons, InputState, MouseState};

#[derive(Debug, Default, Clone)]
pub struct PlayerInputEntry {
    pub(crate) previous: Buttons,
    pub(crate) current: InputState,

    pub(crate) previous_mouse: MouseState,
    pub(crate) current_mouse: MouseState,
}