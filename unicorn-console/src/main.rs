use unicorn;
use log::{debug, error, log_enabled, info, Level};
use env_logger;

use std::{
    path::PathBuf,
    time::{Duration, Instant},
    env,
};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{DeviceEvent, Event, MouseScrollDelta, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut uc = unicorn::core::Unicorn::new();
    uc.setup();

    let event_loop = EventLoop::new();

    let window = init_window(&event_loop);
    let window_size = window.inner_size();
    let scale_factor = window.scale_factor() as f32;
    let mut pixels = init_pixels(&window);

    let mut input = WinitInputHelper::new();
    let mut last_update = Instant::now();
    
    event_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent { event, .. } = &event {
        }

        // Close events
        if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
            *control_flow = ControlFlow::Exit;
            return;
        }

        if input.key_pressed(VirtualKeyCode::Space) {
         //   framework.gui.window_open = !framework.gui.window_open;
        }

        // Update the scale factor
        if let Some(scale_factor) = input.scale_factor() {
         //   framework.scale_factor(scale_factor);
        }

        // Resize the window
        if let Some(size) = input.window_resized() {
            pixels.resize_surface(size.width, size.height);
          //  framework.resize(size.width, size.height);
        }

        // Handle input events
        if input.update(&event) {
            let screen = &mut uc.screen.lock().unwrap();
            screen.cls(5);

            screen.line(0, 0, 50, 50, 7);
            screen.trigon(0, 0, 50, 70, 100, 90, 4);
            screen.print("Hello World".to_string(), 64, 64, 6);

            pixels.get_frame_mut().copy_from_slice(&screen.pixel_buffer);

            pixels.render();


            /*let render_result = pixels.render_with(|encoder, render_target, context| {
                context.scaling_renderer.render(encoder, render_target);

            //framework.render(encoder, render_target, context)?;

                Ok(())
            });

            if render_result.is_err() {
                println!("render_with failed");
                *control_flow = ControlFlow::Exit;
                return;
            }*/

            window.request_redraw();
        }

    });

}

const DEFAULT_WINDOW_RESOLUTION: unicorn::core::resolution::Resolution = unicorn::core::resolution::Resolution::High;

fn init_window(event_loop: &EventLoop<()>) -> Window {
    let size = LogicalSize::new(
        DEFAULT_WINDOW_RESOLUTION.width() as f64,
        DEFAULT_WINDOW_RESOLUTION.height() as f64,
    );
    WindowBuilder::new()
        .with_title("Unicorn Console")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(event_loop)
        .unwrap()
}


fn init_pixels(window: &Window) -> Pixels {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    Pixels::new(128, 128, surface_texture).unwrap()
}