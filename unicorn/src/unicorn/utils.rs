use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use gfx::Screen;
use config::Players;


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

pub struct Widget {
    pub name: String,
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
    pub fn new(name: String,
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

    pub fn update(&mut self, mouse_state: u32, mouse_x: u32, mouse_y: u32) {
        if mouse_state == 1 {
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

    pub fn is_clickable(&mut self, mouse_state: u32, mouse_x: u32, mouse_y: u32) -> bool {
        if mouse_state == 1 {
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
