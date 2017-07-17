use std::cmp::min;
use std::ops::{Index, IndexMut};
use std::str::Chars;
use std::slice::Iter;

use px8::editor::text::cursor::Cursor;

/// A line in a buffer.
pub trait Line<'a> {
    /// The underlying iterator.
    type Iter: Iterator<Item = char> + 'a;

    /// Iterator over characters.
    fn chars_iter(&'a self) -> Self::Iter;
}

impl<'a, T: AsRef<str>> Line<'a> for T {
    type Iter = Chars<'a>;

    fn chars_iter(&self) -> Chars {
        self.as_ref().chars()
    }
}

/// A buffer structure
pub trait TextBuffer<'a> {
    /// The line type of the buffer.
    type Line: 'a + Line<'a>;
    /// The line iterator.
    type LineIter: Iterator<Item = &'a Self::Line>;

    /// Create a new empty split buffer
    fn new() -> Self;

    /// Convert a string to a split buffer
    fn from_str(s: &str) -> Self;

    /// Get the nth line in the buffer by option reference
    fn get_line(&self, n: usize) -> Option<&Self::Line>;

    /// Get the nth line in the buffer by optional mutable reference
    fn get_line_mut(&mut self, n: usize) -> Option<&mut Self::Line>;

    /// Remove the nth line and return it. Panics on out of bound.
    fn remove_line(&mut self, n: usize) -> Self::Line;

    /// Insert line at n. Panics on out of bound.
    fn insert_line(&mut self, n: usize, line: Self::Line);

    /// Convert a vector of lines to a split buffer
    fn from_lines(vec: &[Self::Line]) -> SplitBuffer;

    /// Give a hint on where the operations are most frequent (i.e. where the cursor is). X value.
    fn focus_hint_x(&mut self, x: usize);

    /// Give a hint on where the operations are most frequent (i.e. where the cursor is). Y value.
    fn focus_hint_y(&mut self, y: usize);

    /// Get the number of lines in the buffer.
    fn len(&self) -> usize;

    /// Get an iterator over the lines in the buffer.
    fn lines(&'a self) -> Self::LineIter;

    /// Get the leading whitespaces of the nth line. Used for autoindenting.
    fn get_indent(&self, n: usize) -> &str;
}


/// The buffer data structure, that Sodium is using.
///
/// This structure consists of two "subbuffers", which are just vectors over lines (defined by
/// Strings). The split is called a center.
///
/// The nearer a given operation is to the center, the better it performs.
///
/// The second buffer is in reverse order to get the particular efficiency we want.
pub struct SplitBuffer {
    before: Vec<String>,
    after: Vec<String>,
    #[cfg(debug)]
    _hinted_since_edit: bool,
}

impl SplitBuffer {
    fn up(&mut self) {
        self.after.push(self.before.pop().expect("Popped last element"));
    }

    fn down(&mut self) {
        self.before.push(self.after.pop().expect("Popped last element"));
    }

    fn y(&self) -> usize {
        self.before.len()
    }
}

// TODO remove
impl SplitBuffer {
    /// Convert the buffer to a string.
    pub fn to_string(&self) -> String {
        self.lines().map(|x| x.to_owned() + "\n").collect()
    }
}

impl<'a> TextBuffer<'a> for SplitBuffer {
    type Line = String;
    type LineIter = SplitBufIter<'a>;

    fn new() -> Self {
        SplitBuffer {
            before: vec![String::new()],
            after: Vec::new(),
        }
    }

    fn from_str(s: &str) -> Self {
        SplitBuffer {
            before: s.lines().map(ToOwned::to_owned).collect(),
            after: Vec::new(),
        }
    }

    fn get_line(&self, n: usize) -> Option<&String> {
        if n < self.before.len() {
            Some(&self.before[n])
        } else if n < self.len() {
            let n = self.len() - 1 - n;
            Some(&self.after[n])
        } else {
            None
        }
    }

    fn get_line_mut(&mut self, n: usize) -> Option<&mut String> {
        if n < self.before.len() {
            Some(&mut self.before[n])
        } else if n < self.len() {
            let n = self.len() - 1 - n;
            Some(&mut self.after[n])
        } else {
            None
        }
    }

    fn remove_line(&mut self, n: usize) -> String {
        if n < self.before.len() {
            self.before.remove(n)
        } else if n < self.len() {
            let n = self.len() - 1 - n;
            self.after.remove(n)
        } else {
            panic!("Out of bound");
        }
    }

    fn insert_line(&mut self, n: usize, line: String) {
        if n < self.before.len() {
            self.before.insert(n, line);
        } else if n <= self.len() {
            let n = self.len() - n;
            self.after.insert(n, line);
        } else {
            panic!("Out of bound");
        }
    }

    fn from_lines(ln: &[String]) -> SplitBuffer {
        SplitBuffer {
            before: ln.to_owned(),
            after: Vec::new(),
        }
    }

    fn focus_hint_y(&mut self, y: usize) {
        if y < self.y() {
            for _ in 0..min(self.y() - y, self.before.len()) {
                self.up();
            }
        } else if y > self.y() {
            for _ in 0..min(y - self.y(), self.after.len()) {
                self.down();
            }
        } else if y >= self.len() {
            panic!("Out of bound");
        }
    }

    fn focus_hint_x(&mut self, _: usize) {}

    fn len(&self) -> usize {
        self.before.len() + self.after.len()
    }

    fn lines(&'a self) -> SplitBufIter<'a> {
        SplitBufIter {
            buffer: self,
            line: 0,
        }
    }

    fn get_indent(&self, n: usize) -> &str {
        if let Some(ln) = self.get_line(n) {
            let mut len = 0;
            for c in ln.chars() {
                match c {
                    '\t' | ' ' => len += 1,
                    _          => break,
                }
            }
            &ln[..len]
        } else {
            ""
        }
    }
}


impl Index<usize> for SplitBuffer {
    type Output = String;

    fn index<'a>(&'a self, index: usize) -> &'a String {
        self.get_line(index).expect("Out of bound")
    }
}
impl IndexMut<usize> for SplitBuffer {

    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut String {
        #[cfg(debug)]
        fn debug_check(b: &mut SplitBuffer) {
            if b._hinted_since_edit {
                b._hinted_since_edit = false;
            } else {
                panic!("No focus hint given since last edit!");
            }
        }

        #[cfg(not(debug))]
        fn debug_check(_: &mut SplitBuffer) {}

        debug_check(&mut *self);

        self.get_line_mut(index).expect("Out of bound")
    }
}

/// A iterator over the lines of a split buffer
pub struct SplitBufIter<'a> {
    buffer: &'a SplitBuffer,
    line: usize,
}

impl<'a> Iterator for SplitBufIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<&'a String> {
        self.nth(1)
    }

    fn nth(&mut self, n: usize) -> Option<&'a String> {
        let res = self.buffer.get_line(self.line);
        self.line += n;

        res
    }

    fn count(self) -> usize {
        self.buffer.len()
    }
}

impl<'a> DoubleEndedIterator for SplitBufIter<'a> {
    fn next_back(&mut self) -> Option<&'a String> {
        if self.line == 0 {
            None
        } else {
            self.line -= 1;
            self.buffer.get_line(self.line)
        }
    }
}

/// A SplitBuffer and related state
pub struct Buffer {
    /// The document
    pub raw_buffer: SplitBuffer,
    /// The current cursor
    pub current_cursor: u8,
    /// The cursors
    pub cursors: Vec<Cursor>,
    /// The x coordinate of the scroll
    pub scroll_x: usize,
    /// The y coordinate of the scroll
    pub scroll_y: usize,
    /// The title of the document
    pub title: Option<String>,
    /// True if the buffer is transient and should be deleted when
    /// it is no longer the current buffer.
    pub is_transient: bool,
}

impl Buffer {
    /// Create a new Buffer with default values.
    fn new() -> Buffer {
        Buffer {
            raw_buffer: SplitBuffer::new(),
            current_cursor: 0,
            cursors: vec![Cursor::new()],
            scroll_x: 0,
            scroll_y: 0,
            title: None,
            is_transient: false,
        }
    }
}

impl From<SplitBuffer> for Buffer {
    fn from(b: SplitBuffer) -> Buffer {
        let mut info = Buffer::new();
        info.raw_buffer = b;

        info
    }
}

/// Provides access to buffer manipulation functions.
pub struct BufferManager {
    buffers: Vec<Buffer>,
    current_buffer_index: usize,
}

impl BufferManager {
    /// Create a new BufferManager with default values.
    pub fn new() -> BufferManager {
        BufferManager {
            buffers: vec![Buffer::new()],
            current_buffer_index: 0,
        }
    }

    /// Adds the specified buffer to the set of buffers and returns
    /// its index.
    pub fn new_buffer(&mut self, buffer: Buffer) -> usize {
        self.buffers.push(buffer);

        self.buffers.len() - 1
    }

    /// Returns an iterator over the buffers.
    pub fn iter(&self) -> Iter<Buffer> {
        self.buffers.iter()
    }

    /// Gets the number of buffers.
    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    /// Gets the index of the current buffer.
    pub fn current_buffer_index(&self) -> usize {
        self.current_buffer_index
    }

    /// Get a reference to the currently open buffer.
    pub fn current_buffer(&self) -> &SplitBuffer {
        &self.current_buffer_info().raw_buffer
    }

    /// Get a mutable reference to the currently open buffer.
    pub fn current_buffer_mut(&mut self) -> &mut SplitBuffer {
        &mut self.current_buffer_info_mut().raw_buffer
    }

    /// Get a reference to the currently open buffer information.
    pub fn current_buffer_info(&self) -> &Buffer {
        &self.buffers[self.current_buffer_index]
    }

    /// Get a mutable reference to the currently open buffer information.
    pub fn current_buffer_info_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current_buffer_index]
    }

    /// Switch the current buffer to the specified buffer
    pub fn switch_to(&mut self, n: usize) {
        debug_assert!(n < self.buffers.len(), "Buffer index out of bounds");

        // if the current view is transient, delete it
        let mut n = n;
        if self.current_buffer_info().is_transient {
            let index = self.current_buffer_index;
            self.delete_buffer(index);

            // if the current view is less than the view to switch to
            // then we need to account for the view we just removed
            if index <= n {
                n -= 1;
            }
        }

        self.current_buffer_index = n;
    }

    /// Delete the specified buffer
    pub fn delete_buffer(&mut self, n: usize) {
        assert!(n < self.buffers.len(), "Buffer index out of bounds");

        self.buffers.remove(n);

        if self.buffers.len() == 0 {
            self.buffers.push(Buffer::new());
            self.current_buffer_index = 0;
        } else if self.current_buffer_index <= n {
            self.current_buffer_index  -= 1;
        }
    }

    /// Validates that the specifed buffer index is valid
    pub fn is_buffer_index_valid(&self, n: usize) -> bool {
        n < self.buffers.iter().filter(|b| !b.is_transient).count()
    }
}