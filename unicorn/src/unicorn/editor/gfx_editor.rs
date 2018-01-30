use gfx::Screen;
use config::Players;
use std::sync::{Arc, Mutex};
use std::cmp::{max, min};
use std::collections::HashMap;

use unicorn::UnicornConfig;

use unicorn::editor::Widget;
use unicorn::editor::State;
use unicorn::editor::point_in_rect;
use unicorn;

pub struct PalettePicker {
    state: Arc<Mutex<State>>,
    idx_x: i32,
    idx_y: i32,
    current_color: u32,
    current_selection_x: i32,
    current_selection_y: i32,
}

impl PalettePicker {
    pub fn new(state: Arc<Mutex<State>>) -> PalettePicker {
        PalettePicker {
            state: state,
            idx_x: 165,
            idx_y: 20,
            current_color: 0,
            current_selection_x: 0,
            current_selection_y: 0,
        }
    }

    pub fn update(&mut self, screen: &mut Screen) {
        if self.state.lock().unwrap().mouse_statep == 1 {
            let mouse_x = self.state.lock().unwrap().mouse_x;
            let mouse_y = self.state.lock().unwrap().mouse_y;
            let idx_x_zoom_sprite = self.state.lock().unwrap().idx_x_zoom_sprite;
            let idx_y_zoom_sprite = self.state.lock().unwrap().idx_y_zoom_sprite;
            let zoom_sprite = self.state.lock().unwrap().zoom_sprite;

            if point_in_rect(mouse_x,
                             mouse_y,
                             idx_x_zoom_sprite as i32,
                             idx_y_zoom_sprite as i32,
                             (idx_x_zoom_sprite + 128) as i32,
                             (idx_y_zoom_sprite + 128) as i32) {
                let idx_x = ((mouse_x - idx_x_zoom_sprite as i32) as f64 *
                             zoom_sprite as f64 / 16.)
                        .floor() as u32;
                let idx_y = ((mouse_y - idx_y_zoom_sprite as i32) as f64 *
                             zoom_sprite as f64 / 16.)
                        .floor() as u32;

                let x_zoom_sprite = self.state.lock().unwrap().x_zoom_sprite;
                let y_zoom_sprite = self.state.lock().unwrap().y_zoom_sprite;

                let fill_action = self.state.lock().unwrap().fill_action;

                if fill_action {
                    let color_to_replace = screen.sget(x_zoom_sprite + idx_x,  y_zoom_sprite + idx_y);

                    for x in 0..8 * zoom_sprite {
                        for y in 0..8 * zoom_sprite {
                            let current_color = screen.sget(x_zoom_sprite + x,  y_zoom_sprite + y);
                            if color_to_replace == current_color {
                                screen.sset(x_zoom_sprite + x,
                                            y_zoom_sprite + y,
                                            self.current_color as i32);
                            }
                        }
                    }
                } else {
                    screen.sset(x_zoom_sprite + idx_x as u32,
                                y_zoom_sprite + idx_y as u32,
                                self.current_color as i32);
                }
            }

            if point_in_rect(mouse_x,
                             mouse_y,
                             self.idx_x,
                             self.idx_y,
                             self.idx_x + 4 * 16,
                             self.idx_y + 4 * 16) {
                let idx_x = (((mouse_x - self.idx_x) as f64).floor() / 16.) as i32;
                let idx_y = (((mouse_y - self.idx_y) as f64).floor() / 16.) as i32;

                self.current_color = (idx_x + idx_y * 4) as u32;
                self.current_selection_x = idx_x;
                self.current_selection_y = idx_y;
            }
        }
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        let mut idx = 0;
        let x = self.idx_x;
        let mut y = self.idx_y;

        for i in 0..16 {
            let pos_x = x + (16 * (idx % 4));

            let pos_y = y;
            screen.rectfill(pos_x, pos_y, pos_x + 15, pos_y + 15, i);
            idx += 1;

            if idx > 1 && idx % 4 == 0 {
                y += 16;
            }
        }

        let current_selection_x = (self.idx_x + 16 * self.current_selection_x) - 1;
        let current_selection_y = (self.idx_y + 16 * self.current_selection_y) - 1;

        screen.rect(current_selection_x,
                    current_selection_y,
                    current_selection_x + 17,
                    current_selection_y + 17,
                    7);
    }
}

pub struct Flags {
    state: Arc<Mutex<State>>,
    values: [u32; 8],
    flags: HashMap<u32, u32>,
    idx_flag: [i32; 2],
    size: i32,
}

impl Flags {
    pub fn new(state: Arc<Mutex<State>>) -> Flags {
        let values = [0, 1, 2, 3, 4, 5, 6, 7];
        let mut flags = HashMap::new();

        let idx_flag = state.lock().unwrap().idx_flag;

        for i in values.iter() {
            flags.insert(*i, 0);
        }

        Flags {
            state: state.clone(),
            values: values,
            flags: flags,
            idx_flag: idx_flag,
            size: 4,
        }
    }

    pub fn update(&mut self, screen: &mut Screen) {
        let mut idx = 0;
        let idx_sprite = self.state.lock().unwrap().current_sprite;

        for i in self.values.iter() {
            let flag = screen.fget(idx_sprite, *i as u8);
            let mut color = 8;
            if flag {
                color = 11;
            }
            self.flags.insert(*i, color);

            let mouse_state = self.state.lock().unwrap().mouse_state;
            if mouse_state == 1 {
                let mouse_x = self.state.lock().unwrap().mouse_x;
                let mouse_y = self.state.lock().unwrap().mouse_y;

                if point_in_rect(mouse_x,
                                 mouse_y,
                                 self.idx_flag[0] + idx,
                                 self.idx_flag[1],
                                 self.idx_flag[0] + self.size + idx,
                                 self.idx_flag[1] + self.size) {
                    screen.fset(idx_sprite, *i as u8, !flag);
                }
            }

            idx += 8;
        }
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        let mut idx = 0;

        for k in self.values.iter() {
            let color = self.flags[k];

            screen.rectfill(self.idx_flag[0] + idx,
                            self.idx_flag[1],
                            self.idx_flag[0] + self.size + idx,
                            self.idx_flag[1] + self.size,
                            color as i32);

            idx += 8
        }
    }
}

#[derive(Debug)]
pub enum EditorState {
    SpriteEditor,
    MapEditor,
}

pub struct MapEditor {
    state: Arc<Mutex<State>>,
    coord: [i32; 4],
    offset_x: i32,
    offset_y: i32,
    available_zooms: [f32; 3],
    idx_zoom: u32,
    zoom: f32,
    cache: [u32; unicorn::MAP_WIDTH * unicorn::MAP_HEIGHT],
    select_field: [i32; 2],
    size_sprite: i32,
    current_sprite: [u32; 2],
    sprites_per_x: f32,
    sprites_per_y: f32,
}

impl MapEditor {
    pub fn new(state: Arc<Mutex<State>>) -> MapEditor {
        MapEditor {
            state: state.clone(),
            coord: [0, 8, 200, 182],
            offset_x: 0,
            offset_y: 0,
            available_zooms: [1., 0.5, 0.25],
            idx_zoom: 0,
            zoom: 1.,
            cache: [0; unicorn::MAP_WIDTH * unicorn::MAP_HEIGHT],
            select_field: [0, 8],
            size_sprite: 8,
            current_sprite: [0, 0],
            sprites_per_x: 25.,
            sprites_per_y: 22.,
        }
    }

    pub fn init(&mut self, screen: &mut Screen) {
        info!("[EDITOR][GFX][MAP] Init");

        for y in 0..unicorn::MAP_HEIGHT {
            for x in 0..unicorn::MAP_WIDTH {
                self.cache[x + y * unicorn::MAP_WIDTH] = screen.mget(x as i32, y as i32);
            }
        }
    }

    pub fn update(&mut self, players: Arc<Mutex<Players>>, screen: &mut Screen) {
        if players.lock().unwrap().btnp(0, 0) {
            self.offset_x -= 8;
            self.offset_x = max(0, self.offset_x);
        }

        if players.lock().unwrap().btnp(0, 1) {
            self.offset_x += 8;
            self.offset_x = min(((unicorn::MAP_WIDTH as f32 / self.zoom - self.sprites_per_x * self.zoom) *
                                 self.zoom)
                                        .floor() as i32,
                                self.offset_x);
        }

        if players.lock().unwrap().btnp(0, 2) {
            self.offset_y -= 8;
            self.offset_y = max(0, self.offset_y);
        }

        if players.lock().unwrap().btnp(0, 3) {
            self.offset_y += 8;
            self.offset_y = min(((unicorn::MAP_HEIGHT as f32 / self.zoom - self.sprites_per_y * self.zoom) *
                                 self.zoom)
                                        .floor() as i32,
                                self.offset_y);
        }


        if players.lock().unwrap().btnp(0, 4) {
            self.idx_zoom = (self.idx_zoom + 1) % self.available_zooms.len() as u32;
            self.zoom = self.available_zooms[self.idx_zoom as usize];
            self.size_sprite = (8. * self.zoom).floor() as i32;
        }

        let mouse_x = self.state.lock().unwrap().mouse_x;
        let mouse_y = self.state.lock().unwrap().mouse_y;

        if point_in_rect(mouse_x,
                         mouse_y,
                         self.coord[0],
                         self.coord[1],
                         self.coord[2],
                         self.coord[3]) {
            let mouse_statep = self.state.lock().unwrap().mouse_statep;

            let select_field_x = min(192, mouse_x - mouse_x % self.size_sprite);
            let select_field_y = min(176, mouse_y - mouse_y % self.size_sprite);

            let new_x = ((select_field_x + self.offset_x * self.size_sprite) as f64 /
                         self.size_sprite as f64)
                    .floor() as u32;
            let new_y = ((select_field_y - self.coord[1] + self.offset_y * self.size_sprite) as
                         f64 /
                         self.size_sprite as f64)
                    .floor() as u32;
            if new_x < unicorn::MAP_WIDTH as u32 && new_y < unicorn::MAP_HEIGHT as u32 {
                self.select_field[0] = select_field_x;
                self.select_field[1] = select_field_y;

                if mouse_statep == 1 {
                    let zoom_sprite = self.state.lock().unwrap().zoom_sprite;

                    for x in 0u32..zoom_sprite as u32 {
                        for y in 0u32..zoom_sprite as u32 {
                            let current_sprite = self.state.lock().unwrap().current_sprite + x +
                                                 y * 16;

                            let idx = ((new_x + x) as f64 + (new_y + y) as f64 * unicorn::MAP_WIDTH as f64).floor() as
                                      usize;
                            self.cache[idx] = current_sprite;
                            screen.mset((new_x + x) as i32, (new_y + y) as i32, current_sprite);
                        }
                    }
                }
                self.current_sprite[0] = new_x;
                self.current_sprite[1] = new_y;
            }
        }
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        // clean screen
        screen.rectfill(self.coord[0],
                        self.coord[1],
                        self.coord[2],
                        self.coord[3],
                        0);
        screen.rectfill(self.coord[2], self.coord[1], 240, self.coord[3], 5);

        // draw map
        let mut idx_y = 0;
        for y in self.offset_y as u32..
                 min(unicorn::MAP_HEIGHT as u32,
                     self.offset_y as u32 + (self.sprites_per_y / self.zoom).floor() as u32) {
            let mut idx_x = 0;

            for x in self.offset_x as u32..
                     min(unicorn::MAP_WIDTH as u32,
                         self.offset_x as u32 + (self.sprites_per_x / self.zoom).floor() as u32) {
                let offset = x + y * unicorn::MAP_WIDTH as u32;

                let sprite_number = self.cache[offset as usize];
                if sprite_number != 0 {
                    let sprite_x = (sprite_number % 16) * 8;
                    let sprite_y = (sprite_number as f32 / 16.).floor() as i32 * 8;

                    let dx = idx_x * ((8. * self.zoom).floor() as i32);
                    let dy = idx_y * ((8. * self.zoom).floor() as i32) + 9;
                    screen.sspr(sprite_x as u32,
                                sprite_y as u32,
                                8,
                                8,
                                dx,
                                dy,
                                (self.zoom * 8.).floor() as u32,
                                (self.zoom * 8.).floor() as u32,
                                false,
                                false);
                }

                idx_x += 1
            }

            idx_y += 1
        }


        // draw selected sprites
        let zoom_sprite = self.state.lock().unwrap().zoom_sprite;

        screen.rect(self.select_field[0],
                    self.select_field[1],
                    self.select_field[0] + (8. * self.zoom * zoom_sprite as f32).floor() as i32,
                    self.select_field[1] + (8. * self.zoom * zoom_sprite as f32).floor() as i32,
                    7);

        // Draw info
        screen.print(format!("{:?} {:?}: {:?}",
                             self.current_sprite[0],
                             self.current_sprite[1],
                             self.cache[(self.current_sprite[0] + self.current_sprite[1] * 128) as
                             usize]),
                     210,
                     25,
                     7);
    }
}

pub struct SpriteEditor {
    state: Arc<Mutex<State>>,
    pp: PalettePicker,
    widgets: Vec<Arc<Mutex<Widget>>>,
    buffer_copy: Vec<u32>,
    buffer_copy_size: [u32; 2],
}

impl SpriteEditor {
    pub fn new(state: Arc<Mutex<State>>) -> SpriteEditor {

        let mut widgets = Vec::new();
        let mut highlight = HashMap::new();
        highlight.insert(6, 10);

        widgets.push(Arc::new(Mutex::new(Widget::new(state.clone(),
                                                     "ERASE".to_string(),
                                                     160,
                                                     90,
                                                     8,
                                                     8,
                                                     vec![6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 5, 5,
                                                          5, 5, 6, 6, 6, 5, 6, 5, 5, 6, 5, 6,
                                                          6, 5, 5, 6, 6, 5, 5, 6, 6, 5, 5, 6,
                                                          6, 5, 5, 6, 6, 5, 6, 5, 5, 6, 5, 6,
                                                          6, 6, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6,
                                                          6, 6, 6, 6],
                                                     HashMap::new(),
                                                     false, false))));
        widgets.push(Arc::new(Mutex::new(Widget::new(state.clone(),
                                                     "COPY".to_string(),
                                                     170,
                                                     90,
                                                     8,
                                                     8,
                                                     vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6,
                                                          6, 6, 6, 6, 5, 6, 6, 6, 5, 5, 5, 6,
                                                          5, 6, 5, 6, 5, 5, 5, 6, 5, 6, 5, 6,
                                                          5, 5, 5, 6, 5, 6, 5, 6, 5, 5, 5, 6,
                                                          5, 6, 6, 6, 6, 6, 6, 6, 5, 5, 5, 5,
                                                          5, 5, 5, 5],
                                                     HashMap::new(),
                                                     false, false))));
        widgets.push(Arc::new(Mutex::new(Widget::new(state.clone(),
                                                     "PASTE".to_string(),
                                                     180,
                                                     90,
                                                     8,
                                                     8,
                                                     vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6,
                                                          6, 6, 6, 5, 5, 6, 5, 5, 5, 5, 6, 5,
                                                          5, 6, 5, 5, 5, 5, 6, 5, 5, 6, 5, 5,
                                                          5, 5, 6, 5, 5, 6, 5, 5, 5, 5, 6, 5,
                                                          5, 6, 6, 6, 6, 6, 6, 5, 5, 5, 5, 5,
                                                          5, 5, 5, 5],
                                                     HashMap::new(),
                                                     false, false))));
        widgets.push(Arc::new(Mutex::new(Widget::new(state.clone(),
                                                     "ROTATE LEFT".to_string(),
                                                     190,
                                                     90,
                                                     8,
                                                     8,
                                                     vec![5, 5, 6, 5, 5, 5, 5, 5, 5, 6, 5, 5,
                                                          5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 5,
                                                          5, 6, 5, 5, 5, 5, 6, 5, 5, 5, 6, 5,
                                                          5, 5, 6, 5, 5, 5, 5, 5, 5, 6, 6, 6,
                                                          5, 5, 5, 5, 5, 6, 6, 6, 5, 5, 5, 5,
                                                          5, 5, 5, 5],
                                                     HashMap::new(),
                                                     false, false))));
        widgets.push(Arc::new(Mutex::new(Widget::new(state.clone(),
                                                     "ROTATE RIGHT".to_string(),
                                                     200,
                                                     90,
                                                     8,
                                                     8,
                                                     vec![5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5,
                                                          5, 5, 6, 5, 5, 6, 6, 6, 6, 6, 6, 6,
                                                          5, 6, 5, 5, 5, 5, 6, 5, 5, 6, 5, 5,
                                                          5, 6, 5, 5, 6, 6, 6, 5, 5, 5, 5, 5,
                                                          6, 6, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5,
                                                          5, 5, 5, 5],
                                                     HashMap::new(),
                                                     false, false))));
        widgets.push(Arc::new(Mutex::new(Widget::new(state.clone(),
                                                     "FILL".to_string(),
                                                     210, 90, 8, 8,
                                                     vec![5, 5, 5, 5, 5, 5, 5, 5,
                                                          5, 5, 6, 6, 6, 6, 6, 5,
                                                          5, 5, 6, 6, 6, 6, 6, 5,
                                                          5, 5, 6, 6, 6, 6, 6, 5,
                                                          5, 5, 6, 6, 6, 6, 6, 5,
                                                          5, 6, 5, 5, 5, 5, 5, 5,
                                                          5, 6, 6, 5, 5, 5, 5, 5,
                                                          5, 6, 6, 5, 5, 5, 5, 5],
                                                     highlight.clone(),
                                                     false, true))));

        SpriteEditor {
            state: state.clone(),
            pp: PalettePicker::new(state.clone()),
            widgets: widgets,
            buffer_copy: Vec::new(),
            buffer_copy_size: [0, 0],
        }
    }

    pub fn copy(&mut self, screen: &mut Screen) {
        info!("[Unicorn][EDITOR] Copy");

        let zoom_sprite = self.state.lock().unwrap().zoom_sprite;
        let x_zoom_sprite = self.state.lock().unwrap().x_zoom_sprite;
        let y_zoom_sprite = self.state.lock().unwrap().y_zoom_sprite;

        self.buffer_copy.clear();

        self.buffer_copy_size[0] = 8 * zoom_sprite;
        self.buffer_copy_size[1] = 8 * zoom_sprite;

        for _ in 0..8 * zoom_sprite {
            for _ in 0..8 * zoom_sprite {
                self.buffer_copy.push(0);
            }
        }

        for x in 0..8 * zoom_sprite {
            for y in 0..8 * zoom_sprite {
                self.buffer_copy[(x + y * 8 * zoom_sprite) as usize] =
                    screen.sget(x_zoom_sprite + x, y_zoom_sprite + y);
            }
        }
    }

    pub fn paste(&mut self, screen: &mut Screen) {
        if self.buffer_copy.len() > 0 {
            info!("[Unicorn][EDITOR] Paste");

            let zoom_sprite = self.state.lock().unwrap().zoom_sprite;
            let x_zoom_sprite = self.state.lock().unwrap().x_zoom_sprite;
            let y_zoom_sprite = self.state.lock().unwrap().y_zoom_sprite;

            for x in 0..8 * zoom_sprite {
                for y in 0..8 * zoom_sprite {
                    screen.sset(x_zoom_sprite + x,
                                y_zoom_sprite + y,
                                self.buffer_copy[(x + y * 8 * zoom_sprite) as usize] as
                                i32);
                }
            }
        }
    }

    pub fn erase(&mut self, screen: &mut Screen) {
        info!("[Unicorn][EDITOR] Erase");
        
        let zoom_sprite = self.state.lock().unwrap().zoom_sprite;
        let x_zoom_sprite = self.state.lock().unwrap().x_zoom_sprite;
        let y_zoom_sprite = self.state.lock().unwrap().y_zoom_sprite;

        for x in 0..8 * zoom_sprite {
            for y in 0..8 * zoom_sprite {
                screen.sset(x_zoom_sprite + x, y_zoom_sprite + y, 0);
            }
        }
    }
    pub fn cut(&mut self, screen: &mut Screen) {
        info!("[Unicorn][EDITOR] Cut");

        self.copy(screen);
        self.erase(screen);
    }

    pub fn update(&mut self, players: Arc<Mutex<Players>>, screen: &mut Screen) {
        self.pp.update(screen);

        if players.lock().unwrap().btnp2(1073741948) {
            self.copy(screen);
        }

        if players.lock().unwrap().btnp2(1073741949) {
            self.paste(screen);
        }

        if players.lock().unwrap().btnp2(1073741947) {
            self.cut(screen);
        }

        // Update widgets
        for widget in &self.widgets {
            widget.lock().unwrap().update();
        }

        let zoom_sprite = self.state.lock().unwrap().zoom_sprite;
        let x_zoom_sprite = self.state.lock().unwrap().x_zoom_sprite;
        let y_zoom_sprite = self.state.lock().unwrap().y_zoom_sprite;

        self.state.lock().unwrap().fill_action = false;
        for widget in &self.widgets.clone() {
            let is_click = widget.lock().unwrap().is_click();

            if is_click {
                let name = widget.lock().unwrap().name.clone();
                if name == "FILL" {
                    self.state.lock().unwrap().fill_action = true;
                }

                if name == "ERASE" {
                    self.erase(screen);
                }

                if name == "COPY" {
                    self.copy(screen);
                }

                if name == "PASTE" {
                    self.paste(screen);
                }
                // Rotate right
                if name == "ROTATE RIGHT" {
                    info!("[Unicorn][EDITOR] Rotate Right");

                    let mut buffer_copy = Vec::new();

                    for _ in 0..8 * zoom_sprite {
                        for _ in 0..8 * zoom_sprite {
                            buffer_copy.push(0);
                        }
                    }

                    for x in 0..8 * zoom_sprite {
                        for y in 0..8 * zoom_sprite {
                            buffer_copy[(y + x * 8 * zoom_sprite) as usize] =
                                screen.sget(x_zoom_sprite + x, y_zoom_sprite + y);
                        }
                    }

                    let m = 8 * zoom_sprite;
                    for i in 0..(8 * zoom_sprite) / 2 {
                        for j in 0..8 * zoom_sprite {
                            let tmp = buffer_copy[((m - (i + 1)) + j * 8 * zoom_sprite) as usize];
                            buffer_copy[((m - (i + 1)) + j * 8 * zoom_sprite) as usize] =
                                buffer_copy[(i + j * 8 * zoom_sprite) as usize];
                            buffer_copy[(i + j * 8 * zoom_sprite) as usize] = tmp;
                        }
                    }

                    for x in 0..8 * zoom_sprite {
                        for y in 0..8 * zoom_sprite {
                            screen.sset(x_zoom_sprite + x,
                                        y_zoom_sprite + y,
                                        buffer_copy[(x + y * 8 * zoom_sprite) as usize] as i32);
                        }
                    }
                }
                // Rotate left
                if name == "ROTATE LEFT" {
                    info!("[Unicorn][EDITOR] Rotate Left");

                    let mut buffer_copy = Vec::new();

                    for _ in 0..8 * zoom_sprite {
                        for _ in 0..8 * zoom_sprite {
                            buffer_copy.push(0);
                        }
                    }

                    for x in 0..8 * zoom_sprite {
                        for y in 0..8 * zoom_sprite {
                            buffer_copy[(y + x * 8 * zoom_sprite) as usize] =
                                screen.sget(x_zoom_sprite + x, y_zoom_sprite + y);
                        }
                    }

                    let m = 8 * zoom_sprite;
                    for i in 0..(8 * zoom_sprite) / 2 {
                        for j in 0..8 * zoom_sprite {
                            let right = (j + (m - (i + 1)) * 8 * zoom_sprite) as usize;
                            let left = (j + i * 8 * zoom_sprite) as usize;
                            let tmp = buffer_copy[right];
                            buffer_copy[right] = buffer_copy[left];
                            buffer_copy[left] = tmp;
                        }
                    }

                    for x in 0..8 * zoom_sprite {
                        for y in 0..8 * zoom_sprite {
                            screen.sset(x_zoom_sprite + x,
                                        y_zoom_sprite + y,
                                        buffer_copy[(x + y * 8 * zoom_sprite) as usize] as i32);
                        }
                    }
                }
            }
        }

        if players.lock().unwrap().btnp(0, 4) {
            let idx_zoom_sprite = self.state.lock().unwrap().idx_zoom_sprite;
            let sprite_available_zooms = self.state.lock().unwrap().sprite_available_zooms;

            let new_idx_zoom_sprite = (idx_zoom_sprite + 1) % sprite_available_zooms.len() as u32;
            self.state.lock().unwrap().idx_zoom_sprite = new_idx_zoom_sprite;

            self.state.lock().unwrap().zoom_sprite = sprite_available_zooms[new_idx_zoom_sprite as usize];
        }

        // shift sprite
        // shift left
        if players.lock().unwrap().btnp(0, 0) {
            let mut buffer_copy = Vec::new();

            for _ in 0..8 * zoom_sprite {
                for _ in 0..8 * zoom_sprite {
                    buffer_copy.push(0);
                }
            }

            for x in 0..8 * zoom_sprite {
                for y in 0..8 * zoom_sprite {
                    buffer_copy[(y + x * 8 * zoom_sprite) as usize] =
                        screen.sget(x_zoom_sprite + x, y_zoom_sprite + y);
                }
            }

            let (a, b) = buffer_copy.split_at(8 * zoom_sprite as usize);
            let mut spun_vector: Vec<u32> = vec![];
            spun_vector.extend_from_slice(b);
            spun_vector.extend_from_slice(a);

            for x in 0..8 * zoom_sprite {
                for y in 0..8 * zoom_sprite {
                    screen.sset(x_zoom_sprite + x,
                                y_zoom_sprite + y,
                                spun_vector[(y + x * 8 * zoom_sprite) as usize] as i32);
                }
            }
        }
        // shift right
        if players.lock().unwrap().btnp(0, 1) {
            let mut buffer_copy = Vec::new();

            for _ in 0..8 * zoom_sprite {
                for _ in 0..8 * zoom_sprite {
                    buffer_copy.push(0);
                }
            }

            for x in 0..8 * zoom_sprite {
                for y in 0..8 * zoom_sprite {
                    buffer_copy[(y + x * 8 * zoom_sprite) as usize] =
                        screen.sget(x_zoom_sprite + x, y_zoom_sprite + y);
                }
            }

            let max_size = (8 * zoom_sprite * 8 * zoom_sprite) - (8 * zoom_sprite);
            let (a, b) = buffer_copy.split_at(max_size as usize);
            let mut spun_vector: Vec<u32> = vec![];
            spun_vector.extend_from_slice(b);
            spun_vector.extend_from_slice(a);

            for x in 0..8 * zoom_sprite {
                for y in 0..8 * zoom_sprite {
                    screen.sset(x_zoom_sprite + x,
                                y_zoom_sprite + y,
                                spun_vector[(y + x * 8 * zoom_sprite) as usize] as i32);
                }
            }
        }

        // shift down
        if players.lock().unwrap().btnp(0, 2) {
            let mut buffer_copy = Vec::new();

            for _ in 0..8 * zoom_sprite {
                for _ in 0..8 * zoom_sprite {
                    buffer_copy.push(0);
                }
            }

            for x in 0..8 * zoom_sprite {
                for y in 0..8 * zoom_sprite {
                    buffer_copy[(x + y * 8 * zoom_sprite) as usize] =
                        screen.sget(x_zoom_sprite + x, y_zoom_sprite + y);
                }
            }

            let (a, b) = buffer_copy.split_at(8 * zoom_sprite as usize);
            let mut spun_vector: Vec<u32> = vec![];
            spun_vector.extend_from_slice(b);
            spun_vector.extend_from_slice(a);

            for x in 0..8 * zoom_sprite {
                for y in 0..8 * zoom_sprite {
                    screen.sset(x_zoom_sprite + x,
                                y_zoom_sprite + y,
                                spun_vector[(x + y * 8 * zoom_sprite) as usize] as i32);
                }
            }
        }

        // shift up
        if players.lock().unwrap().btnp(0, 3) {
            let mut buffer_copy = Vec::new();

            for _ in 0..8 * zoom_sprite {
                for _ in 0..8 * zoom_sprite {
                    buffer_copy.push(0);
                }
            }

            for x in 0..8 * zoom_sprite {
                for y in 0..8 * zoom_sprite {
                    buffer_copy[(x + y * 8 * zoom_sprite) as usize] =
                        screen.sget(x_zoom_sprite + x, y_zoom_sprite + y);
                }
            }

            let max_size = (8 * zoom_sprite * 8 * zoom_sprite) - (8 * zoom_sprite);
            let (a, b) = buffer_copy.split_at(max_size as usize);
            let mut spun_vector: Vec<u32> = vec![];
            spun_vector.extend_from_slice(b);
            spun_vector.extend_from_slice(a);

            for x in 0..8 * zoom_sprite {
                for y in 0..8 * zoom_sprite {
                    screen.sset(x_zoom_sprite + x,
                                y_zoom_sprite + y,
                                spun_vector[(x + y * 8 * zoom_sprite) as usize] as i32);
                }
            }
        }
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        self.pp.draw(screen);

        let x_zoom_sprite = self.state.lock().unwrap().x_zoom_sprite;
        let y_zoom_sprite = self.state.lock().unwrap().y_zoom_sprite;
        let zoom_sprite = self.state.lock().unwrap().zoom_sprite;
        let idx_x_zoom_sprite = self.state.lock().unwrap().idx_x_zoom_sprite;
        let idx_y_zoom_sprite = self.state.lock().unwrap().idx_y_zoom_sprite;


        // draw black rect
        screen.rectfill(8, 16,
                        138, 146,
                        0);
        
        // draw the sprite
        screen.sspr(x_zoom_sprite,
                    y_zoom_sprite,
                    8 * zoom_sprite as u32,
                    8 * zoom_sprite as u32,
                    idx_x_zoom_sprite as i32,
                    idx_y_zoom_sprite as i32,
                    128,
                    128,
                    false,
                    false);

        for widget in &self.widgets {
            widget.lock().unwrap().draw(screen);
        }
    }
}

pub struct SpritesMap {
    state: Arc<Mutex<State>>,
    flags: Flags,
}

impl SpritesMap {
    pub fn new(state: Arc<Mutex<State>>) -> SpritesMap {
      //  y = state.lock().unwrap().idx_sprites_batch;

        SpritesMap {
            state: state.clone(),
            flags: Flags::new(state.clone()),
        }
    }

    pub fn update(&mut self, screen: &mut Screen) {
        self.state.lock().unwrap().on_current_sprite = false;

        if self.state.lock().unwrap().mouse_state == 1 {
            let mouse_x = self.state.lock().unwrap().mouse_x;
            let mouse_y = self.state.lock().unwrap().mouse_y;

            let idx_sprites_batch_x = self.state.lock().unwrap().idx_sprites_batch_x;
            let idx_sprites_batch_y = self.state.lock().unwrap().idx_sprites_batch_y;

            if ((mouse_y >= idx_sprites_batch_y) && (mouse_y < (idx_sprites_batch_y + 60))) &&
               ((mouse_x >= idx_sprites_batch_x) && (mouse_x <= (idx_sprites_batch_x + 400))) {

                let y = ((mouse_y - idx_sprites_batch_y) as f64 / 8.).floor() as u32;
                let x = ((mouse_x - idx_sprites_batch_x)as f64 / 8.).floor() as u32;

                self.state.lock().unwrap().current_sprite = x + y * 50;

                let current_sprite = self.state.lock().unwrap().current_sprite;
                self.state.lock().unwrap().x_zoom_sprite = (current_sprite % 50) * 8;
                self.state.lock().unwrap().y_zoom_sprite = (current_sprite as f64 / 50.).floor() as u32 * 8;
            }
        }

        // Update flags
        self.flags.update(screen);
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        self.draw_sprite_map(screen);
        self.draw_sprite_flags(screen);
        self.draw_information(screen);
    }

    pub fn draw_sprite_flags(&mut self, screen: &mut Screen) {
        self.flags.draw(screen);
    }

    pub fn draw_sprite_map(&mut self, screen: &mut Screen) {
        let zoom = self.state.lock().unwrap().zoom_sprite as i32;
        let mut current_sprite_x = 0;
        let mut current_sprite_y = 0;

        let mut idx = self.state.lock().unwrap().idx_map * (4 * 16);

        let idx_sprites_batch_x = self.state.lock().unwrap().idx_sprites_batch_x as i32;
        let idx_sprites_batch_y = self.state.lock().unwrap().idx_sprites_batch_y as i32;

        let mut y = idx_sprites_batch_y;

        // Draw black screen
        screen.rectfill(idx_sprites_batch_x, idx_sprites_batch_y,
                        idx_sprites_batch_x + 400, idx_sprites_batch_y + 60,
                        0);
        
        for _ in 0..2 {
            for _ in 0..4 {
                let mut x = idx_sprites_batch_x;
                for _ in 0..50 {
                    screen.spr(idx, x, y, 1, 1, false, false);
                    if idx == self.state.lock().unwrap().current_sprite {
                        current_sprite_x = x;
                        current_sprite_y = y;
                    }
                    x += 8;
                    idx += 1;
                }
                y += 8;
            }
        }

        current_sprite_x -= 1;
        screen.rect(current_sprite_x,
                    current_sprite_y,
                    current_sprite_x + 8 * zoom,
                    current_sprite_y + 8 * zoom,
                    7);
        screen.rect(current_sprite_x - 1,
                    current_sprite_y - 1,
                    current_sprite_x + 1 + 8 * zoom,
                    current_sprite_y + 1 + 8 * zoom,
                    0);
    }

    pub fn draw_information(&mut self, screen: &mut Screen) {
        if self.state.lock().unwrap().on_current_sprite {
            let on_current_sprite_x = self.state.lock().unwrap().on_current_sprite_x;
            let on_current_sprite_y = self.state.lock().unwrap().on_current_sprite_y;

            screen.print(format!("{:?},{:?}", on_current_sprite_x, on_current_sprite_y),
                         300,
                         0,
                         7);
        }

        screen.print(format!("{:?}", self.state.lock().unwrap().current_sprite),
                     320,
                     20,
                     7);
    }
}

pub struct GFXEditor {
    state: Arc<Mutex<State>>,
    state_editor: EditorState,
    sm: SpritesMap,
    me: MapEditor,
    se: SpriteEditor,
    widgets: Vec<Arc<Mutex<Widget>>>,
}

impl GFXEditor {
    pub fn new(state: Arc<Mutex<State>>) -> GFXEditor {
        let mut widgets = Vec::new();
        let mut highlight = HashMap::new();
        highlight.insert(6, 10);

        widgets.push(Arc::new(Mutex::new(Widget::new(state.clone(),
                                                     "SPRITES".to_string(),
                                                     350,
                                                     20,
                                                     16,
                                                     16,
                                                     vec![ 6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,
                                                           6,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  6,
                                                           6,  5,  6,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  6,
                                                           6,  5,  5,  6,  6,  6,  6,  6,  6,  5,  5,  5,  5,  5,  5,  6,
                                                           6,  5,  5,  6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  5,  5,  6,
                                                           6,  5,  6,  5,  5,  5,  5,  5,  6,  5,  5,  5,  5,  5,  5,  6,
                                                           6,  5,  6,  6,  6,  6,  5,  5,  6,  6,  6,  6,  6,  5,  5,  6,
                                                           6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  5,  5,  6,  5,  5,  6,
                                                           6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  5,  5,  6,  5,  5,  6,
                                                           6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  5,  5,  6,  6,  5,  6,
                                                           6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  5,  5,  6,  5,  6,  6,
                                                           6,  5,  5,  5,  5,  6,  5,  6,  6,  6,  6,  5,  6,  5,  6,  6,
                                                           6,  5,  5,  5,  5,  6,  5,  6,  5,  5,  6,  5,  6,  5,  6,  6,
                                                           6,  5,  5,  5,  5,  6,  6,  6,  5,  5,  6,  6,  6,  5,  5,  6,
                                                           6,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  6,
                                                           6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6],
                                                     highlight.clone(),
                                                     true, true))));
        widgets.push(Arc::new(Mutex::new(Widget::new(state.clone(),
                                                     "MAP".to_string(),
                                                     370,
                                                     20,
                                                     16,
                                                     16,
                                                     vec![ 6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,
                                                           6,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  6,
                                                           6,  5,  6,  6,  6,  5,  5,  5,  5,  5,  5,  6,  6,  6,  5,  6,
                                                           6,  5,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  5,  6,
                                                           6,  5,  6,  6,  6,  5,  5,  5,  5,  5,  5,  6,  6,  6,  5,  6,
                                                           6,  5,  5,  6,  5,  5,  5,  5,  5,  5,  5,  5,  6,  5,  5,  6,
                                                           6,  5,  5,  6,  5,  5,  5,  5,  5,  5,  5,  5,  6,  5,  5,  6,
                                                           6,  5,  5,  6,  5,  5,  5,  5,  5,  5,  5,  5,  6,  5,  5,  6,
                                                           6,  5,  5,  6,  5,  5,  5,  5,  5,  5,  5,  5,  6,  5,  5,  6,
                                                           6,  5,  5,  6,  5,  5,  5,  5,  5,  5,  5,  5,  6,  5,  5,  6,
                                                           6,  5,  6,  6,  6,  5,  5,  5,  5,  5,  5,  5,  6,  5,  5,  6,
                                                           6,  5,  6,  6,  6,  5,  5,  5,  5,  5,  5,  6,  6,  6,  5,  6,
                                                           6,  5,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  5,  6,
                                                           6,  5,  6,  6,  6,  5,  5,  5,  5,  5,  5,  6,  6,  6,  5,  6,
                                                           6,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  6,
                                                           6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6],
                                                     highlight.clone(),
                                                     false, true))));

        GFXEditor {
            state: state.clone(),
            state_editor: EditorState::SpriteEditor,
            sm: SpritesMap::new(state.clone()),
            me: MapEditor::new(state.clone()),
            se: SpriteEditor::new(state.clone()),
            widgets: widgets,
        }
    }

    pub fn init(&mut self, _config: Arc<Mutex<UnicornConfig>>, _screen: &mut Screen) {
        info!("[EDITOR][GFX] Init");
    }

    pub fn update(&mut self, screen: &mut Screen, _players: Arc<Mutex<Players>>) -> bool {
        let mut is_clickable = false;
        for widget in &self.widgets {
            is_clickable = widget.lock().unwrap().is_clickable();
            if is_clickable {
                break;
            }
        }

        if is_clickable {
            for widget in &self.widgets {
                widget.lock().unwrap().reset();
                widget.lock().unwrap().update();

                let is_click = widget.lock().unwrap().is_click();
                if is_click {
                    if widget.lock().unwrap().name == "SPRITES" {
                        self.state_editor = EditorState::SpriteEditor;
                    }
                    if widget.lock().unwrap().name == "MAP" {
                        self.state_editor = EditorState::MapEditor;
                        self.me.init(screen);
                    }
                }
            }
        }

        true
    }

    pub fn draw(&mut self, players: Arc<Mutex<Players>>, screen: &mut Screen) {
        self.sm.update(screen);
        match self.state_editor {
            EditorState::SpriteEditor => {
                self.se.update(players.clone(), screen);
            }
            EditorState::MapEditor => {
                self.me.update(players.clone(), screen);
            }
        }

        let width = screen.mode_width() as i32;
        let height = screen.mode_height() as i32;

        // Fill the screen
        screen.rectfill(0, 0, width, height, 5);

        // Draw sprites map
        self.sm.draw(screen);

        // Draw sprite or map editor
        match self.state_editor {
            EditorState::SpriteEditor => {
                self.se.draw(screen);
            }
            EditorState::MapEditor => {
                self.me.draw(screen);
            }
        }

        for widget in &self.widgets {
            widget.lock().unwrap().draw(screen);
        }
    }
}
