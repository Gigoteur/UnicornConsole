use unicorn::input::ButtonCode;
use gilrs::Button;
use hashbrown::HashMap;

#[derive(Debug)]
pub(crate) struct GamepadBindings {
    pub buttons: HashMap<Button, ButtonCode>,
}

impl Default for GamepadBindings {
    fn default() -> Self {
        let buttons = [
            (Button::DPadUp, ButtonCode::Up),
            (Button::DPadDown, ButtonCode::Down),
            (Button::DPadLeft, ButtonCode::Left),
            (Button::DPadRight, ButtonCode::Right),
            (Button::East, ButtonCode::ButtonA),
            (Button::South, ButtonCode::ButtonB),
            (Button::West, ButtonCode::ButtonC),
            (Button::North, ButtonCode::ButtonD),
            (Button::Start, ButtonCode::Start),
            (Button::Select, ButtonCode::Select),
            (Button::LeftTrigger, ButtonCode::LeftShoulder),
            (Button::RightTrigger, ButtonCode::RightShoulder),
            (Button::LeftThumb, ButtonCode::LeftStick),
            (Button::RightThumb, ButtonCode::RightStick),
        ]
        .into_iter()
        .collect();

        Self { buttons }
    }
}