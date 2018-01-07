use sdl2::controller::GameController;
use sdl2::joystick::Joystick;

pub struct Controllers {
    controllers: Vec<GameController>,
    joysticks: Vec<Joystick>,
    ids: Vec<u32>,
    j_ids: Vec<u32>,
    g_ids: Vec<u32>,
}

impl Controllers {
    pub fn new() -> Controllers {
        Controllers {
            controllers: Vec::new(),
            joysticks: Vec::new(),
            ids: Vec::new(),
            j_ids: Vec::new(),
            g_ids: Vec::new(),
        }
    }

    pub fn push_controller(&mut self, id: u32, controller: GameController) {
        if !self.ids.contains(&id) {
            info!("REGISTERED CONTROLLER {:?}", id);

            self.controllers.push(controller);
            self.ids.push(id);
            self.g_ids.push(id);
        }
    }

    pub fn push_joystick(&mut self, id: u32, joystick: Joystick) {
        if !self.ids.contains(&id) {
            info!("REGISTERED JOYSTICK {:?}", id);

            self.joysticks.push(joystick);
            self.ids.push(id);
            self.j_ids.push(id);
        }
    }

    pub fn is_controller(&self, id: u32) -> bool {
        self.g_ids.contains(&id)
    }

    pub fn is_joystick(&self, id: u32) -> bool {
        self.j_ids.contains(&id)
    }
}
