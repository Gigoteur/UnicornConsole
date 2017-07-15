pub mod gfx_editor;

use gfx::Screen;
use config::Players;
use std::sync::{Arc, Mutex};
use time;

use px8::PX8Config;

#[derive(Debug)]
pub enum STATE {
    GFX_EDITOR,
    MUSIC_EDITOR,
}

pub struct Editor {
    state: STATE,
    gfx_editor: gfx_editor::GFXEditor,
    filename: String,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            state: STATE::GFX_EDITOR,
            gfx_editor: gfx_editor::GFXEditor::new(),
            filename: "".to_string()
        }
    }

    pub fn init(&mut self, config: Arc<Mutex<PX8Config>>, screen: &mut Screen, filename: String) {
        info!("[EDITOR] Init {:?}", filename);
        self.filename = filename;
        config.lock().unwrap().toggle_mouse(true);

        screen.mode(240, 236, 1.);
        screen.font("pico-8");

        self.gfx_editor.init(config, screen);   
    }

    pub fn update(&mut self, players: Arc<Mutex<Players>>) -> bool {
        true
    }

    pub fn draw(&mut self, players: Arc<Mutex<Players>>, screen: &mut Screen) -> f64 {
        let current_time = time::now();

        screen.cls();

        let width = screen.mode_width() as i32;
        let height = screen.mode_height() as i32;

        screen.rectfill(0, 0, width, 8, 11);
        screen.rectfill(0, height-8, width, height, 11);

        // Print current filename
        screen.print(self.filename.clone(), 0, 0, 7);

        match self.state {
            STATE::GFX_EDITOR => { self.gfx_editor.draw(players, screen); }
            STATE::MUSIC_EDITOR => { }
        }

        let diff_time = time::now() - current_time;
        let nanoseconds = (diff_time.num_nanoseconds().unwrap() as f64) -
            (diff_time.num_seconds() * 1000000000) as f64;

        diff_time.num_seconds() as f64 + nanoseconds / 1000000000.0
    }
}
