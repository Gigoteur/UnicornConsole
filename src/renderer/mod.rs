#[cfg(feature = "gfx_rs_renderer")]
pub mod renderer {}

#[cfg(not(feature = "gfx_rs_renderer"))]
pub mod renderer {
    use px8;
    use gfx::{Scale, Screen};

    use sdl2::VideoSubsystem;
    use sdl2::render;
    use sdl2::rect::Rect;
    use sdl2::rect::Point;
    use sdl2::pixels::PixelFormatEnum;
    use std::sync::{Arc, Mutex};
    use num;
    use time::PreciseTime;


    #[derive(Clone, Debug)]
    pub enum RendererError {
        //        Sdl(String),
        //        Renderer(String),
        //        Other(String),
    }

    pub type RendererResult<T> = Result<T, RendererError>;

    pub struct Renderer {
        pub renderer: render::Renderer<'static>,
        pub texture: render::Texture,

        window_width: u32,
        window_height: u32,
        viewport_width: u32,
        viewport_height: u32,
        viewport_offset: Point,
        frame: u32,
    }

    impl Renderer {
        pub fn new(sdl_video: VideoSubsystem,
                   fullscreen: bool,
                   opengl: bool,
                   scale: Scale)
                   -> RendererResult<Renderer> {
            info!("[SDL] Creating window fullscreen={:?} opengl={:?}",
                  fullscreen,
                  opengl);

            let mut window_builder =
                sdl_video.window("PX8",
                                 (px8::SCREEN_WIDTH as usize * scale.factor()) as u32,
                                 (px8::SCREEN_HEIGHT as usize * scale.factor()) as u32);

            let wb = if fullscreen {
                window_builder.fullscreen()
            } else {
                window_builder.resizable().position_centered()
            };

            let window = (if opengl { wb.opengl() } else { wb }).build().unwrap();

            info!("[SDL] Creating renderer");
            let renderer = window
                .renderer()
                .accelerated()
                .present_vsync()
                .build()
                .unwrap();

            info!("[SDL] Creating texture");
            let texture = renderer
                .create_texture(PixelFormatEnum::BGR24,
                                render::TextureAccess::Streaming,
                                px8::SCREEN_WIDTH as u32,
                                px8::SCREEN_HEIGHT as u32)
                .unwrap();

            Ok(Renderer {
                   renderer: renderer,
                   texture: texture,
                   window_width: 0,
                   window_height: 0,
                   viewport_width: 0,
                   viewport_height: 0,
                   viewport_offset: Point::new(0, 0),
                   frame: 0,
               })
        }

        pub fn blit(&mut self, screen: Arc<Mutex<Screen>>) {
            if self.viewport_width == 0 {
                self.update_dimensions();
            }

            // Translate the pixel values to RGB colors.
            let src_buffer = screen.lock().unwrap().back_buffer;
            let mut rgb_buffer = *screen.lock().unwrap().buffer_rgb;
            let mut palette = px8::PALETTE.lock().unwrap();

            let mut j = 0;
            let mut cached_pixel: u8 = 0;
            let mut rgb = palette.get_rgb(cached_pixel as u32);

            let start = PreciseTime::now();

            for pixel in src_buffer.iter() {
                if *pixel != cached_pixel {
                    rgb = palette.get_rgb(*pixel as u32);
                    cached_pixel = *pixel;
                }
                rgb_buffer[j] = rgb.b;
                rgb_buffer[j + 1] = rgb.g;
                rgb_buffer[j + 2] = rgb.r;
                j = j + 3;
            }

            let t1 = PreciseTime::now();

            // Update the texture with the RGB values.
            self.texture
                .update(None, &mut rgb_buffer, px8::SCREEN_WIDTH * 3)
                .unwrap();

            let t2 = PreciseTime::now();

            // Copy the texture to the screen.
            // Only need to clear the screen if there is any border -
            // the main display area will be overwritten.
            if self.viewport_offset.x() != 0 || self.viewport_offset.y() != 0 {
                self.renderer.clear();
            }

            self.renderer
                .copy(&self.texture,
                      Some(Rect::new(0, 0, px8::SCREEN_WIDTH as u32, px8::SCREEN_HEIGHT as u32)),
                      Some(Rect::new(self.viewport_offset.x(),
                                     self.viewport_offset.y(),
                                     self.viewport_width,
                                     self.viewport_height)))
                .unwrap();

            let t3 = PreciseTime::now();

            self.renderer.present();

            let t4 = PreciseTime::now();

            if cfg!(feature = "blit_perf") {
                if self.frame % 60 == 0 {
                    println!("gen_rgb:{} update_tex:{} copy_tex:{} present:{}",
                             start.to(t1),
                             t1.to(t2),
                             t2.to(t3),
                             t3.to(t4))
                }
            }

            self.frame += 1;
        }

        pub fn update_dimensions(&mut self) {
            let (w, h) = self.get_dimensions();
            self.window_width = w;
            self.window_height = h;

            let window_aspect_ratio = (w as f32) / (h as f32);
            let viewport_aspect_ratio = (px8::SCREEN_WIDTH as f32) / (px8::SCREEN_HEIGHT as f32);

            self.viewport_offset = if viewport_aspect_ratio > window_aspect_ratio {
                // Need margin at top and bottom
                self.viewport_width = self.window_width;
                self.viewport_height = (self.viewport_width as f32 / viewport_aspect_ratio) as u32;
                Point::new(0, (self.window_height - self.viewport_height) as i32 / 2)
            } else {
                // Need margin at left and right
                self.viewport_height = self.window_height;
                self.viewport_width = (self.viewport_height as f32 * viewport_aspect_ratio) as u32;
                Point::new((self.window_width - self.viewport_width) as i32 / 2, 0)
            };
        }

        pub fn get_dimensions(&mut self) -> (u32, u32) {
            self.renderer.window().unwrap().size()
        }

        pub fn window_coords_to_viewport_coords(&mut self,
                                                window_x: i32,
                                                window_y: i32)
                                                -> (i32, i32) {
            let viewport_x = ((window_x - self.viewport_offset.x()) as f32 /
                              self.viewport_width as f32 *
                              px8::SCREEN_WIDTH as f32) as i32;
            let viewport_y = ((window_y - self.viewport_offset.y()) as f32 /
                              self.viewport_height as f32 *
                              px8::SCREEN_HEIGHT as f32) as i32;

            (num::clamp(viewport_x, 0, (px8::SCREEN_WIDTH - 1) as i32),
             num::clamp(viewport_y, 0, (px8::SCREEN_HEIGHT - 1) as i32))
        }
    }
}
