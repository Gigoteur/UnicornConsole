use std::net::SocketAddr;

use egui::{Slider, Ui};
use ggrs::PlayerType;

use crate::network::SessionDescriptor;

#[derive(Eq, PartialEq)]
pub(crate) enum PlayMode {
    Local,
    Networked(Networked),
}

#[derive(PartialEq, Eq)]
pub(crate) struct Networked {
    pub(crate) remote_addr: String,
    pub(crate) instance_id: usize,
    pub(crate) remote_player_count: usize,
    pub(crate) port: String,
}

impl Default for Networked {
    fn default() -> Self {
        Self {
            remote_addr: Default::default(),
            instance_id: 1,
            remote_player_count: 1,
            port: Default::default(),
        }
    }
}

impl Default for PlayModeGui {
    fn default() -> Self {
        Self {
            play_mode: PlayMode::Local,
        }
    }
}

pub struct PlayModeGui {
    pub(crate) play_mode: PlayMode,
}

impl PlayModeGui {
    pub(crate) fn draw(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label("Play Mode:");
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(matches!(self.play_mode, PlayMode::Local), "Local")
                    .clicked()
                {
                    self.play_mode = PlayMode::Local
                }
                if ui
                    .selectable_label(
                        matches!(self.play_mode, PlayMode::Networked(..)),
                        "Networked",
                    )
                    .clicked()
                {
                    self.play_mode = PlayMode::Networked(Networked::default())
                };
            });

            if let PlayMode::Networked(networked) = &mut self.play_mode {
                ui.horizontal(|ui| {
                    ui.label("Local Port: ");
                    ui.text_edit_singleline(&mut networked.port);
                });

                ui.add(Slider::new(&mut networked.instance_id, 1..=2).text("Unique Instance Id"));

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Remote Address:");
                    ui.text_edit_singleline(&mut networked.remote_addr);
                });

                ui.add(
                    Slider::new(&mut networked.remote_player_count, 1..=4)
                        .text("Remote Player Count"),
                );
            }
        });
    }

    pub(crate) fn generate_session_descriptor(
        &self,
        local_player_count: usize,
    ) -> Option<SessionDescriptor> {
        let mut player_types = Vec::new();

        let port = match &self.play_mode {
            PlayMode::Local => {
                player_types.extend(std::iter::repeat(PlayerType::Local).take(local_player_count));
                8000
            }
            PlayMode::Networked(networked) => {
                let remote_addr = networked.remote_addr.parse::<SocketAddr>();
                let port = networked.port.parse::<u16>();

                if remote_addr.is_err() {
                    println!("Remote Addr is invalid");
                    return None;
                } else if port.is_err() {
                    println!("Port is invalid");
                    return None;
                }

                let remote_addr = remote_addr.unwrap();
                let port = port.unwrap();

                if networked.instance_id == 1 {
                    player_types
                        .extend(std::iter::repeat(PlayerType::Local).take(local_player_count));
                    player_types.extend(
                        std::iter::repeat(PlayerType::Remote(remote_addr))
                            .take(networked.remote_player_count),
                    );
                } else if networked.instance_id == 2 {
                    player_types.extend(
                        std::iter::repeat(PlayerType::Remote(remote_addr))
                            .take(networked.remote_player_count),
                    );
                    player_types
                        .extend(std::iter::repeat(PlayerType::Local).take(local_player_count));
                } else {
                    println!("Player # should be 1 or 2");
                    return None;
                };

                port
            }
        };

        let player_types = player_types.into_boxed_slice();

        Some(SessionDescriptor {
            num_players: player_types.len(),
            player_types,
            port,
        })
    }
}