use egui::{Button, Context};


use self::{controller_gui::ControllerGui, play_mode_gui::PlayModeGui};
pub mod controller_gui;
pub mod framework;
pub mod play_mode_gui;

pub struct Gui {
    pub window_open: bool,
    pub game_file: Option<PathBuf>,

    pub seed: String,

    pub wasm_console: Option<WasmConsole>,
    pub initial_state: Option<WasmConsoleState>,

    pub play_mode_gui: PlayModeGui,
    pub controller_gui: ControllerGui,
}


const DEFAULT_SEED: &str = "a12cade";

impl Default for Gui {
    fn default() -> Self {
        Self {
            seed: DEFAULT_SEED.to_string(),
            window_open: true,
            game_file: None,

            wasm_console: None,
            initial_state: None,

            play_mode_gui: PlayModeGui::default(),
            controller_gui: ControllerGui::default(),
        }
    }
}

impl Gui {
    fn ui(
        &mut self,
        pixels: &mut Pixels,
        window: &Window,
        session: &mut Option<P2PSession<WasmConsole>>,
        ctx: &Context,
        input: &mut LocalInputManager,
        gilrs: &mut Gilrs,
    ) {
        let mut is_open = self.window_open;
        egui::Window::new("Main Menu")
            .open(&mut is_open)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Select Game").clicked() {
                            self.game_file = FileDialog::new()
                                .add_filter("gcrom (.gcrom), wasm (.wasm)", &["gcrom", "wasm"])
                                .pick_file();
                        };

                        if let Some(file) = &self.game_file {
                            let filename = file
                                .file_name()
                                .expect("filename not found")
                                .to_string_lossy()
                                .to_string();
                            ui.label(filename);
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Random Seed:");
                        ui.text_edit_singleline(&mut self.seed);
                        if u64::from_str_radix(&self.seed, 16).is_err() {
                            self.seed = DEFAULT_SEED.to_string()
                        }
                    })
                });

                self.controller_gui
                    .draw(ui, session.is_none(), input, gilrs);

                self.play_mode_gui.draw(ui);

                let launch_game_text = if let Some(session) = session {
                    if session.current_state() == SessionState::Synchronizing {
                        "Waiting to establish connection..."
                    } else {
                        "Connected!"
                    }
                } else {
                    "Launch Game"
                };

                ui.separator();

                ui.horizontal(|ui| {
                    let launch_game = egui::Button::new(launch_game_text);
                    if ui
                        .add_enabled(self.game_file.is_some() && session.is_none(), launch_game)
                        .clicked()
                    {
                        *session = self.try_launch_game(pixels, window);
                    }

                    let buttons_enabled = self.game_file.is_some() && session.is_some();

                    if ui
                        .add_enabled(buttons_enabled, Button::new("Reset Game"))
                        .clicked()
                    {
                        let console = self.wasm_console.as_mut().unwrap();
                        console.load_save_state(self.initial_state.as_ref().unwrap().clone());
                    }

                    if ui
                        .add_enabled(buttons_enabled, Button::new("Quit Game"))
                        .clicked()
                    {
                        self.wasm_console = None;
                        *session = None;
                    }
                });
            });
    }

    /// Quickly launch a single player session, usually from the command line
    pub(crate) fn fast_launch_game(
        &mut self,
        game_path: PathBuf,
        seed: u64,
        pixels: &mut Pixels,
        window: &Window,
    ) -> Option<P2PSession<WasmConsole>> {
        let rom = match Rom::try_load(&game_path) {
            Err(e) => {
                println!("{}", e);
                return None;
            }
            Ok(rom) => rom,
        };

        self.game_file = Some(game_path);

        let session_descriptor = SessionDescriptor {
            num_players: 1,
            player_types: vec![PlayerType::Local].into_boxed_slice(),
            port: 8000,
        };

        Some(self.init_with_console(seed, rom, pixels, window, session_descriptor))
    }

    fn init_with_console(
        &mut self,
        seed: u64,
        rom: Rom,
        pixels: &mut Pixels,
        window: &Window,
        session_descriptor: SessionDescriptor,
    ) -> P2PSession<WasmConsole> {
        pixels.resize_buffer(rom.width() as u32, rom.height() as u32);
        window.set_inner_size(PhysicalSize::new(
            rom.width().max(DEFAULT_WINDOW_RESOLUTION.width()),
            rom.height().max(DEFAULT_WINDOW_RESOLUTION.height()),
        ));

        let (max_prediction, new_session) = {
            let new_session = init_session(
                &rom,
                session_descriptor.port,
                &session_descriptor.player_types,
            );
            (new_session.max_prediction(), new_session)
        };

        self.window_open = false;

        let (mut console, reset) = WasmConsole::new(rom, seed, session_descriptor, max_prediction);
        console.sync_mouse(window);

        self.wasm_console = Some(console);
        self.initial_state = Some(reset);
        new_session
    }

    pub(crate) fn try_launch_game(
        &mut self,
        pixels: &mut Pixels,
        window: &Window,
    ) -> Option<P2PSession<WasmConsole>> {
        let path = self.game_file.as_ref().unwrap();

        let session_descriptor = self
            .play_mode_gui
            .generate_session_descriptor(self.controller_gui.local_player_count)?;

        let rom = match Rom::try_load(path) {
            Err(e) => {
                println!("{}", e);
                return None;
            }
            Ok(rom) => rom,
        };

        let seed = u64::from_str_radix(&self.seed, 16).unwrap();

        Some(self.init_with_console(seed, rom, pixels, window, session_descriptor))
    }
}

fn init_session(
    rom: &Rom,
    port: u16,
    players: &[PlayerType<SocketAddr>],
) -> P2PSession<WasmConsole> {
    let mut sess_builder = SessionBuilder::new()
        .with_num_players(players.len())
        .with_fps(rom.frame_rate.frames_per_second())
        .unwrap();

    for (id, address) in players.iter().enumerate() {
        sess_builder = sess_builder.add_player(*address, id).unwrap();
    }

    let socket = UdpNonBlockingSocket::bind_to_port(port).unwrap();
    sess_builder.start_p2p_session(socket).unwrap()
}