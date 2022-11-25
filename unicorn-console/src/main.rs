
use unicorn;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{DeviceEvent, Event, MouseScrollDelta, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uc = unicorn::Unicorn::new();

    let event_loop = EventLoop::new();

    let window = init_window(&event_loop);
    let window_size = window.inner_size();
    let scale_factor = window.scale_factor() as f32;

}

const DEFAULT_WINDOW_RESOLUTION: Resolution = Resolution::High;

fn init_window(event_loop: &EventLoop<()>) -> Window {
    let size = LogicalSize::new(
        DEFAULT_WINDOW_RESOLUTION.width() as f64,
        DEFAULT_WINDOW_RESOLUTION.height() as f64,
    );
    WindowBuilder::new()
        .with_title("Gamercade Console")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(event_loop)
        .unwrap()
}

fn init_pixels(window: &Window) -> Pixels {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    Pixels::new(320, 180, surface_texture).unwrap()
}