use px8::editor::text::TextEditor;
use px8::editor::text::buffers::TextBuffer;

/// Convert a usize tuple to isize
pub fn to_signed_pos((x, y): (usize, usize)) -> (isize, isize) {
    (x as isize, y as isize)
}

impl TextEditor {
    /// Get the position of the current cursor, bounded
    #[inline]
    pub fn pos(&self) -> (usize, usize) {
        let cursor = self.cursor();
        self.bound((cursor.x, cursor.y), false)
    }

    #[inline]
    /// Get the X coordinate of the current cursor (bounded)
    pub fn x(&self) -> usize {
        self.pos().0
    }

    #[inline]
    /// Get the Y coordinate of the current cursor (bounded)
    pub fn y(&self) -> usize {
        self.pos().1
    }

    /// Convert a position value to a bounded position value
    #[inline]
    pub fn bound(&self, (x, mut y): (usize, usize), tight: bool) -> (usize, usize) {
        y = if y >= self.buffers.current_buffer().len() {
            self.buffers.current_buffer().len() - 1
        } else {
            y
        };

        let ln = self.buffers.current_buffer()[y].len() + if tight {0} else {1};
        if x >= ln {
            if ln == 0 {
                (0, y)
            } else {
                (ln - 1, y)
            }
        } else {
            (x, y)
        }
    }

    /// Bound horizontally, i.e. don't change the vertical axis only make sure that the horizontal
    /// axis is bounded.
    #[inline]
    pub fn bound_hor(&self, (x, y): (usize, usize), tight: bool) -> (usize, usize) {
        (self.bound((x, y), tight).0, y)
    }
    /// Bound vertically, i.e. don't change the horizontal axis only make sure that the vertical
    /// axis is bounded.
    #[inline]
    pub fn bound_ver(&self, (x, mut y): (usize, usize)) -> (usize, usize) {

        // Is this premature optimization? Yes, yes it is!
        y = if y > self.buffers.current_buffer().len() - 1 {
            self.buffers.current_buffer().len() - 1
        } else {
            y
        };

        (x, y)
    }
}
