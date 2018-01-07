pub mod gfx_editor;
pub mod music_editor;
pub mod text_editor;

use gfx::Screen;
use config::Players;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use time;

use sound::sound::{SoundInternal, Sound};

use unicorn::{UnicornCartridge, UnicornConfig, Palettes};

#[derive(Clone)]
pub struct Button {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: u32,
    text: String,
    highlight: bool,
    clicked: bool,
}

impl Button {
    pub fn new(x1: i32,
               y1: i32,
               x2: i32,
               y2: i32,
               color: u32,
               text: String,
               highlight: bool)
               -> Button {
        Button {
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
            color: color,
            text: text,
            highlight: highlight,
            clicked: highlight,
        }
    }

    pub fn update(&mut self, x: i32, y: i32) {
        self.clicked = (self.x1 <= x && x <= self.x2) && (self.y1 <= y && y <= self.y2);
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        screen.rectfill(self.x1, self.y1, self.x2, self.y2, self.color as i32);
        let mut color = 1;
        if self.clicked {
            color = 7;
        }
        screen.print(self.text.clone(), self.x1 + 1, self.y1, color);
    }

    pub fn is_click(&mut self) -> bool {
        self.clicked
    }
}

pub struct ButtonSlider {
    text: String,
    value: String,
    global_text: String,
    x: i32,
    y: i32,
    color_text: i32,
    color_background: i32,
    color_clicked: i32,
    minus: Vec<i32>,
    plus: Vec<i32>,
    minus_click: bool,
    plus_click: bool,
}

impl ButtonSlider {
    pub fn new(text: String, value: String, x: i32, y: i32, color_text: i32, color_background: i32, color_clicked: i32) -> ButtonSlider {
        ButtonSlider {
            text: text.clone(),
            value: value.clone(),
            global_text: "".to_string(),
            x: x,
            y: y,
            color_text: color_text,
            color_background: color_background,
            color_clicked: color_clicked,
            minus: vec![0, 0, 0, 0],
            plus: vec![0, 0, 0, 0],
            minus_click: false,
            plus_click: false,
        }
    }

    pub fn update_value(&mut self, value: String) {
        self.value = value.clone();
    }

    pub fn update(&mut self, mouse_state: u32, mouse_x: i32, mouse_y: i32, players: Arc<Mutex<Players>>) {
        self.global_text = format!("{} {}", self.text, self.value);

        let len_text_size = (4*self.global_text.len()) as i32 + 2;
        self.minus[0] = self.x + len_text_size;
        self.minus[1] = self.y;
        self.minus[2] = self.x + len_text_size + 8;
        self.minus[3] = self.y + 8;

        self.plus[0] = self.minus[0] + 10;
        self.plus[1] = self.y;
        self.plus[2] = self.minus[0] + 10 + 8;
        self.plus[3] = self.y + 8;

        if mouse_state == 1 {
            self.minus_click = point_in_rect(mouse_x, mouse_y, self.minus[0], self.minus[1], self.minus[2], self.minus[3]);
            self.plus_click = point_in_rect(mouse_x, mouse_y, self.plus[0], self.plus[1], self.plus[2], self.plus[3]);
        } else {
            self.minus_click = false;
            self.plus_click = false;
        }
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        let len_text_size = (4*self.global_text.len()) as i32;

        screen.rectfill(self.x, self.y, self.x + len_text_size, self.y+8, self.color_background);
        if self.minus_click {
            screen.rectfill(self.minus[0], self.minus[1], self.minus[2], self.minus[3], self.color_clicked);
        } else {
            screen.rectfill(self.minus[0], self.minus[1], self.minus[2], self.minus[3], self.color_background);
        }

        if self.plus_click {
            screen.rectfill(self.plus[0], self.plus[1], self.plus[2], self.plus[3], self.color_clicked);
        } else {
            screen.rectfill(self.plus[0], self.plus[1], self.plus[2], self.plus[3], self.color_background);
        }

        screen.print(self.global_text.clone(), self.x, self.y+2, self.color_text);
        screen.print("-".to_string(), self.minus[0] + 2, self.minus[1] + 2, self.color_text);
        screen.print("+".to_string(), self.plus[0] + 2, self.plus[1] + 2, self.color_text);
    }

    pub fn is_plus_click(&mut self) -> bool {
        self.plus_click
    }

    pub fn is_minus_click(&mut self) -> bool {
        self.minus_click
    }
}

pub fn point_in_rect(x: i32,
                     y: i32,
                     coord_x1: i32,
                     coord_y1: i32,
                     coord_x2: i32,
                     coord_y2: i32)
                     -> bool {
    (coord_x1 <= x && x < coord_x2) && (coord_y1 <= y && y < coord_y2)
}

#[derive(Clone, Copy)]
pub struct State {
    mouse_x: i32,
    mouse_y: i32,
    mouse_state: u32,
    mouse_statep: u32,

    idx_sprites_batch_x: i32,
    idx_sprites_batch_y: i32,
    idx_sprite_info: [i32; 2],
    idx_flag: [i32; 2],
    current_sprite: u32,

    x_zoom_sprite: u32,
    y_zoom_sprite: u32,
    zoom_sprite: u32,
    idx_zoom_sprite: u32,
    sprite_available_zooms: [u32; 4],

    idx_x_zoom_sprite: u32,
    idx_y_zoom_sprite: u32,

    idx_map: u32,

    on_current_sprite_x: u32,
    on_current_sprite_y: u32,
    on_current_sprite: bool,

    fill_action: bool,
}

impl State {
    pub fn new() -> State {
        State {
            mouse_x: 0,
            mouse_y: 0,
            mouse_state: 0,
            mouse_statep: 0,

            idx_sprites_batch_x: 0,
            idx_sprites_batch_y: 170,
            idx_sprite_info: [129, 190],

            idx_flag: [40, 160],
            current_sprite: 0,

            x_zoom_sprite: 0,
            y_zoom_sprite: 0,
            zoom_sprite: 1,
            idx_zoom_sprite: 0,
            sprite_available_zooms: [1, 2, 4, 8],

            idx_x_zoom_sprite: 10,
            idx_y_zoom_sprite: 18,

            idx_map: 0,

            on_current_sprite_x: 0,
            on_current_sprite_y: 0,
            on_current_sprite: false,

            fill_action: false,
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
    long_clicked: bool,
}

impl Widget {
    pub fn new(state: Arc<Mutex<State>>,
               name: String,
               x: u32,
               y: u32,
               w: u32,
               h: u32,
               data: Vec<u8>,
               highlight: HashMap<u32, u32>,
               clicked: bool,
               long_clicked: bool)
               -> Widget {
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
            clicked: clicked,
            long_clicked: long_clicked,
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
            let mouse_x = self.state.lock().unwrap().mouse_x as u32;
            let mouse_y = self.state.lock().unwrap().mouse_y as u32;

            let is_clicked = (self.x1 <= mouse_x && mouse_x < self.x2) &&
                             (self.y1 <= mouse_y && mouse_y < self.y2);

            if self.long_clicked {
                if is_clicked && self.clicked {
                    self.clicked = !self.clicked;
                } else {
                    self.clicked = is_clicked;
                }
            } else {
                self.clicked = is_clicked;
            }
        } else {
            if !self.long_clicked {
                self.clicked = false;
            }
        }
    }

    pub fn is_clickable(&mut self) -> bool {
        let mouse_state = self.state.lock().unwrap().mouse_state;

        if mouse_state == 1 {
            let mouse_x = self.state.lock().unwrap().mouse_x as u32;
            let mouse_y = self.state.lock().unwrap().mouse_y as u32;
            return (self.x1 <= mouse_x && mouse_x < self.x2) &&
                    (self.y1 <= mouse_y && mouse_y < self.y2);
    
        }

        false
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        let mut idx_w = 0;
        let mut idx_h = 0;

        for pixel in &self.data {
            if self.highlight.len() > 0 && self.clicked {
                let pixel = *pixel as u32;
                let pp = self.highlight.get(&pixel).unwrap_or(&pixel);
                screen.pset((self.x1 + idx_w) as i32,
                            (self.y1 + idx_h) as i32,
                            *pp as i32);
            } else {
                screen.pset((self.x1 + idx_w) as i32,
                            (self.y1 + idx_h) as i32,
                            *pixel as i32);
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
    GfxEditor,
    TextEditor,
    MusicEditor,
}

pub struct Editor {
    state: Arc<Mutex<State>>,
    state_editor: STATE,
    gfx: gfx_editor::GFXEditor,
    txt: text_editor::TextEditor,
    music: music_editor::MusicEditor,
    filename: String,
    widgets: Vec<Arc<Mutex<Widget>>>,
}

impl Editor {
    pub fn new(screen: Arc<Mutex<Screen>>) -> Editor {
        let state = Arc::new(Mutex::new(State::new()));
        let mut highlight = HashMap::new();
        highlight.insert(6, 10);

        let mut widgets = Vec::new();
        widgets.push(Arc::new(Mutex::new(Widget::new(state.clone(),
                                                     "GFX".to_string(),
                                                     200,
                                                       1,
                                                     16,
                                                     16,
                                                     vec![ 6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,
                                                           6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                           6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,  6, 11,  6,
                                                           6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,  6,  6, 11,  6,
                                                           6, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11,  6,
                                                           6, 11, 11, 11, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11,  6,
                                                           6, 11, 11, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11,  6,
                                                           6, 11, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11,  6,
                                                           6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                           6, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11, 11,  6,
                                                           6, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11, 11, 11,  6,
                                                           6, 11, 11, 11,  6,  6,  6, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                           6, 11,  6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                           6, 11,  6,  6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                           6, 11,  11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                           6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6],
                                                     highlight.clone(),
                                                     true, true))));
        widgets.push(Arc::new(Mutex::new(Widget::new(state.clone(),
                                                     "TEXT".to_string(),
                                                     220,
                                                     1,
                                                     16,
                                                     16,
                                                     vec![ 6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,
                                                           6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                           6, 11,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6, 11,  6,
                                                           6, 11,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6, 11,  6,
                                                           6, 11,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6, 11,  6,
                                                           6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                           6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                           6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                           6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                           6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                           6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                           6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                           6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                           6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                           6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                           6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6],
                                                     highlight.clone(),
                                                     false, true))));

        widgets.push(Arc::new(Mutex::new(Widget::new(state.clone(),
                                                     "MUSIC".to_string(),
                                                     240,
                                                     1,
                                                     16,
                                                     16,
                                                     vec![ 6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,
                                                           6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                           6, 11, 11, 11, 11, 11, 11, 11, 11,  6,  6,  6,  6,  6, 11,  6,
                                                           6, 11, 11, 11, 11,  6,  6,  6,  6,  6,  6,  6,  6,  6, 11,  6,
                                                           6, 11, 11, 11,  6,  6,  6,  6,  6, 11, 11, 11, 11,  6, 11,  6,
                                                           6, 11, 11, 11,  6,  6, 11, 11, 11, 11, 11, 11, 11,  6, 11,  6,
                                                           6, 11, 11, 11,  6, 11, 11, 11, 11, 11, 11, 11, 11,  6, 11,  6,
                                                           6, 11, 11, 11,  6, 11, 11, 11, 11, 11, 11, 11, 11,  6, 11,  6,
                                                           6, 11, 11, 11,  6, 11, 11, 11, 11, 11, 11, 11, 11,  6, 11,  6,
                                                           6, 11, 11, 11,  6, 11, 11, 11, 11, 11, 11, 11, 11,  6, 11,  6,
                                                           6, 11, 11, 11,  6, 11, 11, 11, 11, 11, 11, 11, 11,  6, 11,  6,
                                                           6, 11,  6,  6,  6, 11, 11, 11, 11, 11, 11,  6,  6,  6, 11,  6,
                                                           6, 11,  6,  6,  6, 11, 11, 11, 11, 11, 11,  6,  6,  6, 11,  6,
                                                           6, 11,  6,  6,  6, 11, 11, 11, 11, 11, 11,  6,  6,  6, 11,  6,
                                                           6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                           6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6],
                                                     highlight.clone(),
                                                     false, true))));

        Editor {
            state: state.clone(),
            state_editor: STATE::GfxEditor,
            gfx: gfx_editor::GFXEditor::new(state.clone()),
            txt: text_editor::TextEditor::new(state.clone()),
            music: music_editor::MusicEditor::new(state.clone()),
            filename: "".to_string(),
            widgets: widgets,
        }
    }

    pub fn init(&mut self, config: Arc<Mutex<UnicornConfig>>, palettes: Arc<Mutex<Palettes>>, screen: &mut Screen, filename: String, code: String) {
        info!("[EDITOR] Init {:?}", filename);
        self.filename = filename.clone();
        config.lock().unwrap().toggle_mouse(true);

        palettes.lock().unwrap().switch_to_palette("pico-8");
        screen.font("pico-8");

        self.gfx.init(config.clone(), screen);
        self.txt.init(config.clone(), screen, filename.clone(), code);
        self.music.init(config.clone(), screen);
    }

    pub fn get_code(&mut self) -> Vec<String> {
        self.txt.get_buffer()
    }

    pub fn update(&mut self, cartridge: &mut UnicornCartridge, screen: &mut Screen, players: Arc<Mutex<Players>>, sound_internal: Arc<Mutex<SoundInternal>>, sound: Arc<Mutex<Sound>>) -> bool {
        self.state.lock().unwrap().update(players.clone());

        let mut is_clickable = false;
        for widget in &self.widgets {
            is_clickable = widget.lock().unwrap().is_clickable();
            if is_clickable {
                break;
            }
        }

        if is_clickable {
            for widget in &self.widgets {
                widget.lock().unwrap().update();
            }
        }

        for widget in &self.widgets {
            let is_click = widget.lock().unwrap().is_click();
            if is_click {
                if widget.lock().unwrap().name == "GFX" {
                    self.state_editor = STATE::GfxEditor;
                } else if widget.lock().unwrap().name == "TEXT" {
                    self.state_editor = STATE::TextEditor;
                } else if widget.lock().unwrap().name == "MUSIC" {
                    self.state_editor = STATE::MusicEditor;
                }
            }
        }

        match self.state_editor {
            STATE::GfxEditor => {
                self.gfx.update(screen, players.clone());
            }
            STATE::TextEditor => {
                self.txt.update(players.clone());
            }
            STATE::MusicEditor => {
                self.music.update(cartridge, screen, players.clone(), sound_internal.clone(), sound.clone());
            }
        }

        true
    }

    pub fn draw(&mut self, players: Arc<Mutex<Players>>, palettes: Arc<Mutex<Palettes>>, screen: &mut Screen) -> f64 {
        let current_time = time::now();

        screen.cls(-1);

        let width = screen.mode_width() as i32;
        let height = screen.mode_height() as i32;

        match self.state_editor {
            STATE::GfxEditor => {
                self.gfx.draw(players, screen);
            }
            STATE::TextEditor => {
                self.txt.draw(players, palettes, screen);
            }
            STATE::MusicEditor => {
                self.music.draw(players, screen);
            }
        }

        screen.rectfill(0, 0, width, 16, 11);
        screen.rectfill(0, height - 8, width, height, 11);

        // Print current filename
        screen.print(self.filename.clone(), 0, 2, 7);

        for widget in &self.widgets {
            widget.lock().unwrap().draw(screen);
        }

        let diff_time = time::now() - current_time;
        let nanoseconds = (diff_time.num_nanoseconds().unwrap() as f64) -
                          (diff_time.num_seconds() * 1000000000) as f64;

        diff_time.num_seconds() as f64 + nanoseconds / 1000000000.0
    }
}
