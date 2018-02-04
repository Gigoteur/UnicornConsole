#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use self::wasm::*;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
mod normal;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub use self::normal::*;