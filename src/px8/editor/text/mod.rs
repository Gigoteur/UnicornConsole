pub mod buffers;
pub mod cursor;
pub mod position;
pub mod movement;
pub mod insert;

use gfx::Screen;
use config::Players;

use px8::PX8Config;
use px8::editor::State;

use self::buffers::BufferManager;
use self::cursor::Cursor;
use px8::editor::text::buffers::TextBuffer;
use px8::editor::text::buffers::SplitBuffer;
use px8::editor::text::buffers::Buffer;

use std::sync::{Arc, Mutex};
use std::cmp::{max, min};
use std::collections::HashMap;
use time;


pub struct TextEditor {
    pub buffers: BufferManager,
}

impl TextEditor {
    pub fn new(state: Arc<Mutex<State>>) -> TextEditor {
        TextEditor {
            buffers: BufferManager::new(),
        }
    }

    pub fn init(&mut self, config: Arc<Mutex<PX8Config>>, screen: &mut Screen, filename: String, code: String) {
        info!("[GFX_EDITOR] Init");

        info!("[GFX_EDITOR] {:?}", self.pos());

        let mut new_buffer: Buffer = SplitBuffer::from_str(&code).into();
        new_buffer.title = Some(filename);

        let new_buffer_index = self.buffers.new_buffer(new_buffer);
        self.buffers.switch_to(new_buffer_index);
        self.hint();
    }

    pub fn update(&mut self, players: Arc<Mutex<Players>>) -> bool {
        true
    }

    /// Hint the buffer about the cursor position.
    pub fn hint(&mut self) {
    
        let x = self.cursor().x;
        let y = self.cursor().y;

        self.buffers.current_buffer_mut().focus_hint_y(y);
        self.buffers.current_buffer_mut().focus_hint_x(x);
    }

    pub fn draw(&mut self, players: Arc<Mutex<Players>>, screen: &mut Screen) {
        let (scroll_x, scroll_y) = {
            let current_buffer = self.buffers.current_buffer_info();

            (current_buffer.scroll_x, current_buffer.scroll_y)
        };
        let (pos_x, pos_y) = self.pos();
        let w = screen.width;

        screen.rect(4 * (pos_x - scroll_x) as i32,
                    (8 * (pos_y - scroll_y) as i32) + 8,
                    4 * (pos_x - scroll_x) as i32 + 4,
                    (8 * (pos_y - scroll_y) as i32) + 8 + 8,
                    8);



        for (y, row) in self.buffers
        .current_buffer()
        .lines()
        .enumerate() {
            for (x, c) in row.chars().enumerate() {
                let c = if c == '\t' { ' ' } else { c };

                let pos_char_x = 4 * (x - scroll_x) as i32;
                let pos_char_y = 8 * (y - scroll_y) as i32;
                //info!("OFF C {:?} X {:?} {:?}", c, pos_char_x, pos_char_y);
                screen.print_char(c,
                                  pos_char_x,
                                  pos_char_y + 8,
                                  7);
            }
        }

    }
}
