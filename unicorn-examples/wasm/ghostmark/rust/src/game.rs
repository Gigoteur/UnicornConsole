use unicorn_rs::api;

pub struct Ghost {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: u32,
}

impl Ghost {
    pub fn new() -> Ghost {
        Ghost {
            x: api::rnd_range(10, api::mode_width() as i32) as f32,
            y: api::rnd_range(10, api::mode_height() as i32) as f32,
            vx: api::rnd_range(-10, 10) as f32 / 10.0,
            vy: api::rnd_range(-10, 10) as f32 / 10.0,
            color: api::rnd_range(0, 16) as u32,
        }
    }

    pub fn update(&mut self) {
        self.x = self.x + self.vx;
        self.y = self.y + self.vy;

        if self.x < 0. || self.x > 128. {
            self.vx = -self.vx;
        }

        if self.y > api::mode_height() as f32 {
            self.y = api::mode_height() as f32;
            self.vy = -(api::rnd_range(0, 100) as f32 / 25.0);
        }

        self.vy = self.vy + 0.05;
    }

    pub fn draw(&mut self) {
        api::spr(self.color, self.x as i32, self.y as i32, 1, 1, false, false, 0.0, 1.0, false);
    }
}

// Our game state. Edit this as you wish.
pub struct MyGame {
    frame_counter: usize,
    pub sprites: Vec<Ghost>,
}

impl crate::Game for MyGame {
    /// Handle all of your initialization logic here.
    fn init() -> Self {
        let mut sprites = Vec::new();

        for _ in 0..500 {
            sprites.push(Ghost::new());
        }

        Self {
            frame_counter: 0,
            sprites: sprites,
        }
    }

    /// Handle all of your game state logic here
    fn update(&mut self) {
        if api::btnp(5, 0) {
            for _ in 0..500 {
                self.sprites.push(Ghost::new());
            }
        }
        
        for ghost in &mut self.sprites {
            ghost.update();
        }
    }

    /// Handle all of your rendering code here
    fn draw(&mut self) {
        api::cls(0);
        for ghost in &mut self.sprites {
            ghost.draw();
        }
    }
}