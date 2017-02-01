use sdl2::controller::GameController;
use sdl2::joystick::Joystick;

pub struct Controllers {
    controllers: Vec<GameController>,
    joysticks: Vec<Joystick>,
}

impl Controllers {
    pub fn new() -> Controllers {
        Controllers {
            controllers: Vec::new(),
            joysticks: Vec::new(),
        }
    }

    pub fn push_controller(&mut self, controller: GameController) {
        self.controllers.push(controller);
    }

    pub fn push_joystick(&mut self, joystick: Joystick) {
        self.joysticks.push(joystick);
    }
}