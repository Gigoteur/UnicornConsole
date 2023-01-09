use std::net::SocketAddr;
use bytemuck::{Pod, Zeroable};

use ggrs::{Config, PlayerType};
use wasmtime::Global;

use unicorn::input::{Buttons, InputState, MouseState};
use crate::UnicornConsole;

#[derive(Clone)]
pub struct UnicornConsoleState {
    pub(crate) previous_buttons: Box<[Buttons]>,
 //   pub(crate) memories: Vec<Vec<u8>>,
 //   pub(crate) mutable_globals: Vec<Global>,
}

pub struct SaveStateDefinition {
    pub(crate) memories: Vec<String>,
    pub(crate) mutable_globals: Vec<String>,
}

#[derive(Pod, Zeroable, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct NetworkInputState {
    pub input_state: InputState,
    pub mouse_state: MouseState,
}

impl Config for UnicornConsole {
    type Input = NetworkInputState;
    type State = UnicornConsoleState;
    type Address = SocketAddr;
}

#[derive(Clone)]
pub struct SessionDescriptor {
    pub num_players: usize,
    pub player_types: Box<[PlayerType<SocketAddr>]>,
    pub port: u16,
}