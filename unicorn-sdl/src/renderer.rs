pub mod renderer {
    use sdl2::video::{Window};
    use unicorn;
    use unicorn::gfx::{Scale, Screen};

    use sdl2::VideoSubsystem;
    use sdl2::render;
    use sdl2::pixels::PixelFormatEnum;
    use time::Instant;

    #[derive(Clone, Debug)]
    pub enum RendererError {
        //        Sdl(String),
        //        Renderer(String),
        //        Other(String),
    }

    pub type RendererResult<T> = Result<T, RendererError>;

    pub struct Renderer {
        pub canvas: render::Canvas<Window>,
        pub texture: render::Texture,
        buffer_rgb: Vec<u8>,
        frame: u32,
    }

    impl Renderer {
        pub fn new(sdl_video: VideoSubsystem,
                   screen: &mut Screen,
                   fullscreen: bool,
                   opengl: bool,
                   scale: Scale)
                   -> RendererResult<Renderer> {
            info!("[SDL] Creating window fullscreen={:?} opengl={:?}",
                  fullscreen,
                  opengl);

            let mut window_builder = sdl_video.window("Unicorn Console",
                        (screen.width as usize * scale.factor()) as u32,
                        (screen.height as usize * scale.factor()) as u32);

            let wb = if fullscreen {
                window_builder.fullscreen()
            } else {
                window_builder.resizable().position_centered()
            };

            let window = (if opengl { wb.opengl() } else { wb }).build().unwrap();

            if !cfg!(target_os = "android") {
            //    let temp_surface = Surface::load_bmp(Path::new("unicorn_logo_alpha.bmp")).unwrap();
            //    window.set_icon(temp_surface);
            }

            info!("[SDL] Creating renderer");
            let canvas = window.into_canvas()
                .accelerated()
                .present_vsync()
                .build()
                .unwrap();

            let texture_creator = canvas.texture_creator();


            info!("[SDL] Creating texture");
            let texture_width = screen.width as u32;
            let texture_height = screen.height as u32;
            let texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24,
                                texture_width,
                                texture_height)
                .unwrap();


            Ok(Renderer {
                canvas: canvas,
                texture: texture,
                buffer_rgb: vec![0; 0],
                frame: 0,
            })
        }

        pub fn blit(&mut self, screen: &mut Screen) {
            // Translate the pixel values to RGB colors.
            let src_buffer = &screen.frame_buffer;
            let rgb_buffer_len = src_buffer.len() * 3;
            if self.buffer_rgb.len() != rgb_buffer_len {
                self.buffer_rgb = vec![0; rgb_buffer_len];
            }
            let rgb_buffer = &mut self.buffer_rgb;
            let mut palette = unicorn::unicorn::PALETTE.lock().unwrap();

            let mut j = 0;
            let mut cached_pixel: u32 = 0;
            let mut rgb = palette.get_rgb(cached_pixel as u32);

            let start = Instant::now();

            for pixel in src_buffer.iter() {
                if *pixel != cached_pixel {
                    rgb = palette.get_rgb(*pixel as u32);
                    cached_pixel = *pixel;
                }
                rgb_buffer[j] = rgb.r;
                rgb_buffer[j + 1] = rgb.g;
                rgb_buffer[j + 2] = rgb.b;
                j += 3;
            }

            let t1 = Instant::now();

            // Update the texture with the RGB values.
            self.texture
                .update(None, &rgb_buffer, screen.width * 3)
                .unwrap();

            let t2 = Instant::now();

            self.canvas
                .copy(&self.texture, None, None)
                .unwrap();

            let t3 = Instant::now();

            self.canvas.present();

            let t4 = Instant::now();

            if cfg!(feature = "blit_perf") {
                if self.frame % 60 == 0 {
                    info!("gen_rgb:{} update_tex:{} copy_tex:{} present:{}",
                          t1 - start,
                          t2 - t1,
                          t3 - t2,
                          t4 - t3)
                }
            }

            self.frame += 1;
        }

        pub fn get_dimensions(&mut self) -> (u32, u32) {
            self.canvas.window().size()
        }
    }
}