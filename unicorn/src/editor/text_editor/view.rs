use std::cmp;
use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;
use std::io::Write;
use std::fs::{File, rename};
use std::sync::{Mutex, Arc};
use std::time::SystemTime;
use std::rc::Rc;

use unicode_width::UnicodeWidthChar;
use std::collections::HashMap;

#[cfg(feature = "syntect")]
use syntect::highlighting::{ThemeSet, Style};
#[cfg(feature = "syntect")]
use syntect::easy::HighlightLines;

use gfx::Screen;
use editor::text_editor::buffer::{Buffer, Mark};
use editor::text_editor::overlay::{Overlay, OverlayType};
use editor::text_editor::utils;
use editor::text_editor::textobject::{Anchor, TextObject, Kind, Offset};
use unicorn::Palettes;


/// A View is an abstract Window (into a Buffer).
///
/// It draws a portion of a Buffer to a `UIBuffer` which in turn is drawn to the
/// screen. It maintains the status bar for the current view, the "dirty status"
/// which is whether the buffer has been modified or not and a number of other
/// pieces of information.
#[cfg(not(feature = "syntect"))]
pub struct View {
    pub buffer: Arc<Mutex<Buffer>>,
    pub last_buffer: Option<Arc<Mutex<Buffer>>>,
    pub overlay: Overlay,
    height: usize,
    width: usize,
    /// First character of the top line to be displayed
    top_line: Mark,
    /// Index into the top_line - used for horizontal scrolling
    left_col: usize,
    /// The current View's cursor - a reference into the Buffer
    cursor: Mark,
    /// Number of lines from the top/bottom of the View after which vertical
    /// scrolling begins.
    threshold: usize,
    /// Message to be displayed in the status bar along with the time it
    /// was displayed.
    message: Option<(&'static str, SystemTime)>,
    colors: HashMap<String, i32>,
}
#[cfg(feature = "syntect")]
pub struct View {
    pub buffer: Arc<Mutex<Buffer>>,
    pub last_buffer: Option<Arc<Mutex<Buffer>>>,
    pub overlay: Overlay,
    height: usize,
    width: usize,
    /// First character of the top line to be displayed
    top_line: Mark,
    /// Index into the top_line - used for horizontal scrolling
    left_col: usize,
    /// The current View's cursor - a reference into the Buffer
    cursor: Mark,
    /// Number of lines from the top/bottom of the View after which vertical
    /// scrolling begins.
    threshold: usize,
    /// Message to be displayed in the status bar along with the time it
    /// was displayed.
    message: Option<(&'static str, SystemTime)>,
    themes: Rc<ThemeSet>,
    theme_name: String,
    colors: HashMap<String, i32>,
}

impl View {
    #[cfg(not(feature = "syntect"))]
    pub fn new(buffer: Arc<Mutex<Buffer>>, width: usize, height: usize) -> View {
        let cursor = Mark::Cursor(0);
        let top_line = Mark::DisplayMark(0);

        {
            let mut b = buffer.lock().unwrap();

            b.set_mark(cursor, 0);
            b.set_mark(top_line, 0);
        }

        View {
            buffer: buffer,
            last_buffer: None,
            top_line: top_line,
            left_col: 0,
            cursor: cursor,
            overlay: Overlay::None,
            threshold: 5,
            message: None,
            height: height,
            width: width,
            colors: HashMap::new(),
        }
    }


    #[cfg(feature = "syntect")]
    pub fn new(buffer: Arc<Mutex<Buffer>>, themes: Rc<ThemeSet>, theme_name: String, width: usize, height: usize) -> View {
        let cursor = Mark::Cursor(0);
        let top_line = Mark::DisplayMark(0);

        {
            let mut b = buffer.lock().unwrap();

            b.set_mark(cursor, 0);
            b.set_mark(top_line, 0);
        }

        View {
            buffer: buffer,
            last_buffer: None,
            top_line: top_line,
            left_col: 0,
            cursor: cursor,
            overlay: Overlay::None,
            threshold: 5,
            message: None,
            themes: themes,
            theme_name: theme_name,
            height: height,
            width: width,
            colors: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.colors.clear();
    }
    
    pub fn set_buffer(&mut self, buffer: Arc<Mutex<Buffer>>) {
        self.last_buffer = Some(self.buffer.clone());

        {
            let mut b = buffer.lock().unwrap();

            b.set_mark(self.cursor, 0);
            b.set_mark(self.top_line, 0);
        }

        self.buffer = buffer;
    }

    pub fn switch_last_buffer(&mut self) {
        let buffer = self.buffer.clone();
        let last_buffer = match self.last_buffer.clone() {
            Some(buf) => buf,
            None => return
        };

        self.buffer = last_buffer;
        self.last_buffer = Some(buffer);
    }

    /// Get the height of the View.
    ///
    /// This is the height of the UIBuffer minus the status bar height.
    pub fn get_height(&self) -> usize {
        self.height - 1
    }

    /// Get the width of the View.
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Resize the view
    ///
    /// This involves simply changing the size of the associated UIBuffer
    pub fn resize(&mut self, width: usize, height: usize) {
        self.height = height;
        self.width = width;
    }

    /// Clear the buffer
    ///
    /// Fills every cell in the UIBuffer with the space (' ') char.
    pub fn clear(&mut self, rb: &mut Screen) {
        for row in 0..self.height {
            for col in 0..self.width {
                rb.print_char(' ', col as i32, row as i32, 7);
            }
        }
    }

    #[cfg(not(feature = "syntect"))]
    pub fn draw(&mut self, rb: &mut Screen, palettes: Arc<Mutex<Palettes>>, syntax_enabled: bool) {
       self.draw_cursor(rb);
       // self.clear(rb);
        {
            let buffer = self.buffer.lock().unwrap();
            let height = self.get_height() - 1;
            let width = self.get_width() - 1;

            let mut lines = buffer.lines_from(self.top_line).unwrap().take(height);
            for y_position in 0..height {
                let line = lines.next().unwrap_or_else(Vec::new);
                draw_line(rb, &line, y_position as usize, self.left_col);
            }

        }


        self.draw_status(rb);
    }

    #[cfg(feature = "syntect")]
    pub fn draw(&mut self, rb: &mut Screen, palettes: Arc<Mutex<Palettes>>, syntax_enabled: bool) {
        self.draw_cursor(rb);

       // self.clear(rb);
        {
            let buffer = self.buffer.lock().unwrap();
            let height = self.get_height() - 1;
            let width = self.get_width() - 1;

            let mut lines = buffer.lines_from(self.top_line).unwrap().take(height);
            for y_position in 0..height {
                let line = lines.next().unwrap_or_else(Vec::new);
                if syntax_enabled {
                    if let Some(ref syntax) = buffer.syntax {
                        let line_str = String::from_utf8(line).unwrap();
                        let mut h = HighlightLines::new(syntax, &self.themes.themes[&*self.theme_name]);
                        let ranges: Vec<(Style, &str)> = h.highlight(&line_str);
                        let mut x = 0;

                        for (style, text) in ranges {
                            let fg = format!("{0:02.x}{1:02.x}{2:02.x}",
                                              style.foreground.r, style.foreground.g, style.foreground.b);
                           // let fg = Color::Byte(utils::rgb_to_short(&*fg) as u16);
                          //  let bg = format!("{0:02.x}{1:02.x}{2:02.x}",
                          //                   style.background.r, style.background.g, style.background.b);
                           // let bg = Color::Byte(utils::rgb_to_short(&*bg) as u16);

                           if !self.colors.contains_key(&fg) {
                               let color: u32 = self.colors.len() as u32 + 16;
                               self.colors.insert(fg.clone(), color as i32);
                               palettes.lock().unwrap().set_color(color, style.foreground.r, style.foreground.g, style.foreground.b);
                           }

                           let color = self.colors[&fg];

                            for ch in text.chars().skip(self.left_col) {
                                match ch {
                                    '\t' => {
                                        let w = 4 - x % 4;
                                        for _ in 0..w {
                                            print_char(rb, ' ', x as i32, y_position as i32, 7);
                                            x += 1;
                                        }
                                    }
                                    '\n' => {
                                        // Replace any cells after end of line with ' '
                                        while x <= width as i32 {
                                            print_char(rb, ' ', x as i32, y_position as i32, 7);
                                            x += 1;
                                        }
                                    }
                                    _ => {
                                        if x >= width as i32 {
                                            break;
                                        }
                                        print_char(rb, ch, x as i32, y_position as i32, color);
                                        x += 1;
                                    }
                                }
                                if x >= width as i32 {
                                    break;
                                }
                            }


                        }
                        // If the line is too long to fit on the screen, show an indicator
                        let indicator = if line_str.len() > width + self.left_col { '→' } else { ' ' };
                        print_char(rb, indicator, width as i32, y_position as i32, 7);
                    } else {
                        draw_line(rb, &line, y_position as usize, self.left_col);
                    }
                } else {
                    draw_line(rb, &line, y_position as usize, self.left_col);
                }
            }

        }


        self.draw_status(rb);
    }

    #[cfg_attr(feature="clippy", allow(needless_range_loop))]
    fn draw_status(&mut self, rb: &mut Screen) {
        let buffer = self.buffer.lock().unwrap();
        let buffer_status = buffer.status_text();
        let mut cursor_status = buffer.get_mark_display_coords(self.cursor).unwrap_or((0,0));
        cursor_status = (cursor_status.0 + 1, cursor_status.1 + 1);
        let status_text = format!("{} ({}, {})", buffer_status, cursor_status.0, cursor_status.1).into_bytes();
        let status_text_len = status_text.len();
        let width = self.get_width();
        let height = self.get_height() - 1;


        for index in 0..width*4 {
            let ch: char = if index < status_text_len {
                status_text[index] as char
            } else { ' ' };
            print_char(rb, ch, index as i32 * 4, height as i32 * 7, 9);
        }

        if buffer.dirty {
            let data = ['[', '*', ']'];
            for (idx, ch) in data.iter().enumerate() {
                print_char(rb, *ch, (status_text_len + idx + 1) as i32, height as i32 * 7, 8);
            }
        }
        if let Some((message, _time)) = self.message {
            for (offset, ch) in message.chars().enumerate() {
                print_char(rb, ch, offset as i32, height as i32 + 1, 7);
            }
        }
    }

    fn draw_cursor(&mut self, rb: &mut Screen) {
        let buffer = self.buffer.lock().unwrap();
        if let Some(top_line) = buffer.get_mark_display_coords(self.top_line) {
            if let Some((x, y)) = buffer.get_mark_display_coords(self.cursor) {
                let mut x1 = (x - self.left_col) as i32;
                let mut y1 = y as i32 - top_line.1 as i32;

                x1 = x1 * 4;
                y1 = y1 * 7;
                
                rb.rectfill(x1, 16+y1, x1+4, 16+y1+7, 3);
            }
        }
    }

    pub fn set_overlay(&mut self, overlay_type: OverlayType) {
        match overlay_type {
            OverlayType::Prompt => {
                self.overlay = Overlay::Prompt {
                    cursor_x: 1,
                    prefix: ":",
                    data: String::new(),
                };
            }

            OverlayType::SelectFile => {
                let prefix = "Enter file path:";

                self.overlay = Overlay::SelectFile {
                    cursor_x: prefix.len(),
                    prefix: prefix,
                    data: String::new(),
                };
            }
        }
    }

    /// Display the given message
    pub fn show_message(&mut self, message: &'static str) {
        self.message = Some((message, SystemTime::now()));
    }

    /// Clear the currently displayed message if it has been there for 5 or more seconds
    ///
    /// Does nothing if there is no message, or of the message has been there for
    /// less that five seconds.
    pub fn maybe_clear_message(&mut self) {
        if let Some((_message, time)) = self.message {
            if let Ok(elapsed) = time.elapsed() {
                if elapsed.as_secs() >= 5 {
                    self.message = None;
                }
            }
        }
    }

    pub fn move_mark(&mut self, mark: Mark, object: TextObject) {
        self.buffer.lock().unwrap().set_mark_to_object(mark, object);
        self.maybe_move_screen();
    }

    /// Update the top_line mark if necessary to keep the cursor on the screen.
    fn maybe_move_screen(&mut self) {
        let mut buffer = self.buffer.lock().unwrap();
        if let (Some(cursor), Some((_, top_line))) = (buffer.get_mark_display_coords(self.cursor),
                                                      buffer.get_mark_display_coords(self.top_line)) {

            let width  = (self.get_width()  - self.threshold) as isize;
            let height = (self.get_height() - self.threshold) as isize;

            //left-right shifting
            self.left_col = match cursor.0 as isize - self.left_col as isize {
                x_offset if x_offset < self.threshold as isize => {
                    cmp::max(0, self.left_col as isize - (self.threshold as isize - x_offset)) as usize
                }
                x_offset if x_offset >= width => {
                    self.left_col + (x_offset - width + 1) as usize
                }
                _ => { self.left_col }
            };

            //up-down shifting
            match cursor.1 as isize - top_line as isize {
                y_offset if y_offset < self.threshold as isize && top_line > 0 => {
                    let amount = (self.threshold as isize - y_offset) as usize;
                    let obj = TextObject {
                        kind: Kind::Line(Anchor::Same),
                        offset: Offset::Backward(amount, self.top_line)
                    };
                    buffer.set_mark_to_object(self.top_line, obj);
                }
                y_offset if y_offset >= height => {
                    let amount = (y_offset - height + 1) as usize;
                    let obj = TextObject {
                        kind: Kind::Line(Anchor::Same),
                        offset: Offset::Forward(amount, self.top_line)
                    };
                    buffer.set_mark_to_object(self.top_line, obj);
                }
                _ => { }
            }
        }
    }

    // Delete chars from the first index of object to the last index of object
    pub fn delete_object(&mut self, object: TextObject) {
        self.buffer.lock().unwrap().remove_object(object);
    }

    pub fn delete_from_mark_to_object(&mut self, mark: Mark, object: TextObject) {
        let mut buffer = self.buffer.lock().unwrap();
        if let Some(mark_pos) = buffer.get_object_index(object) {
            if let Some(midx) = buffer.get_mark_idx(mark) {
                buffer.remove_from_mark_to_object(mark, object);
                buffer.set_mark(mark, cmp::min(mark_pos.absolute, midx));
            }
        }
    }

    /// Insert a chacter into the buffer & update cursor position accordingly.
    pub fn insert_char(&mut self, ch: char) {
        self.buffer.lock().unwrap().insert_char(self.cursor, ch as u8);
        // NOTE: the last param to char_width here may not be correct
        if let Some(ch_width) = utils::char_width(ch, false, 4, 1) {
            let obj = TextObject {
                kind: Kind::Char,
                offset: Offset::Forward(ch_width, Mark::Cursor(0))
            };
            self.move_mark(Mark::Cursor(0), obj)
        }
    }

    pub fn undo(&mut self) {
        {
            let mut buffer = self.buffer.lock().unwrap();
            let point = if let Some(transaction) = buffer.undo() { transaction.end_point }
                        else { return; };
            buffer.set_mark(self.cursor, point);
        }
        self.maybe_move_screen();
    }

    pub fn redo(&mut self) {
        {
            let mut buffer = self.buffer.lock().unwrap();
            let point = if let Some(transaction) = buffer.redo() { transaction.end_point }
                        else { return; };
            buffer.set_mark(self.cursor, point);
        }
        self.maybe_move_screen();
    }

    pub fn get_buffer(&mut self) -> Vec<String> {
        let mut data = Vec::new();

        let buffer = self.buffer.lock().unwrap();
        for line in buffer.lines() {
           // data += &String::from_utf8(line).unwrap();
           data.push(String::from_utf8(line).unwrap());
        }

        data
    }

    /// Whether or not the current buffer has unsaved changes
    pub fn buffer_is_dirty(&mut self) -> bool {
        self.buffer.lock().unwrap().dirty
    }

}

pub fn draw_line(rb: &mut Screen, line: &[u8], idx: usize, left: usize) {
    let width = rb.width - 1;
    let mut x = 0;
    let y: i32 = idx as i32;

    for ch in line.iter().skip(left) {
        let ch = *ch as char;
        match ch {
            '\t' => {
                let w = 4 - x % 4;
                for _ in 0..w {
                    print_char(rb, ' ', x, y, 7);
                    x += 1;
                }
            }
            '\n' => {}
            _ => {
                print_char(rb, ch, x, y, 7);
                x += 1;
            }
        }
        if x >= width as i32 {
            break;
        }
    }

    // Replace any cells after end of line with ' '
    while x < width as i32 {
        print_char(rb, ' ', x, y, 7);
        x += 1;
    }

    // If the line is too long to fit on the screen, show an indicator
    let indicator = if line.len() > width + left { '→' } else { ' ' };
    print_char(rb, indicator, width as i32, y, 7);
}


pub fn print_char(screen: &mut Screen, data: char, x: i32, y: i32, col: i32) {
    screen.print_char(data, x * 4, 16 + y * 7, col);
}

#[cfg(test)]
mod tests {

    use std::sync::{Arc, Mutex};
    use std::rc::Rc;

    use syntect::highlighting::ThemeSet;

    use view::View;
    use buffer::Buffer;

    fn setup_view(testcase: &'static str) -> View {
        let theme_name = String::from("base16-eighties.dark");
        let theme_set = Rc::new(ThemeSet::load_defaults());
        let buffer = Arc::new(Mutex::new(Buffer::new()));
        let mut view = View::new(buffer.clone(), theme_set, theme_name, 50, 50);
        for ch in testcase.chars() {
            view.insert_char(ch);
        }

        let mut buffer = buffer.lock().unwrap();
        buffer.set_mark(view.cursor, 0);
        view
    }

    #[test]
    fn test_insert_char() {
        let mut view = setup_view("test\nsecond");
        view.insert_char('t');

        {
            let mut buffer = view.buffer.lock().unwrap();
            assert_eq!(buffer.lines().next().unwrap(), b"ttest\n");
        }
    }
}
