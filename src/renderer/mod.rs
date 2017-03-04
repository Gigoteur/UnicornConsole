#[cfg(feature = "gfx_rs_renderer")]
pub mod renderer {

}

#[cfg(not(feature = "gfx_rs_renderer"))]
pub mod renderer {
    use px8;
    use gfx::{Scale, Screen};

    use sdl2::VideoSubsystem;
    use sdl2::render;
    use sdl2::pixels::PixelFormatEnum;
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Debug)]
    pub enum RendererError {
        Sdl(String),
        Renderer(String),
        Other(String),
    }

    pub type RendererResult<T> = Result<T, RendererError>;

    pub struct Renderer {
        pub renderer: render::Renderer<'static>,
        pub texture: render::Texture,
    }

    impl Renderer {
        pub fn new(sdl_video: VideoSubsystem, fullscreen: bool, opengl: bool, scale: Scale) -> RendererResult<Renderer> {
            info!("[SDL] Creating window fullscreen={:?} opengl={:?}", fullscreen, opengl);

            let mut window_builder = sdl_video.window("PX8",
                                                      (px8::SCREEN_WIDTH as usize * scale.factor()) as u32,
                                                      (px8::SCREEN_HEIGHT as usize * scale.factor()) as u32);

            let window;
            if opengl {
                if fullscreen {
                    window = window_builder.fullscreen().opengl().build().unwrap();
                } else {
                    window = window_builder.resizable().position_centered().opengl().build().unwrap();
                }
            } else {
                if fullscreen {
                    window = window_builder.fullscreen().build().unwrap();
                } else {
                    window = window_builder.resizable().position_centered().build().unwrap();
                }
            }

            info!("[SDL] Creating renderer");
            let renderer = window.renderer().accelerated().present_vsync().build().unwrap();

            info!("[SDL] Creating texture");
            let texture = renderer.create_texture(PixelFormatEnum::BGR24,
                                                  render::TextureAccess::Streaming,
                                                  px8::SCREEN_WIDTH as u32,
                                                  px8::SCREEN_HEIGHT as u32).unwrap();


            Ok(Renderer {
                renderer: renderer,
                texture: texture,
            })
        }

        pub fn blit(&mut self, screen: Arc<Mutex<Screen>>) {
            self.texture.update(None,
                                &mut *screen.lock().unwrap().buffer_rgb,
                                px8::SCREEN_WIDTH * 3).unwrap();

            self.renderer.clear();
            self.renderer.copy(&self.texture, None, None);
            self.renderer.present();
        }

        pub fn update_dimensions(&mut self) {
        }

        pub fn get_dimensions(&mut self) -> (u32, u32) {
            let size = self.renderer.window().unwrap().size();

            return (size.0, size.1)
        }
    }
}