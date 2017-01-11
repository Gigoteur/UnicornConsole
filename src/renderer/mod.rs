#[cfg(not(feature = "sdl_renderer"))]
pub mod renderer {
    pub mod glium_sdl2;

    use px8;
    use gfx::{Scale};

    use sdl2::video::gl_attr::GLAttr;
    use sdl2::VideoSubsystem;

    use glium;
    use glium::{DrawError, DrawParameters, IndexBuffer, Program, VertexBuffer, Surface};
    use glium::index::PrimitiveType;
    use glium::program::ProgramChooserCreationError;
    use glium::{Api, GliumCreationError, SwapBuffersError, Version};
    use glium::backend::Facade;
    use glium::texture::{ClientFormat, MipmapsOption, PixelValue, TextureCreationError, UncompressedFloatFormat};
    use glium::texture::pixel_buffer::PixelBuffer;
    use glium::texture::texture2d::Texture2d;

    use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
    use nalgebra::{Diagonal, Matrix4, Vector4};

    use self::glium_sdl2::{Display, DisplayBuild, GliumSdl2Error};

    #[derive(Clone, Debug)]
    pub enum RendererError {
        Sdl(String),
        Renderer(String),
        Other(String),
    }

    pub type RendererResult<T> = Result<T, RendererError>;

    impl From<DrawError> for RendererError {
        fn from(e: DrawError) -> RendererError {
            RendererError::Renderer(format!("{:?}", e))
        }
    }

    impl From<TextureCreationError> for RendererError {
        fn from(e: TextureCreationError) -> RendererError {
            RendererError::Renderer(format!("{:?}", e))
        }
    }


    impl From<glium::vertex::BufferCreationError> for RendererError {
        fn from(e: glium::vertex::BufferCreationError) -> RendererError {
            RendererError::Renderer(format!("{:?}", e))
        }
    }

    impl From<glium::index::BufferCreationError> for RendererError {
        fn from(e: glium::index::BufferCreationError) -> RendererError {
            RendererError::Renderer(format!("{:?}", e))
        }
    }

    impl From<ProgramChooserCreationError> for RendererError {
        fn from(e: ProgramChooserCreationError) -> RendererError {
            RendererError::Renderer(format!("{:?}", e))
        }
    }

    impl From<GliumCreationError<GliumSdl2Error>> for RendererError {
        fn from(e: GliumCreationError<GliumSdl2Error>) -> RendererError {
            RendererError::Renderer(format!("{:?}", e))
        }
    }

    impl From<SwapBuffersError> for RendererError {
        fn from(e: SwapBuffersError) -> RendererError {
            RendererError::Renderer(format!("{:?}", e))
        }
    }


    unsafe impl PixelValue for px8::Color {
        fn get_format() -> ClientFormat {
            ClientFormat::U8
        }
    }


    type Texture = Texture2d;

    #[derive(Copy, Clone)]
    pub struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords);

    pub struct Renderer {
        vertex_buffer: VertexBuffer<Vertex>,
        index_buffer: IndexBuffer<u16>,
        pixel_buffer: PixelBuffer<px8::Color>,
        program: Program,
        texture: Texture,
        matrix: Matrix4<f32>,
        display: Display,
    }

    const TEXTURE_WIDTH: u32 = 256;
    const TEXTURE_HEIGHT: u32 = 256;
    const TEX_OFFSET_X: f32 = px8::SCREEN_WIDTH as f32 / TEXTURE_WIDTH as f32;
    const TEX_OFFSET_Y: f32 = px8::SCREEN_HEIGHT as f32 / TEXTURE_HEIGHT as f32;

    const ASPECT_RATIO: f32 = px8::SCREEN_WIDTH as f32 / px8::SCREEN_HEIGHT as f32;

    fn aspect_ratio_correction(width: u32, height: u32) -> (f32, f32) {
        let fb_aspect_ratio = width as f32 / height as f32;
        let scale = ASPECT_RATIO / fb_aspect_ratio;
        if fb_aspect_ratio >= ASPECT_RATIO {
            (scale, 1.0)
        } else {
            (1.0, 1.0 / scale)
        }
    }

    impl Renderer {

        pub fn new(sdl_video: VideoSubsystem, fullscreen: bool, scale: Scale) -> RendererResult<Renderer> {
            let display;

            info!("SDL2 Video with opengl [glium]");

            configure_gl_attr(&mut sdl_video.gl_attr());

            if fullscreen {
                info!("SDL2 window fullscreen");

                display = try!(sdl_video.window("PX8",
                                                (px8::SCREEN_WIDTH * scale.factor()) as u32,
                                                (px8::SCREEN_HEIGHT * scale.factor()) as u32)
                    .resizable()
                    .fullscreen()
                    .position_centered()
                    .build_glium());
            } else {
                info!("SDL2 window");

                display = try!(sdl_video.window("PX8",
                                                (px8::SCREEN_WIDTH * scale.factor()) as u32,
                                                (px8::SCREEN_HEIGHT * scale.factor()) as u32)
                    .resizable()
                    .position_centered()
                    .build_glium());
            }

            info!("Init Renderer with GLIUM");

            let vertexes = [
                Vertex {
                    position: [-1.0, -1.0],
                    tex_coords: [0.0, TEX_OFFSET_Y],
                },
                Vertex {
                    position: [-1.0, 1.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [1.0, 1.0],
                    tex_coords: [TEX_OFFSET_X, 0.0],
                },
                Vertex {
                    position: [1.0, -1.0],
                    tex_coords: [TEX_OFFSET_X, TEX_OFFSET_Y],
                }
            ];

            info!("Creating VertexBuffer");

            let vertex_buffer = try!(VertexBuffer::immutable(&display, &vertexes));

            info!("Creating IndexBuffer");

            let index_buffer =
            try!(IndexBuffer::immutable(&display, PrimitiveType::TriangleStrip, &[1u16, 2, 0, 3]));

            info!("Compiling shader");

            let program = try!(program!(
              &display,
              140 => {
                vertex: include_str!("shader/vert_140.glsl"),
                fragment: include_str!("shader/frag_140.glsl"),
                outputs_srgb: true
              },
              110 => {
                vertex: include_str!("shader/vert_110.glsl"),
                fragment: include_str!("shader/frag_110.glsl"),
                outputs_srgb: true
              }
            ));

            info!("Creating PixelBuffer");

            let pixel_buffer = PixelBuffer::new_empty(&display, px8::SCREEN_WIDTH * px8::SCREEN_HEIGHT);
            pixel_buffer.write(&vec![px8::Color::Black; pixel_buffer.get_size()]);

            info!("Creating Texture");
            let mut texture = try!(Texture::empty_with_format(&display,
                                                              UncompressedFloatFormat::U8U8U8,
                                                              MipmapsOption::NoMipmap,
                                                              TEXTURE_WIDTH,
                                                              TEXTURE_HEIGHT));


            info!("Uploading Pixels");
            texture.main_level().raw_upload_from_pixel_buffer(pixel_buffer.as_slice(),
                                                              0..px8::SCREEN_WIDTH as u32,
                                                              0..px8::SCREEN_HEIGHT as u32,
                                                              0..1);

            let (width, height) = display.get_context().get_framebuffer_dimensions();
            let (x_scale, y_scale) = aspect_ratio_correction(width, height);
            let matrix = Matrix4::from_diagonal(&Vector4::new(x_scale, y_scale, 1.0, 1.0));

            Ok(Renderer {
                vertex_buffer: vertex_buffer,
                index_buffer: index_buffer,
                pixel_buffer: pixel_buffer,
                program: program,
                texture: texture,
                matrix: matrix,
                display: display,
            })

        }
        pub fn draw<S: Surface>(&self, frame: &mut S) -> RendererResult<()> {
            let uniforms = uniform! {
                matrix: self.matrix.as_ref().clone(),
                tex: self.texture.sampled()
                    .minify_filter(MinifySamplerFilter::Nearest)
                    .magnify_filter(MagnifySamplerFilter::Nearest)
            };

            let params = DrawParameters { ..Default::default() };
            try!(frame.draw(&self.vertex_buffer,
                            &self.index_buffer,
                            &self.program,
                            &uniforms,
                            &params));
            Ok(())
        }

        pub fn blit(&mut self, back_buffer: &px8::ScreenBuffer) {
            self.update_pixels(back_buffer);

            let mut target = self.display.draw();
            target.clear_color(0.0, 0.0, 0.0, 0.0);
            self.draw(&mut target);
            target.finish();
        }


        pub fn update_pixels(&mut self, pixels: &px8::ScreenBuffer) {
            self.pixel_buffer.write(pixels);

            self.texture.main_level().raw_upload_from_pixel_buffer(self.pixel_buffer.as_slice(),
                                                                   0..px8::SCREEN_WIDTH as u32,
                                                                   0..px8::SCREEN_HEIGHT as u32,
                                                                   0..1);
        }

        pub fn update_dimensions(&mut self) {
            let (width, height) = self.display.get_context().get_framebuffer_dimensions();
            let (x_scale, y_scale) = aspect_ratio_correction(width, height);
            self.matrix.m11 = x_scale;
            self.matrix.m22 = y_scale;
        }

        pub fn get_dimensions(&mut self) -> (u32, u32) {
            return self.display.get_context().get_framebuffer_dimensions();
        }
    }

    #[cfg(target_os = "linux")]
    fn configure_gl_attr(gl_attr: &mut GLAttr) {
        info!("Init OPENGL for Linux");
    }

    #[cfg(target_os = "windows")]
    fn configure_gl_attr(gl_attr: &mut GLAttr) {
        info!("Init OPENGL for Windows");
    }

    #[cfg(target_os = "macos")]
    fn configure_gl_attr(gl_attr: &mut GLAttr) {
        info!("Init OPENGL for OSX");

        use sdl2::video::GLProfile;
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_flags().forward_compatible().set();
    }

    #[cfg(target_os = "emscripten")]
    fn configure_gl_attr(gl_attr: &mut GLAttr) {
        info!("Init OPENGL for Emscripten");
    }
}

#[cfg(feature = "sdl_renderer")]
pub mod renderer {
    use px8;
    use gfx::{Scale};

    use sdl2::VideoSubsystem;
    use sdl2::render;
    use sdl2::pixels::PixelFormatEnum;

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
        pub fn new(sdl_video: VideoSubsystem, fullscreen: bool, scale: Scale) -> RendererResult<Renderer> {
            info!("[SDL] Creating window");

            let mut window_builder = sdl_video.window("PX8",
                                                      (px8::SCREEN_WIDTH as usize * scale.factor()) as u32,
                                                      (px8::SCREEN_HEIGHT as usize * scale.factor()) as u32);
            let window = window_builder.position_centered().build().unwrap();

            info!("[SDL] Creating renderer");
            let mut renderer = window.renderer().accelerated().present_vsync().build().unwrap();

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

        pub fn blit(&mut self, back_buffer: &px8::ScreenBuffer) {
            let mut data = Box::new([0; px8::SCREEN_HEIGHT*px8::SCREEN_WIDTH*3]);

            for x in 0..px8::SCREEN_WIDTH {
                for y in 0..px8::SCREEN_HEIGHT {
                    let col_rgb = px8::Color::to_rgb(back_buffer[x + y * px8::SCREEN_WIDTH]);

                    data[(x+y*px8::SCREEN_WIDTH)*3] = col_rgb.b;
                    data[(x+y*px8::SCREEN_WIDTH)*3+1] = col_rgb.g;
                    data[(x+y*px8::SCREEN_WIDTH)*3+2] = col_rgb.r;
                }
            }

            self.texture.update(None,
                                &mut *data,
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