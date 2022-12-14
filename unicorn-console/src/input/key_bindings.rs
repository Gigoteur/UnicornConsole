use std::path::PathBuf;

use gamercade_core::ButtonCode;

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use winit::event::VirtualKeyCode;

use super::key_types::{AnalogStick, KeyType, TriggerSide};

const INPUT_FILE_NAME: &str = "keyboardInput.json";

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct KeyBindings {
    pub buttons: Vec<HashMap<VirtualKeyCode, KeyType>>,
}

impl KeyBindings {
    pub fn load() -> Self {
        let path = PathBuf::from(INPUT_FILE_NAME);
        if path.exists() {
            match std::fs::read(INPUT_FILE_NAME) {
                Ok(file) => match serde_json::from_slice::<Self>(&file) {
                    Ok(key_bindings) => {
                        println!("Successfully loaded key bindings from: {}", INPUT_FILE_NAME);
                        return key_bindings;
                    }
                    Err(e) => {
                        println!("{} found, but unable to parse: {}", INPUT_FILE_NAME, e);
                    }
                },
                Err(e) => println!("{} found, but unable to read: {}", INPUT_FILE_NAME, e),
            };

            println!("Using default config.");
            Self::default()
        } else {
            println!(
                "{} not found. Generating default input file.",
                INPUT_FILE_NAME
            );
            let bindings = Self::default();

            let json = serde_json::to_string_pretty(&bindings).unwrap();

            match std::fs::write(path, json) {
                Ok(()) => println!("Successfully generated default {}", INPUT_FILE_NAME),
                Err(e) => println!("Error writing {}: {}", INPUT_FILE_NAME, e),
            };

            bindings
        }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        let buttons = vec![[
            //Sticks
            (VirtualKeyCode::X, KeyType::Button(ButtonCode::LeftStick)),
            (VirtualKeyCode::B, KeyType::Button(ButtonCode::RightStick)),
            //Shoulders
            (VirtualKeyCode::E, KeyType::Button(ButtonCode::LeftShoulder)),
            (
                VirtualKeyCode::Q,
                KeyType::Trigger(TriggerSide::LeftTrigger),
            ),
            (
                VirtualKeyCode::R,
                KeyType::Button(ButtonCode::RightShoulder),
            ),
            (
                VirtualKeyCode::Y,
                KeyType::Trigger(TriggerSide::RightTrigger),
            ),
            //DPad:
            (VirtualKeyCode::Up, KeyType::Button(ButtonCode::Up)),
            (VirtualKeyCode::Down, KeyType::Button(ButtonCode::Down)),
            (VirtualKeyCode::Left, KeyType::Button(ButtonCode::Left)),
            (VirtualKeyCode::Right, KeyType::Button(ButtonCode::Right)),
            //Buttons:
            (VirtualKeyCode::U, KeyType::Button(ButtonCode::ButtonA)),
            (VirtualKeyCode::I, KeyType::Button(ButtonCode::ButtonB)),
            (VirtualKeyCode::J, KeyType::Button(ButtonCode::ButtonC)),
            (VirtualKeyCode::K, KeyType::Button(ButtonCode::ButtonD)),
            (VirtualKeyCode::Key5, KeyType::Button(ButtonCode::Start)),
            (VirtualKeyCode::Key6, KeyType::Button(ButtonCode::Select)),
            //Left Stick Axis
            (
                VirtualKeyCode::W,
                KeyType::AnalogStick(AnalogStick::LeftYPositive),
            ),
            (
                VirtualKeyCode::S,
                KeyType::AnalogStick(AnalogStick::LeftYNegative),
            ),
            (
                VirtualKeyCode::A,
                KeyType::AnalogStick(AnalogStick::LeftXNegative),
            ),
            (
                VirtualKeyCode::D,
                KeyType::AnalogStick(AnalogStick::LeftXPositive),
            ),
            //Right Stick Axis,
            (
                VirtualKeyCode::T,
                KeyType::AnalogStick(AnalogStick::RightYPositive),
            ),
            (
                VirtualKeyCode::G,
                KeyType::AnalogStick(AnalogStick::RightYNegative),
            ),
            (
                VirtualKeyCode::F,
                KeyType::AnalogStick(AnalogStick::RightXNegative),
            ),
            (
                VirtualKeyCode::H,
                KeyType::AnalogStick(AnalogStick::RightXPositive),
            ),
        ]
        .into_iter()
        .collect::<HashMap<VirtualKeyCode, KeyType>>()];

        Self { buttons }
    }
}