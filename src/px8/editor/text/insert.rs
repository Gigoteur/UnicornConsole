use px8::editor::text::TextEditor;
use px8::editor::text::cursor::Cursor;
use px8::editor::text::buffers::TextBuffer;

impl TextEditor {
    pub fn insert_str(&mut self, txt: String) {
        for c in txt.chars() {
            self.insert(c);
        }
    }

    pub fn insert(&mut self, c: char) {
        let (mut x, mut y) = self.pos();
        self.buffers.current_buffer_mut()[y].insert(x, c);

        let right = self.right(1, false);
        self.goto(right);

        self.hint();
    }

    #[inline]
    pub fn delete(&mut self) {
        let &Cursor{ x, y, .. } = self.cursor();
        if x == self.buffers.current_buffer()[y].len() {
            if y + 1 < self.buffers.current_buffer().len() {
                let s = self.buffers.current_buffer_mut().remove_line(y + 1);
                self.buffers.current_buffer_mut()[y].push_str(&s);
            }
        } else if x < self.buffers.current_buffer()[y].len() {
            self.buffers.current_buffer_mut()[y].remove(x);
        }

        self.hint();
    }

    /// Backspace.
    #[inline]
    pub fn backspace(&mut self) {
        let previous = self.previous(1);
        if let Some(p) = previous {
            self.goto(p);
            self.delete();
        }
    }
}