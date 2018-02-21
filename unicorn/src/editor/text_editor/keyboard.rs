use config::scancode::Scancode;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Tab,
    Enter,
    Esc,
    Backspace,
    Right,
    Left,
    Down,
    Up,
    Delete,

    Char(char),
    Ctrl(char),
}

impl Key {
    pub fn from_special_code(scancode: Scancode) -> Option<Key> {
        match scancode {
            Scancode::Left => Some(Key::Left),
            Scancode::Right => Some(Key::Right),
            Scancode::Up => Some(Key::Up),
            Scancode::Down => Some(Key::Down),
            Scancode::Delete => Some(Key::Delete),
            Scancode::Backspace => Some(Key::Backspace),
            Scancode::Return => Some(Key::Enter),
            Scancode::KpEnter => Some(Key::Enter),
            Scancode::Tab => Some(Key::Tab),

            _ => None,
        }
    }
}