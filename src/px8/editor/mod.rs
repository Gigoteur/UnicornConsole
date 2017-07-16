pub mod gfx_editor;
pub mod text_editor;

use gfx::Screen;
use config::Players;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use time;

use px8::PX8Config;

pub fn point_in_rect(x: i32, y: i32, coord_x1: i32, coord_y1: i32, coord_x2: i32, coord_y2: i32) -> bool {
    (coord_x1 <= x && x < coord_x2) && (coord_y1 <= y && y < coord_y2)
}

pub struct State {
    mouse_x: i32,
    mouse_y: i32,
    mouse_state: u32,
    mouse_statep: u32,

    idx_sprites_batch: i32,
    idx_sprite_info: [i32; 2],
    idx_flag: [i32; 2],
    idx_sprite_number: [i32; 2],
    current_sprite: u32,

    x_zoom_sprite: u32,
    y_zoom_sprite: u32,
    zoom_sprite: u32,
    idx_zoom_sprite: u32,
    sprite_available_zooms: [u32; 3],

    idx_x_zoom_sprite: u32,
    idx_y_zoom_sprite: u32,

    idx_map: u32,

    on_current_sprite_x: u32,
    on_current_sprite_y: u32,
    on_current_sprite: bool,
}

impl State {
    pub fn new() -> State {
        State {
            mouse_x: 0,
            mouse_y: 0,
            mouse_state: 0,
            mouse_statep: 0,

            idx_sprites_batch: 196,
            idx_sprite_info: [129, 190],
            idx_flag: [140, 200],
            idx_sprite_number: [140, 208],
            current_sprite: 0,

            x_zoom_sprite: 0,
            y_zoom_sprite: 0,
            zoom_sprite: 1,
            idx_zoom_sprite: 0,
            sprite_available_zooms: [1, 2, 4],

            idx_x_zoom_sprite: 10,
            idx_y_zoom_sprite: 10,

            idx_map: 0,

            on_current_sprite_x: 0,
            on_current_sprite_y: 0,
            on_current_sprite: false,
        }
    }

    pub fn update(&mut self, players: Arc<Mutex<Players>>) {
        self.mouse_state = players.lock().unwrap().mouse_state();
        self.mouse_statep = players.lock().unwrap().mouse_state_quick();
        self.mouse_x = players.lock().unwrap().mouse_coordinate(0);
        self.mouse_y = players.lock().unwrap().mouse_coordinate(1);
    }
}

pub struct Widget {
    state: Arc<Mutex<State>>,
    name: String,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
    w: u32,
    h: u32,
    data: Vec<u8>,
    highlight: HashMap<u32, u32>,
    clicked: bool,
}

impl Widget {
    pub fn new(state: Arc<Mutex<State>>, name: String, x: u32, y: u32, w: u32, h: u32, data: Vec<u8>, highlight: HashMap<u32, u32>, clicked: bool) -> Widget {
        Widget {
            state: state,
            name: name,
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
            w: w,
            h: h,
            data: data,
            highlight: highlight,
            clicked: clicked
        }
    }

    pub fn reset(&mut self) {
        self.clicked = false;
    }

    pub fn is_click(&mut self) -> bool {
        self.clicked
    }

    pub fn update(&mut self) {
        let mouse_state = self.state.lock().unwrap().mouse_state;

        if mouse_state == 1 {
            self.clicked = false;

            let mouse_x = self.state.lock().unwrap().mouse_x as u32;
            let mouse_y = self.state.lock().unwrap().mouse_y as u32;

            self.clicked = (self.x1 <= mouse_x && mouse_x < self.x2) && (self.y1 <= mouse_y && mouse_y < self.y2);
        }
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        let mut idx_w = 0;
        let mut idx_h = 0;

        for pixel in &self.data {
            if self.highlight.len() > 0 && self.clicked {
                let pixel = *pixel as u32;
                let pp = self.highlight.get(&pixel).unwrap_or(&pixel);
                screen.pset((self.x1+idx_w) as i32, (self.y1+idx_h) as i32, *pp as i32);
            } else {
                screen.pset((self.x1+idx_w) as i32, (self.y1+idx_h) as i32, *pixel as i32);
            }

            idx_w += 1;
            if idx_w == self.w {
                idx_w = 0;
                idx_h += 1;
            }

        }
    }
}

#[derive(Debug)]
pub enum STATE {
    GFX_EDITOR,
    TEXT_EDITOR,
    MUSIC_EDITOR,
}

pub struct Editor {
    state: STATE,
    gfx_editor: gfx_editor::GFXEditor,
    txt_editor: text_editor::TextEditor,
    filename: String,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            state: STATE::GFX_EDITOR,
            gfx_editor: gfx_editor::GFXEditor::new(),
            txt_editor: text_editor::TextEditor::new(),
            filename: "".to_string()
        }
    }

    pub fn init(&mut self, config: Arc<Mutex<PX8Config>>, screen: &mut Screen, filename: String) {
        info!("[EDITOR] Init {:?}", filename);
        self.filename = filename;
        config.lock().unwrap().toggle_mouse(true);

        screen.mode(240, 236, 1.);
        screen.font("pico-8");

        self.gfx_editor.init(config.clone(), screen);   
        self.txt_editor.init(config.clone(), screen);   
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
            STATE::TEXT_EDITOR => { self.txt_editor.draw(players, screen); }
            STATE::MUSIC_EDITOR => { }
        }

        let diff_time = time::now() - current_time;
        let nanoseconds = (diff_time.num_nanoseconds().unwrap() as f64) -
            (diff_time.num_seconds() * 1000000000) as f64;

        diff_time.num_seconds() as f64 + nanoseconds / 1000000000.0
    }
}
