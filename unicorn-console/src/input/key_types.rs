use unicorn::input::ButtonCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(untagged)]
pub(crate) enum KeyType {
    Button(ButtonCode),
    AnalogStick(AnalogStick),
    Trigger(TriggerSide),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) enum AnalogStick {
    LeftXPositive,
    LeftXNegative,
    LeftYPositive,
    LeftYNegative,
    RightXPositive,
    RightXNegative,
    RightYPositive,
    RightYNegative,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) enum TriggerSide {
    LeftTrigger,
    RightTrigger,
}