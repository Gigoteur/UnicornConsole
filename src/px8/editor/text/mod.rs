pub mod buffers;
pub mod cursor;
pub mod position;

use gfx::Screen;
use config::Players;
use self::buffers::BufferManager;
use px8::PX8Config;

use std::sync::{Arc, Mutex};
use std::cmp::{max, min};
use std::collections::HashMap;
use time;


pub struct TextEditor {
    pub buffers: BufferManager,
}

impl TextEditor {
    pub fn new() -> TextEditor {
        TextEditor {
            buffers: BufferManager::new(),
        }
    }

    pub fn init(&mut self, config: Arc<Mutex<PX8Config>>, screen: &mut Screen) {
        info!("[GFX_EDITOR] Init");
    }

    pub fn update(&mut self, players: Arc<Mutex<Players>>) -> bool {
        true
    }

    pub fn draw(&mut self, players: Arc<Mutex<Players>>, screen: &mut Screen) {}
}
