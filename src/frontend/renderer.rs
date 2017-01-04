pub mod renderer {
    use px8;

    use glium;
    use glium::{DrawError, DrawParameters, IndexBuffer, Program, VertexBuffer, Surface};
    use glium::backend::Facade;
    use glium::index::PrimitiveType;
    use glium::program::ProgramChooserCreationError;
    use glium::texture::{ClientFormat, MipmapsOption, PixelValue, TextureCreationError, UncompressedFloatFormat};
    use glium::texture::pixel_buffer::PixelBuffer;
    use glium::texture::texture2d::Texture2d;

    use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
    use nalgebra::{Diagonal, Matrix4, Vector4};

    use frontend::{FrontendError, FrontendResult};

    type Texture = Texture2d;

    #[derive(Copy, Clone)]
    pub struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords);


    unsafe impl PixelValue for px8::Color {
        fn get_format() -> ClientFormat {
            ClientFormat::U8
        }
    }

    impl From<DrawError> for FrontendError {
        fn from(e: DrawError) -> FrontendError {
            FrontendError::Renderer(format!("{:?}", e))
        }
    }

    impl From<glium::vertex::BufferCreationError> for FrontendError {
        fn from(e: glium::vertex::BufferCreationError) -> FrontendError {
            FrontendError::Renderer(format!("{:?}", e))
        }
    }

    impl From<glium::index::BufferCreationError> for FrontendError {
        fn from(e: glium::index::BufferCreationError) -> FrontendError {
            FrontendError::Renderer(format!("{:?}", e))
        }
    }

    impl From<ProgramChooserCreationError> for FrontendError {
        fn from(e: ProgramChooserCreationError) -> FrontendError {
            FrontendError::Renderer(format!("{:?}", e))
        }
    }

    impl From<TextureCreationError> for FrontendError {
        fn from(e: TextureCreationError) -> FrontendError {
            FrontendError::Renderer(format!("{:?}", e))
        }
    }

    pub struct Renderer {
        vertex_buffer: VertexBuffer<Vertex>,
        index_buffer: IndexBuffer<u16>,
        pixel_buffer: PixelBuffer<px8::Color>,
        program: Program,
        texture: Texture,
        matrix: Matrix4<f32>,
    }

    const TEXTURE_WIDTH: u32 = 256;
    const TEXTURE_HEIGHT: u32 = 256;
    const TEX_OFFSET_X: f32 = px8::SCREEN_WIDTH as f32 / TEXTURE_WIDTH as f32;
    const TEX_OFFSET_Y: f32 = px8::SCREEN_HEIGHT as f32 / TEXTURE_HEIGHT as f32;

    fn upload_pixels(texture: &mut Texture, pixel_buffer: &PixelBuffer<px8::Color>) {
        texture.main_level().raw_upload_from_pixel_buffer(pixel_buffer.as_slice(),
                                                          0..px8::SCREEN_WIDTH as u32,
                                                          0..px8::SCREEN_HEIGHT as u32,
                                                          0..1);
    }

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
        pub fn new<F: Facade>(display: &F) -> FrontendResult<Renderer> {
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

            let vertex_buffer = try!(VertexBuffer::immutable(display, &vertexes));

            info!("Creating IndexBuffer");

            let index_buffer =
            try!(IndexBuffer::immutable(display, PrimitiveType::TriangleStrip, &[1u16, 2, 0, 3]));

            info!("Compiling shader");

            let program = try!(program!(
              display,
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

            let pixel_buffer = PixelBuffer::new_empty(display, px8::SCREEN_WIDTH * px8::SCREEN_HEIGHT);
            pixel_buffer.write(&vec![px8::Color::Black; pixel_buffer.get_size()]);

            info!("Creating Texture");
            let mut texture = try!(Texture::empty_with_format(display,
                                                              UncompressedFloatFormat::U8U8U8,
                                                              MipmapsOption::NoMipmap,
                                                              TEXTURE_WIDTH,
                                                              TEXTURE_HEIGHT));


            info!("Uploading Pixels");
            upload_pixels(&mut texture, &pixel_buffer);

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
            })
        }

        pub fn draw<S: Surface>(&self, frame: &mut S) -> FrontendResult<()> {
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

        pub fn update_dimensions<F: Facade>(&mut self, display: &F) {
            let (width, height) = display.get_context().get_framebuffer_dimensions();
            let (x_scale, y_scale) = aspect_ratio_correction(width, height);
            self.matrix.m11 = x_scale;
            self.matrix.m22 = y_scale;
        }

        pub fn update_pixels(&mut self, pixels: &px8::ScreenBuffer) {
            self.pixel_buffer.write(pixels);
            upload_pixels(&mut self.texture, &self.pixel_buffer);
        }

        pub fn get_dimensions<F: Facade>(&mut self, display: &F) -> (u32, u32) {
            return display.get_context().get_framebuffer_dimensions();
        }
    }
}