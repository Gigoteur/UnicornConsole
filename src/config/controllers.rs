use sdl2::controller::GameController;
use sdl2::joystick::Joystick;

pub struct Controllers {
    controllers: Vec<GameController>,
    joysticks: Vec<Joystick>,
    ids: Vec<u32>,
}

impl Controllers {
    pub fn new() -> Controllers {
        Controllers {
            controllers: Vec::new(),
            joysticks: Vec::new(),
            ids: Vec::new(),
        }
    }

    pub fn push_controller(&mut self, id: u32, controller: GameController) {
        self.controllers.push(controller);
        self.ids.push(id);
    }

    pub fn push_joystick(&mut self, id: u32, joystick: Joystick) {
        self.joysticks.push(joystick);
        self.ids.push(id);
    }

    pub fn contains(&mut self, id: u32) -> bool {
        self.ids.contains(&id)
    }
}