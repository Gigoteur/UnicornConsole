use px8::editor::text::TextEditor;
use px8::editor::text::buffers::TextBuffer;

impl TextEditor {
    /// Goto a given position. Does not automatically bound.
    #[inline]
    pub fn goto(&mut self, (x, y): (usize, usize)) {
        self.cursor_mut().y = y;
        self.cursor_mut().x = x;
    }

    /// Get the previous position, i.e. the position before the cursor (*not* left to the cursor).
    /// Includes newline positions.
    #[inline]
    pub fn previous(&self, n: usize) -> Option<(usize, usize)> {
        self.before(n, self.pos())
    }
    /// Get the next position, i.e. the position after the cursor (*not* right to the cursor)
    #[inline]
    pub fn next(&self, n: usize) -> Option<(usize, usize)> {
        self.after(n, self.pos())
    }

    /// Get position after a given position, i.e. a generalisation of .next()
    #[inline]
    pub fn after(&self, n: usize, (x, y): (usize, usize)) -> Option<(usize, usize)> {

        // TODO: Make this more idiomatic {
        if x + n < self.buffers.current_buffer()[y].len() {

            Some((x + n, y))
        } else {
            if y + 1 >= self.buffers.current_buffer().len() {
                None
            } else {
                let mut mv = n + x - self.buffers.current_buffer()[y].len();
                let mut ry = y + 1;

                loop {
                    if mv < self.buffers.current_buffer()[ry].len() {
                        return Some((mv, ry));
                    } else {
                        if ry + 1 < self.buffers.current_buffer().len() {
                            mv -= self.buffers.current_buffer()[ry].len();
                            ry += 1;
                        } else {
                            return None;
                        }
                    }
                }

            }
        }
        // }
    }

    /// Get the position before a given position, i.e. a generalisation .before(). Includes
    /// newline positions.
    #[inline]
    pub fn before(&self, n: usize, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        if x >= n {
            Some((x - n, y))
        } else {
            if y == 0 {
                None
            } else {
                let mut mv = n - x - 1;
                let mut ry = y - 1;

                loop {
                    if mv <= self.buffers.current_buffer()[ry].len() {
                        return Some((self.buffers.current_buffer()[ry].len() - mv, ry));
                    } else {
                        if ry > 0 && mv >= self.buffers.current_buffer()[ry].len() {
                            mv -= self.buffers.current_buffer()[ry].len();
                            ry -= 1;
                        } else if ry == 0 {
                            return None;

                        }
                    }
                }
            }
        }
    }

    /// Get the position of the character right to the cursor (horizontally bounded)
    #[inline]
    pub fn right(&self, n: usize, tight: bool) -> (usize, usize) {
        self.bound_hor((self.x() + n, self.y()), tight)
    }
    /// Get the position of the character right to the cursor (unbounded)
    #[inline]
    pub fn right_unbounded(&self, n: usize) -> (isize, isize) {
        ((self.x() + n) as isize, self.y() as isize)
    }

    /// Get the position of the character left to the cursor (horizontally bounded)
    #[inline]
    pub fn left(&self, n: usize) -> (usize, usize) {
        if n <= self.x() {
            (self.x() - n, self.y())
        } else {
            (0, self.y())
        }
    }
    /// Get the position of the character left to the cursor (unbounded)
    #[inline]
    pub fn left_unbounded(&self, n: usize) -> (isize, isize) {
        (self.x() as isize - n as isize, self.y() as isize)
    }

    /// Get the position of the character above the cursor (vertically bounded)
    #[inline]
    pub fn up(&self, n: usize) -> (usize, usize) {
        if n <= self.y() {
            (self.cursor().x, self.y() - n)
        } else {
            (self.cursor().x, 0)
        }
    }
    /// Get the position of the character above the cursor (unbounded)
    #[inline]
    pub fn up_unbounded(&self, n: usize) -> (isize, isize) {
        (self.cursor().x as isize, self.y() as isize - n as isize)
    }

    /// Get the position of the character under the cursor (vertically bounded)
    #[inline]
    pub fn down(&self, n: usize) -> (usize, usize) {
        self.bound_ver((self.cursor().x, self.y() + n))
    }
    /// Get the position of the character above the cursor (unbounded)
    #[inline]
    pub fn down_unbounded(&self, n: usize) -> (isize, isize) {
        (self.cursor().x as isize, self.y() as isize + n as isize)

    }

    /// Get n'th next ocurrence of a given charecter (relatively to the cursor)
    pub fn next_ocur(&self, c: char, n: usize) -> Option<usize> {
        let mut dn = 0;
        let mut x  = self.x();

        for (i, ch) in self.buffers.current_buffer()[self.y()].chars().skip(x).enumerate() {
            if ch == c {
                if i > 0 {
                    dn += 1;
                    if dn == n {
                        x += i;
                        return Some(x);
                    }
                }
            }
        }

        None
    }

    /// Get n'th previous ocurrence of a given charecter (relatively to the cursor)
    pub fn previous_ocur(&self, c: char, n: usize) -> Option<usize> {
        let mut dn = 0;
        let mut x  = self.x();
        let y      = self.y();

        for (i, ch) in self.buffers.current_buffer()[y].chars().rev().skip(self.buffers.current_buffer()[y].len() - x).enumerate() {
            if ch == c {
                dn += 1;
                if dn == n {
                    x -= i + 1;
                    return Some(x);
                }
            }
        }

        None
    }

    /// Get next WORD forward
    /// "A WORD consists of a sequence of non-blank characters, separated with
    /// whitespace.  An empty line is also considered to be a WORD."
    pub fn _next_word_forward(&self, n: usize) -> Option<usize> {
        let mut dn = 0;
        let mut x  = self.x();

        for (i, ch) in self.buffers.current_buffer()[self.y()].chars().skip(x).enumerate() {
            if ch.is_whitespace() {
                dn += 1;
                if dn == n {
                    x += i + 1;
                    return Some(x);
                }
            }
        }

        None
    }

}