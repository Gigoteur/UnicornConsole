use std::net::SocketAddr;

use unicorn::input::{Buttons, InputState, MouseState};
use ggrs::{Config, PlayerType};
use wasmtime::Global;

use unicorn::core::Unicorn;

#[derive(Clone)]
pub struct WasmConsoleState {
    pub(crate) previous_buttons: Box<[Buttons]>,
    pub(crate) memories: Vec<Vec<u8>>,
    pub(crate) mutable_globals: Vec<Global>,
}

pub struct SaveStateDefinition {
    pub(crate) memories: Vec<String>,
    pub(crate) mutable_globals: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct NetworkInputState {
    pub input_state: InputState,
    pub mouse_state: MouseState,
}

/*impl Config for Unicorn {
    type Input = NetworkInputState;
    type State = WasmConsoleState;
    type Address = SocketAddr;
}*/

#[derive(Clone)]
pub struct SessionDescriptor {
    pub num_players: usize,
    pub player_types: Box<[PlayerType<SocketAddr>]>,
    pub port: u16,
}