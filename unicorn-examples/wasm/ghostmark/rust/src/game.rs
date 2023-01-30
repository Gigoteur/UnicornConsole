use unicorn_rs::api;

// Our game state. Edit this as you wish.
pub struct MyGame {
    frame_counter: usize,
    x_pos: i32,
    y_pos: i32,
}

impl crate::Game for MyGame {
    /// Handle all of your initialization logic here.
    fn init() -> Self {
        // Initialize our values to 0, and width/height divided
        // by two.
        Self {
            frame_counter: 0,
            x_pos: 0,
            y_pos: 0,
        }
    }

    /// Handle all of your game state logic here
    fn update(&mut self) {
    }

    /// Handle all of your rendering code here
    fn draw(&self) {
        api::cls(0);
    }
}