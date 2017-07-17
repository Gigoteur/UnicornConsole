use px8::editor::text::TextEditor;

#[derive(Clone)]
/// A cursor, i.e. a state defining a mode, and a position. The cursor does not define the content
/// of the current file.
pub struct Cursor {
    /// The x coordinate of the cursor
    pub x: usize,
    /// The y coordinate of the cursor
    pub y: usize,
}

impl Cursor {
    /// Create a new default cursor
    pub fn new() -> Cursor {
        Cursor {
            x: 0,
            y: 0,
        }
    }
}

impl TextEditor {
    /// Get the character under the cursor
    #[inline]
    pub fn current(&self) -> Option<char> {
        let (x, y) = self.pos();
        match self.buffers.current_buffer()[y].chars().nth(x) {
            Some(c) => Some(c),
            None => None,
        }
    }

    /// Get the current cursor
    #[inline]
    pub fn cursor(&self) -> &Cursor {
        let buffer = self.buffers.current_buffer_info();
        buffer.cursors.get(buffer.current_cursor as usize).unwrap()
    }

    /// Get the current cursor mutably
    #[inline]
    pub fn cursor_mut(&mut self) -> &mut Cursor {
        let buffer = self.buffers.current_buffer_info_mut();
        buffer.cursors.get_mut(buffer.current_cursor as usize).unwrap()
    }

    /// Go to next cursor
    #[inline]
    pub fn next_cursor(&mut self) {
        let buffer = self.buffers.current_buffer_info_mut();
        buffer.current_cursor = (buffer.current_cursor.wrapping_add(1)) %
                                (buffer.cursors.len() as u8);
    }

    /// Go to previous cursor
    #[inline]
    pub fn prev_cursor(&mut self) {
        let buffer = self.buffers.current_buffer_info_mut();
        if buffer.current_cursor != 0 {
            buffer.current_cursor -= 1;
        } else {
            buffer.current_cursor = (buffer.cursors.len() - 1) as u8;
        }
    }
}