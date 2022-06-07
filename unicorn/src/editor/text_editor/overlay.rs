use unicode_width::UnicodeWidthChar;
use gfx::Screen;

use editor::text_editor::keyboard::Key;

/// State for the overlay
pub enum OverlayEvent {
    Finished(Option<String>),
    Ok,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OverlayType {
    Prompt,
    SelectFile,
}


/// An interface for user interaction
///
/// This can be a prompt, autocompletion list, anything thatn requires input
/// from the user.
pub enum Overlay {
    Prompt {
        cursor_x: usize,
        data: String,
        prefix: &'static str,
    },

    SavePrompt {
        cursor_x: usize,
        data: String,
        prefix: &'static str,
    },

    SelectFile {
        cursor_x: usize,
        data: String,
        prefix: &'static str,
    },

    None,
}

impl Overlay {
    pub fn draw(&self, rb: &mut Screen) {
        match *self {
            Overlay::SelectFile     {prefix, ref data, ..} |
            Overlay::Prompt         {prefix, ref data, ..} |
            Overlay::SavePrompt     {prefix, ref data, ..} => {
                let height = (rb.height - 1) as i32;
                let offset = prefix.len();

                // draw the given prefix
                for (index, ch) in prefix.chars().enumerate() {
                    rb.print_char(ch, index as i32, height, 7);
                }

                // draw the overlay data
                for (index, ch) in data.chars().enumerate() {
                    rb.print_char(ch, index as i32 + offset as i32, height, 7);
                }
            }

            _ => {}
        }
    }

    pub fn draw_cursor(&mut self, _rb: &mut Screen) {
        match *self {
            Overlay::SelectFile     {cursor_x: _, ..} |
            Overlay::Prompt         {cursor_x: _, ..} |
            Overlay::SavePrompt     {cursor_x: _, ..} => {
                // Prompt is always on the bottom, so we can use the
                // height given by the frontend here
                //let height = (rb.height - 1) as i32;
                //rb.set_cursor(cursor_x as isize, height as isize)
            },

            _ => {}
        }
    }

    pub fn handle_key_event(&mut self, key: Key) -> OverlayEvent {
        match *self {
            Overlay::SelectFile {ref mut cursor_x, ref mut data, ..} |
            Overlay::Prompt     {ref mut cursor_x, ref mut data, ..} |
            Overlay::SavePrompt {ref mut cursor_x, ref mut data, ..} => {
                match key {
                    Key::Esc => return OverlayEvent::Finished(None),
                    Key::Backspace => {
                        if let Some(c) = data.pop() {
                            if let Some(width) = UnicodeWidthChar::width(c) {
                                *cursor_x -= width;
                            }
                        }
                    }
                    Key::Enter => {
                        // FIXME: dont clone
                        let data = data.clone();
                        return OverlayEvent::Finished(Some(data))
                    }
                    Key::Char(c) => {
                        if let Some(width) = UnicodeWidthChar::width(c) {
                            data.push(c);
                            *cursor_x += width;
                        }
                    }
                    _ => {}
                }
            }

            _ => {}
        }
        OverlayEvent::Ok
    }
}
