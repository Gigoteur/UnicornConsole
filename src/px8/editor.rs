use gfx::Screen;
use config::Players;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use px8::PX8Config;
use time;


pub fn point_in_rect(x: i32, y: i32, coord_x1: i32, coord_y1: i32, coord_x2: i32, coord_y2: i32) -> bool {
    (coord_x1 <= x && x < coord_x2) && (coord_y1 <= y && y < coord_y2)
}

pub struct State {
    mouse_x: i32,
    mouse_y: i32,
    mouse_state: u32,
    mouse_statep: u32,

    idx_sprites_batch: i32,
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

            idx_sprites_batch: 88,
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
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32, color: u32, text: String, highlight: bool) -> Button {
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
        let mut color = 3;
        if self.clicked {
            color = 1;
        }
        screen.print(self.text.clone(), self.x1 + 1, self.y1, color);
    }

    pub fn is_click(&mut self) -> bool {
        self.clicked
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
    pub fn new(state: Arc<Mutex<State>>, name: String, x: u32, y: u32, w: u32, h: u32, data: Vec<u8>, highlight: HashMap<u32, u32>) -> Widget {
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
            clicked: false
        }
    }

    pub fn is_click(&mut self) -> bool {
        self.clicked
    }

    pub fn update(&mut self) {
        let mouse_state = self.state.lock().unwrap().mouse_state;
        self.clicked = false;

        if mouse_state == 1 {
            let mouse_x = self.state.lock().unwrap().mouse_x as u32;
            let mouse_y = self.state.lock().unwrap().mouse_y as u32;

            self.clicked = (self.x1 <= mouse_x && mouse_x < self.x2) && (self.y1 <= mouse_y && mouse_y < self.y2);
        }
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        let mut idx_w = 0;
        let mut idx_h = 0;

        for pixel in &self.data {
            screen.pset((self.x1+idx_w) as i32, (self.y1+idx_h) as i32, *pixel as i32);

            idx_w += 1;
            if idx_w == self.w {
                idx_w = 0;
                idx_h += 1;
            }

        }
    }
}

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
            idx_x: 80,
            idx_y: 16,
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

            if point_in_rect(mouse_x, mouse_y,
                             idx_x_zoom_sprite as i32,
                             idx_y_zoom_sprite as i32,
                             (idx_x_zoom_sprite + 8 * 8) as i32,
                             (idx_y_zoom_sprite + 8 * 8) as i32) {
                let idx_x = ((mouse_x - idx_x_zoom_sprite as i32) as f64 * zoom_sprite as f64 / 8.).floor() as u32;
                let idx_y = ((mouse_y - idx_y_zoom_sprite as i32) as f64 * zoom_sprite as f64 / 8.).floor() as u32;

                let x_zoom_sprite = self.state.lock().unwrap().x_zoom_sprite;
                let y_zoom_sprite = self.state.lock().unwrap().y_zoom_sprite;

                screen.sset(x_zoom_sprite + idx_x as u32, y_zoom_sprite + idx_y as u32, self.current_color as i32);
            }

            if point_in_rect(mouse_x, mouse_y,
                             self.idx_x,
                             self.idx_y,
                             self.idx_x + 4 * 8,
                             self.idx_y + 4 * 8) {
                let idx_x = (((mouse_x - self.idx_x) as f64).floor() / 8.) as i32;
                let idx_y = (((mouse_y - self.idx_y) as f64).floor() / 8.) as i32;

                self.current_color = (idx_x + idx_y * 4) as u32;
                self.current_selection_x = idx_x;
                self.current_selection_y = idx_y;
            }
        }
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        let mut idx = 0;
        let mut x = self.idx_x;
        let mut y = self.idx_y;

        for i in 0..16 {
            let pos_x = x + (8 * (idx % 4));

            let pos_y = y;
            screen.rectfill(pos_x, pos_y, pos_x + 7, pos_y + 7, i);
            idx += 1;

            if idx > 1 && idx % 4 == 0 {
                y += 8;
            }
        }

        let current_selection_x = (self.idx_x + 8*self.current_selection_x) - 1;
        let current_selection_y = (self.idx_y + 8*self.current_selection_y) - 1;

        screen.rect(current_selection_x, current_selection_y, current_selection_x+9, current_selection_y+9, 7);
    }
}

pub struct Flags {
    state: Arc<Mutex<State>>,
    values: [u32; 8],
    flags: HashMap<u32, u32>,
}

impl Flags {
    pub fn new(state: Arc<Mutex<State>>) -> Flags {
        let values = [1, 2, 4, 8, 16, 32, 64, 128];
        let mut flags = HashMap::new();

        for i in values.iter() {
            flags.insert(*i, 0);
        }

        Flags {
            state: state.clone(),
            values: values,
            flags: flags,
        }
    }

    pub fn update(&mut self, screen: &mut Screen) {
        let mut idx = 0;
        let idx_sprite = self.state.lock().unwrap().current_sprite;

        for i in self.values.iter() {
            let flag = screen.fget(idx_sprite, *i as u8);
            let mut color = 0;
            if flag {
                color = 7;
            }
            self.flags.insert(*i, color);

            let mouse_state = self.state.lock().unwrap().mouse_state;
            if mouse_state == 1 {
                let mouse_x = self.state.lock().unwrap().mouse_x;
                let mouse_y = self.state.lock().unwrap().mouse_y;

                if point_in_rect(mouse_x, mouse_y,
                                 80+idx, 74, 82+idx, 76) {
                    screen.fset(idx_sprite, *i as u8, !flag);
                }
            }

            idx += 6;
        }
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        let mut idx = 0;

        for k in self.values.iter() {
            let color = self.flags[k];

            screen.rectfill(80 + idx, 74, 82 + idx, 76, color as i32);

            idx += 6
        }
    }
}

pub struct MapEditor {
    state: Arc<Mutex<State>>,
}

impl MapEditor {
    pub fn new(state: Arc<Mutex<State>>) -> MapEditor {
        MapEditor {
            state: state.clone(),
        }
    }

    pub fn update(&mut self, players: Arc<Mutex<Players>>, screen: &mut Screen) {}
    pub fn draw(&mut self, screen: &mut Screen) {

    }
}

pub struct SpriteEditor {
    state: Arc<Mutex<State>>,
    pp: PalettePicker,
}

impl SpriteEditor {
    pub fn new(state: Arc<Mutex<State>>) -> SpriteEditor {
        SpriteEditor {
            state: state.clone(),
            pp: PalettePicker::new(state.clone()),
        }
    }

    pub fn update(&mut self, players: Arc<Mutex<Players>>, screen: &mut Screen) {
        self.pp.update(screen);

        if players.lock().unwrap().btnp(0, 4) {
            let idx_zoom_sprite = self.state.lock().unwrap().idx_zoom_sprite;
            let sprite_available_zooms = self.state.lock().unwrap().sprite_available_zooms;

            let new_idx_zoom_sprite = (idx_zoom_sprite + 1) % sprite_available_zooms.len() as u32;
            self.state.lock().unwrap().idx_zoom_sprite = new_idx_zoom_sprite;

            self.state.lock().unwrap().zoom_sprite = sprite_available_zooms[new_idx_zoom_sprite as usize];
        }

        /*

        if players.lock().unwrap().btnp(0, 0) {
            buffer = self.tools.get_current_formatted_buffer();
            shift_buffer = shift("left", 1, buffer);
            self.tools.paste_formatted_buffer(shift_buffer);
        }

        if players.lock().unwrap().btnp(0, 1) {
            buffer = self.tools.get_current_formatted_buffer();
            shift_buffer = shift("right", 1, buffer);
            self.tools.paste_formatted_buffer(shift_buffer)
        }

        if players.lock().unwrap().btnp(0, 2) {
            buffer = self.tools.get_current_formatted_buffer()
            shift_buffer = shift("up", 1, buffer)
            self.tools.paste_formatted_buffer(shift_buffer)
        }

        if players.lock().unwrap().btnp(0, 3) {
            buffer = self.tools.get_current_formatted_buffer()
            shift_buffer = shift("down", 1, buffer)
            self.tools.paste_formatted_buffer(shift_buffer)
        }*/
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        self.pp.draw(screen);

        screen.print(format!("{:?}", self.state.lock().unwrap().current_sprite), 80, 64, 7);

        let x_zoom_sprite = self.state.lock().unwrap().x_zoom_sprite;
        let y_zoom_sprite = self.state.lock().unwrap().y_zoom_sprite;
        let zoom_sprite = self.state.lock().unwrap().zoom_sprite;
        let idx_x_zoom_sprite = self.state.lock().unwrap().idx_x_zoom_sprite;
        let idx_y_zoom_sprite = self.state.lock().unwrap().idx_y_zoom_sprite;


        screen.sspr(x_zoom_sprite,
                    y_zoom_sprite,
                    8*zoom_sprite as u32,
                    8*zoom_sprite as u32,
                    idx_x_zoom_sprite as i32,
                    idx_y_zoom_sprite as i32,
                    8*8,
                    8*8,
                    false,
                    false);
    }
}

pub struct SpritesMap {
    state: Arc<Mutex<State>>,
    buttons: [i32; 4],
    buttons_map: Vec<Arc<Mutex<Button>>>,
    flags: Flags,
}

impl SpritesMap {
    pub fn new(state: Arc<Mutex<State>>) -> SpritesMap {
        let mut buttons_map = Vec::new();
        buttons_map.push(Arc::new(Mutex::new(Button::new(96, 79, 100, 87, 2, "1".to_string(), true))));
        buttons_map.push(Arc::new(Mutex::new(Button::new(101, 79, 105, 87, 2, "2".to_string(), false))));
        buttons_map.push(Arc::new(Mutex::new(Button::new(106, 79, 110, 87, 2, "3".to_string(), false))));
        buttons_map.push(Arc::new(Mutex::new(Button::new(111, 79, 115, 87, 2, "4".to_string(), false))));

        SpritesMap {
            state: state.clone(),
            buttons: [96, 79, 115, 87],
            buttons_map: buttons_map.clone(),
            flags: Flags::new(state.clone()),
        }
    }

    pub fn update(&mut self, screen: &mut Screen) {
        self.state.lock().unwrap().on_current_sprite = false;

        if self.state.lock().unwrap().mouse_state == 1 {
            let mouse_x = self.state.lock().unwrap().mouse_x;
            let mouse_y = self.state.lock().unwrap().mouse_y;

            if point_in_rect(mouse_x, mouse_y,
                             self.buttons[0], self.buttons[1],
                             self.buttons[2], self.buttons[3]) {
                let mut btn_idx = 0;
                for button in &self.buttons_map {
                    let mut button = button.lock().unwrap();
                    button.update(mouse_x, mouse_y);
                    if button.is_click() {
                        self.state.lock().unwrap().idx_map = btn_idx;
                        self.state.lock().unwrap().current_sprite = 64 * btn_idx;
                    }

                    btn_idx += 1;
                }
            }


            let idx_sprites_batch = self.state.lock().unwrap().idx_sprites_batch;

            if (mouse_y >= self.state.lock().unwrap().idx_sprites_batch) && mouse_y < 120 {
                let y = ((mouse_y - idx_sprites_batch) as f64 / 8.).floor() as u32;
                let x = (mouse_x as f64 / 8.).floor() as u32;

                let idx_map = self.state.lock().unwrap().idx_map;
                self.state.lock().unwrap().current_sprite = (x + y * 16) + 64 * idx_map;

                let current_sprite = self.state.lock().unwrap().current_sprite;
                self.state.lock().unwrap().x_zoom_sprite = (current_sprite % 16) * 8;
                self.state.lock().unwrap().y_zoom_sprite = (current_sprite as f64 / 16.).floor() as u32 * 8;
            }
        }

        // Update flags
        self.flags.update(screen);
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        self.draw_sprite_map(screen);
        self.draw_sprite_flags(screen);
        self.draw_button(screen);
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
        let mut y = self.state.lock().unwrap().idx_sprites_batch;
        for j in 0..4 {
            let mut x = 0;
            for _ in 0..16 {
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

        current_sprite_x -= 1;
        screen.rect(current_sprite_x, current_sprite_y, current_sprite_x+8*zoom, current_sprite_y+8*zoom, 7);
        screen.rect(current_sprite_x - 1, current_sprite_y - 1, current_sprite_x+1+8*zoom, current_sprite_y+1+8*zoom, 0);
    }

    pub fn draw_button(&mut self, screen: &mut Screen) {
        for button in &self.buttons_map {
            button.lock().unwrap().draw(screen);
        }
    }

    pub fn draw_information(&mut self, screen: &mut Screen) {
        if self.state.lock().unwrap().on_current_sprite {
            let on_current_sprite_x = self.state.lock().unwrap().on_current_sprite_x;
            let on_current_sprite_y = self.state.lock().unwrap().on_current_sprite_y;

            screen.print(format!("{:?},{:?}", on_current_sprite_x, on_current_sprite_y),
                         0, 120,
                         5);
        }
    }
}

pub struct Editor {
    state: Arc<Mutex<State>>,
    sm: SpritesMap,
    se: SpriteEditor,
    widgets: Vec<Arc<Mutex<Widget>>>,
}

impl Editor {
    pub fn new() -> Editor {
        let state = Arc::new(Mutex::new(State::new()));

        let mut widgets = Vec::new();

        widgets.push(Arc::new(Mutex::new(Widget::new(state.clone(), "SPRITE EDITOR".to_string(), 110, 1, 8, 6,
                                                       vec![6, 11, 11, 11, 11, 11, 11, 6,
                                                            11, 6, 6, 6, 6, 6, 6, 11,
                                                            11, 6, 11, 11, 11, 11, 6, 11,
                                                            11, 6, 11, 11, 11, 11, 6, 11,
                                                            11, 6, 6, 6, 6, 6, 6, 11,
                                                            6, 11, 11, 11, 11, 11, 11, 6],
                                                        HashMap::new()))));
        widgets.push(Arc::new(Mutex::new(Widget::new(state.clone(), "MAP EDITOR".to_string(), 119, 1, 8, 6,
                                                     vec![11, 11, 11, 11, 11, 11, 11, 11,
                                                          11, 6, 6, 6, 6, 6, 6, 11,
                                                          11, 6, 11, 11, 11, 11, 6, 11,
                                                          11, 6, 11, 11, 11, 11, 6, 11,
                                                          11, 6, 6, 6, 6, 6, 6, 11,
                                                          11, 11, 11, 11, 11, 11, 11, 11],
                                                     HashMap::new()))));

        Editor {
            state: state.clone(),
            sm: SpritesMap::new(state.clone()),
            se: SpriteEditor::new(state.clone()),
            widgets: widgets,
        }
    }

    pub fn init(&mut self, config: Arc<Mutex<PX8Config>>) {
        info!("[EDITOR] Init");
        config.lock().unwrap().toggle_mouse(true);
    }

    pub fn update(&mut self, players: Arc<Mutex<Players>>) -> bool {

        true
    }

    pub fn draw(&mut self, players: Arc<Mutex<Players>>, screen: &mut Screen) -> f64 {
        let current_time = time::now();

        screen.cls();

        self.state.lock().unwrap().update(players.clone());
        self.sm.update(screen);
        self.se.update(players.clone(), screen);

        for widget in &self.widgets {
            widget.lock().unwrap().update();
        }

        for widget in &self.widgets {
            if widget.lock().unwrap().is_click() {
                info!("CLICKED");
            }
        }

        // Draw contour
        screen.rectfill(0, 0, 128, 8, 11);
        screen.rectfill(0, 120, 128, 128, 11);
        screen.rectfill(0, 75, 128, 87, 5);
        screen.rectfill(0, 9, 8, 77, 5);
        screen.rectfill(75, 9, 128, 76, 5);

        // Draw sprites map
        self.sm.draw(screen);

        // Draw sprite editor
        self.se.draw(screen);

        for widget in &self.widgets {
            widget.lock().unwrap().draw(screen);
        }

        let diff_time = time::now() - current_time;
        let nanoseconds = (diff_time.num_nanoseconds().unwrap() as f64) -
            (diff_time.num_seconds() * 1000000000) as f64;

        // Elapsed time
        diff_time.num_seconds() as f64 + nanoseconds / 1000000000.0
    }
}
