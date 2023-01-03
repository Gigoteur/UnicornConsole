use egui::{Button, Context};
use std::path::PathBuf;
use std::net::SocketAddr;

use gilrs::Gilrs;
use pixels::Pixels;
use rfd::FileDialog;
use winit::{window::Window};
use ggrs::{P2PSession, PlayerType, SessionBuilder, SessionState, UdpNonBlockingSocket};

use unicorn;

pub mod controller;
pub mod framework;
pub mod play_mode_gui;

use crate::input::LocalInputManager;
use crate::DEFAULT_WINDOW_RESOLUTION;
use crate::network::SessionDescriptor;
use controller::ControllerGui;
use play_mode_gui::PlayModeGui;
use crate::UnicornConsole;

pub struct Gui {
    pub window_open: bool,
    pub game_file: Option<PathBuf>,

    pub unicorn_console: Option<UnicornConsole>,

    pub controller_gui: ControllerGui,
    pub play_mode_gui : PlayModeGui,
}


impl Default for Gui {
    fn default() -> Self {
        Self {
            window_open: true,
            game_file: None,
            unicorn_console : None,
            controller_gui: ControllerGui::default(),
            play_mode_gui: PlayModeGui::default(),
        }
    }
}

impl Gui {
    fn ui(
        &mut self,
        pixels: &mut Pixels,
        window: &Window,
        session: &mut Option<P2PSession<UnicornConsole>>,
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
                                .add_filter("corn (.corn), p8 (.p8)", &["corn", "p8"])
                                .pick_file();
                        };

                        if let Some(file) = &self.game_file {
                            let filename = file
                                .file_name()
                                .expect("filename not found")
                                .to_string_lossy()
                                .to_string();
                            ui.label(filename.clone());
                        }
                    });
                });

                self.controller_gui
                .draw(ui, session.is_none(), input, gilrs);

                self.play_mode_gui.draw(ui);

                // Draw internal content
                let launch_game_text = "Launch Game";
                
                ui.separator();

                ui.horizontal(|ui| {
                    let launch_game = Button::new(launch_game_text);
                    if ui
                        .add_enabled(self.game_file.is_some(), launch_game)
                        .clicked()
                    {
                        // Launch the game !
                        *session = self.try_launch_game(pixels, window);
                    }

                    let buttons_enabled = self.game_file.is_some();

                    // Reset the game
                    if ui
                        .add_enabled(buttons_enabled, Button::new("Reset Game"))
                        .clicked()
                    {
                    }

                    // Quit the console
                    if ui
                        .add_enabled(buttons_enabled, Button::new("Quit"))
                        .clicked()
                    {
                    }
                });
            });
    }

    fn init_with_console(
        &mut self,
        rom: unicorn::core::Unicorn,
        pixels: &mut Pixels,
        window: &Window,
        session_descriptor: SessionDescriptor,
    ) -> P2PSession<UnicornConsole> {
       /*  pixels.resize_buffer(rom.width(), rom.height());
        window.set_inner_size(PhysicalSize::new(
            rom.width().max(DEFAULT_WINDOW_RESOLUTION.width() as u32),
            rom.height().max(DEFAULT_WINDOW_RESOLUTION.height() as u32),
        ));*/

        let (max_prediction, new_session) = {
            let new_session = init_session(
                &rom,
                session_descriptor.port,
                &session_descriptor.player_types,
            );
            (new_session.max_prediction(), new_session)
        };

        self.window_open = false;

        let (mut console, reset) = UnicornConsole::new(rom);//, session_descriptor, max_prediction);
        //console.sync_mouse(window);

        self.unicorn_console = Some(console);
        //self.initial_state = Some(reset);
        new_session
    }

    pub(crate) fn try_launch_game(
        &mut self,
        pixels: &mut Pixels,
        window: &Window,
    ) -> Option<P2PSession<UnicornConsole>> {
        let path = self.game_file.as_ref().unwrap();

        let session_descriptor = self
            .play_mode_gui
            .generate_session_descriptor(self.controller_gui.local_player_count)?;

        let mut rom = unicorn::core::Unicorn::new();
        rom.load_cartridge(String::from(path.to_string_lossy()));
        rom.init();

        Some(self.init_with_console(rom, pixels, window, session_descriptor))
    }

}

fn init_session(
    rom: &unicorn::core::Unicorn,
    port: u16,
    players: &[PlayerType<SocketAddr>],
) -> P2PSession<UnicornConsole> {
    let mut sess_builder = SessionBuilder::new()
        .with_num_players(players.len())
        .with_fps(60)
        .unwrap();

    for (id, address) in players.iter().enumerate() {
        sess_builder = sess_builder.add_player(*address, id).unwrap();
    }

    let socket = UdpNonBlockingSocket::bind_to_port(port).unwrap();
    sess_builder.start_p2p_session(socket).unwrap()
}