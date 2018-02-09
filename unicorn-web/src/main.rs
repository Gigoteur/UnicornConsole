// From https://github.com/koute/pinky/blob/master/pinky-web Thx !!
#![recursion_limit="2048"]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate stdweb;
extern crate unicorn;

#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::DerefMut;
use std::cmp::max;
use std::error::Error;

//lazy_static! {
//    pub static ref FRAMEBUFFER: Mutex<[u32; 400 * 240]> = {
//        Mutex::new([0; 400 * 240])
//    };
//}

static mut FRAMEBUFFER: [u32; 400 * 240] = [0; 400 * 240];

use stdweb::web::{
    self,
    IEventTarget,
    INode,
    IElement,
    FileReader,
    FileReaderResult,
    Element,
    ArrayBuffer
};

use stdweb::web::event::{
    IEvent,
    IKeyboardEvent,
    ClickEvent,
    ChangeEvent,
    ProgressLoadEvent,
    KeydownEvent,
    KeyupEvent,
    KeyboardLocation
};

use stdweb::web::html_element::InputElement;
use stdweb::unstable::TryInto;
use stdweb::{Value, UnsafeTypedArray, Once};

macro_rules! enclose {
    ( [$( $x:ident ),*] $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}


struct UnicornWeb {
    state: unicorn::unicorn::Unicorn,
    //framebuffer: [u32; 400 * 240],
    audio_buffer: Vec< f32 >,
    audio_chunk_counter: u32,
    audio_underrun: Option< usize >,
    paused: bool,
    busy: bool,
    js_ctx: Value
}

// This creates a really basic WebGL context for blitting a single texture.
// On some web browsers this is faster than using a 2d canvas.
fn setup_webgl( canvas: &Element ) -> Value {
    const FRAGMENT_SHADER: &'static str = r#"
        precision mediump float;
        varying vec2 v_texcoord;
        uniform sampler2D u_sampler;
        void main() {
            gl_FragColor = vec4( texture2D( u_sampler, vec2( v_texcoord.s, v_texcoord.t ) ).rgb, 1.0 );
        }
    "#;

    const VERTEX_SHADER: &'static str = r#"
        attribute vec2 a_position;
        attribute vec2 a_texcoord;
        uniform mat4 u_matrix;
        varying vec2 v_texcoord;
        void main() {
            gl_Position = u_matrix * vec4( a_position, 0.0, 1.0 );
            v_texcoord = a_texcoord;
        }
    "#;

    fn ortho( left: f64, right: f64, bottom: f64, top: f64 ) -> Vec< f64 > {
        let mut m = vec![ 1.0, 0.0, 0.0, 0.0,
                          0.0, 1.0, 0.0, 0.0,
                          0.0, 0.0, 1.0, 0.0,
                          0.0, 0.0, 0.0, 1.0 ];

        m[ 0 * 4 + 0 ] = 2.0 / (right - left);
        m[ 1 * 4 + 1 ] = 2.0 / (top - bottom);
        m[ 3 * 4 + 0 ] = (right + left) / (right - left) * -1.0;
        m[ 3 * 4 + 1 ] = (top + bottom) / (top - bottom) * -1.0;

        return m;
    }

    js!(
        var gl;
        var webgl_names = ["webgl", "experimental-webgl", "webkit-3d", "moz-webgl"];
        for( var i = 0; i < webgl_names.length; ++i ) {
            var name = webgl_names[ i ];
            try {
                gl = @{canvas}.getContext( name );
            } catch( err ) {}

            if( gl ) {
                console.log( "WebGL support using context:", name );
                break;
            }
        }

        var vertex_shader = gl.createShader( gl.VERTEX_SHADER );
        var fragment_shader = gl.createShader( gl.FRAGMENT_SHADER );
        gl.shaderSource( vertex_shader, @{VERTEX_SHADER} );
        gl.shaderSource( fragment_shader, @{FRAGMENT_SHADER} );
        gl.compileShader( vertex_shader );
        gl.compileShader( fragment_shader );

        if( !gl.getShaderParameter( vertex_shader, gl.COMPILE_STATUS ) ) {
            console.error( "WebGL vertex shader compilation failed:", gl.getShaderInfoLog( vertex_shader ) );
            return null;
        }

        if( !gl.getShaderParameter( fragment_shader, gl.COMPILE_STATUS ) ) {
            console.error( "WebGL fragment shader compilation failed:", gl.getShaderInfoLog( fragment_shader ) );
            return null;
        }

        var program = gl.createProgram();
        gl.attachShader( program, vertex_shader );
        gl.attachShader( program, fragment_shader );
        gl.linkProgram( program );
        if( !gl.getProgramParameter( program, gl.LINK_STATUS ) ) {
            console.error( "WebGL program linking failed!" );
            return null;
        }

        gl.useProgram( program );

        var vertex_attr = gl.getAttribLocation( program, "a_position" );
        var texcoord_attr = gl.getAttribLocation( program, "a_texcoord" );

        gl.enableVertexAttribArray( vertex_attr );
        gl.enableVertexAttribArray( texcoord_attr );

        var sampler_uniform = gl.getUniformLocation( program, "u_sampler" );
        gl.uniform1i( sampler_uniform, 0 );

        var matrix = @{ortho( 0.0, 400.0, 240.0, 0.0 )};
        var matrix_uniform = gl.getUniformLocation( program, "u_matrix" );
        gl.uniformMatrix4fv( matrix_uniform, false, matrix );

        var texture = gl.createTexture();
        gl.bindTexture( gl.TEXTURE_2D, texture );
        gl.texImage2D( gl.TEXTURE_2D, 0, gl.RGBA, 512, 512, 0, gl.RGBA, gl.UNSIGNED_BYTE, new Uint8Array( 512 * 512 * 4 ) );
        gl.texParameteri( gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST );
        gl.texParameteri( gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST );

        var vertex_buffer = gl.createBuffer();
        gl.bindBuffer( gl.ARRAY_BUFFER, vertex_buffer );
        var vertices = [
            0.0, 0.0,
            0.0, 240.0,
            400.0, 0.0,
            400.0, 240.0
        ];
        gl.bufferData( gl.ARRAY_BUFFER, new Float32Array( vertices ), gl.STATIC_DRAW );
        gl.vertexAttribPointer( vertex_attr, 2, gl.FLOAT, false, 0, 0 );

        var texcoord_buffer = gl.createBuffer();
        gl.bindBuffer( gl.ARRAY_BUFFER, texcoord_buffer );
        var texcoords = [
            0.0, 0.0,
            0.0, 240.0 / 400.0,
            1.0, 0.0,
            1.0, 240.0 / 400.0
        ];
        gl.bufferData( gl.ARRAY_BUFFER, new Float32Array( texcoords ), gl.STATIC_DRAW );
        gl.vertexAttribPointer( texcoord_attr, 2, gl.FLOAT, false, 0, 0 );

        var index_buffer = gl.createBuffer();
        gl.bindBuffer( gl.ELEMENT_ARRAY_BUFFER, index_buffer );
        var indices = [
            0, 1, 2,
            2, 3, 1
        ];
        gl.bufferData( gl.ELEMENT_ARRAY_BUFFER, new Uint16Array( indices ), gl.STATIC_DRAW );

        gl.clearColor( 0.0, 0.0, 0.0, 1.0 );
        gl.enable( gl.DEPTH_TEST );
        gl.viewport( 0, 0, 512, 512 );

        return gl;
    )
}

impl UnicornWeb {
    fn new( canvas: &Element ) -> Self {
        let gl = setup_webgl( &canvas );

        let js_ctx = js!(
            var h = {};
            var canvas = @{canvas};

            h.gl = @{gl};
            h.audio = new AudioContext();
            h.empty_audio_buffers = [];
            h.play_timestamp = 0;

            if( !h.gl ) {
                console.log( "No WebGL; using Canvas API" );

                // If the WebGL **is** supported but something else
                // went wrong the web browser won't let us create
                // a normal canvas context on a WebGL-ified canvas,
                // so we recreate a new canvas here to work around that.
                var new_canvas = canvas.cloneNode( true );
                canvas.parentNode.replaceChild( new_canvas, canvas );
                canvas = new_canvas;

                h.ctx = canvas.getContext( "2d" );
                h.img = h.ctx.createImageData( 512, 512 );
                h.buffer = new Uint32Array( h.img.data.buffer );
            }

            return h;
        );

        UnicornWeb {
            state: unicorn::unicorn::Unicorn::new(),
            audio_buffer: Vec::with_capacity( 44100 ),
            audio_chunk_counter: 0,
            audio_underrun: None,
            paused: false,
            busy: false,
            js_ctx
        }
    }

    fn pause( &mut self ) {
        self.paused = true;
    }

    fn unpause( &mut self ) {
        self.paused = false;
        self.busy = false;
    }

    // This will run the emulator either until we've finished
    // a frame, or until we've generated one audio chunk,
    // in which case we'll temporairly give back the control
    // of the main thread back to the web browser so that
    // it can handle other events and process audio.
    fn run_a_bit( &mut self ) -> Result< bool, Box< Error > > {
        if self.paused {
            return Ok(true);
        }

        self.state.update();
        self.state.draw();
        //self.state.update_sound();

        /*let audio_chunk_counter = self.audio_chunk_counter;
        loop {
            let result = nes::Interface::execute_cycle( self );
            match result {
                Ok( processed_whole_frame ) => {
                    if processed_whole_frame {
                        return Ok( true );
                    } else if self.audio_chunk_counter != audio_chunk_counter {
                        return Ok( false );
                    }
                },
                Err( error ) => {
                    js!( console.error( "Execution error:", @{format!( "{}", error )} ); );
                    self.pause();

                    return Err( error );
                }
            }
        }*/

        Ok(true)
    }

    fn draw( &mut self ) {
        let mut palette = unicorn::unicorn::PALETTE.lock().unwrap();
        let framebuffer = &self.state.screen.lock().unwrap().frame_buffer;

        let mut i = 0;
        for pixel_in in framebuffer.iter() {
            let rgb = palette.get_rgb(*pixel_in as u32);
            unsafe {
                FRAMEBUFFER[i] = ((rgb.r as u32) << 16) | ((rgb.g as u32) << 8) | ((rgb.b as u32));
            }
            i += 1;
        }

        if !self.paused {
           // for (pixel_in, pixel_out) in framebuffer.iter().zip( self.framebuffer.iter_mut() ) {
           //     *pixel_out = palette[ pixel_in.color_in_system_palette_index() as usize ];
           // }
        }

        js! {
            var h = @{&self.js_ctx};
            var framebuffer = @{unsafe { UnsafeTypedArray::new( &FRAMEBUFFER ) }};
            if( h.gl ) {
                var data = new Uint8Array( framebuffer.buffer, framebuffer.byteOffset, framebuffer.byteLength );
                h.gl.texSubImage2D( h.gl.TEXTURE_2D, 0, 0, 0, 400, 240, h.gl.RGBA, h.gl.UNSIGNED_BYTE, data );
                h.gl.drawElements( h.gl.TRIANGLES, 6, h.gl.UNSIGNED_SHORT, 0 );
            } else {
                h.buffer.set( framebuffer );
                h.ctx.putImageData( h.img, 0, 0 );
            }
        }
    }

    fn on_key( &mut self, key: &str, location: KeyboardLocation, is_pressed: bool ) -> bool {
       /* let button = match (key, location) {
            ("Enter", _) => nes::Button::Start,
            ("Shift", KeyboardLocation::Right) => nes::Button::Select,
            ("ArrowUp", _) => nes::Button::Up,
            ("ArrowLeft", _) => nes::Button::Left,
            ("ArrowRight", _) => nes::Button::Right,
            ("ArrowDown", _) => nes::Button::Down,

            // On Edge the arrows have different names
            // for some reason.
            ("Up", _) => nes::Button::Up,
            ("Left", _) => nes::Button::Left,
            ("Right", _) => nes::Button::Right,
            ("Down", _) => nes::Button::Down,

            ("z", _) => nes::Button::A,
            ("x", _) => nes::Button::B,

            // For those using the Dvorak layout.
            (";", _) => nes::Button::A,
            ("q", _) => nes::Button::B,

            // For those using the Dvorak layout **and** Microsoft Edge.
            //
            // On `keydown` we get ";" as we should, but on `keyup`
            // we get "Unidentified". Seriously Microsoft, how buggy can
            // your browser be?
            ("Unidentified", _) if is_pressed == false => nes::Button::A,

            _ => return false
        };

        nes::Interface::set_button_state( self, nes::ControllerPort::First, button, is_pressed );*/

        return true;
    }
}

fn emulate_for_a_single_frame( uc: Rc< RefCell< UnicornWeb > > ) {
    uc.borrow_mut().busy = true;

    uc.borrow_mut().run_a_bit();

    uc.borrow_mut().busy = false;

    /*web::set_timeout( enclose!( [uc] move || {
        let finished_frame = match uc.borrow_mut().run_a_bit() {
            Ok( result ) => result,
            Err( error ) => {
                handle_error( error );
                return;
            }
        };

        if !finished_frame {
            web::set_timeout( move || { emulate_for_a_single_frame( uc ); }, 0 );
        } else {
            let mut uc = uc.borrow_mut();
            if let Some( count ) = uc.audio_underrun.take() {
                for _ in 0..count {
                    if let Err( error ) = uc.run_a_bit() {
                        handle_error( error );
                        return;
                    }
                }
            }

            uc.busy = false;
        }
    }), 0 );*/
}

fn main_loop( uc: Rc< RefCell< UnicornWeb > > ) {
    // If we're running too slowly there is no point
    // in queueing up even more work.
    if !uc.borrow_mut().busy {
        emulate_for_a_single_frame( uc.clone() );
    }

    uc.borrow_mut().draw();

    web::window().request_animation_frame( move |_| {
        main_loop( uc );
    });
}

#[derive(Deserialize)]
struct RomEntry {
    name: String,
    file: String
}

js_deserializable!( RomEntry );

fn show( id: &str ) {
    web::document().get_element_by_id( id ).unwrap().class_list().remove( "hidden" );
}

fn hide( id: &str ) {
    web::document().get_element_by_id( id ).unwrap().class_list().add( "hidden" );
}


fn support_input( uc: Rc< RefCell< UnicornWeb > > ) {
    web::window().add_event_listener( enclose!( [uc] move |event: KeydownEvent| {
        let handled = uc.borrow_mut().on_key( &event.key(), event.location(), true );
        if handled {
            event.prevent_default();
        }
    }));

    web::window().add_event_listener( enclose!( [uc] move |event: KeyupEvent| {
        let handled = uc.borrow_mut().on_key( &event.key(), event.location(), false );
        if handled {
            event.prevent_default();
        }
    }));
}

fn handle_error< E: Into< Box< Error > > >( error: E ) {
    let error_message = format!( "{}", error.into() );
    web::document().get_element_by_id( "error-description" ).unwrap().set_text_content( &error_message );

    hide( "viewport" );
    hide( "change-rom-button" );
    hide( "rom-menu-close" );
    show( "change-rom-menu" );
    show( "error" );
}

fn main() {
    stdweb::initialize();

    let canvas = web::document().get_element_by_id( "viewport" ).unwrap();
    let uc = Rc::new( RefCell::new( UnicornWeb::new( &canvas ) ) );

    uc.borrow_mut().state.setup();
    uc.borrow_mut().state.init();

    let data = include_bytes!("../../unicorn/sys/unicorn.uni");
    let data_final: Vec<u8> = unicorn::unicorn::array_to_vec(data);

    uc.borrow_mut().state.load_cartridge_raw("unicorn.uni", data_final, true);

    hide( "loading" );
    hide( "error" );

    show( "viewport" );

    support_input( uc.clone() );

    web::window().request_animation_frame( move |_| {
        main_loop( uc );
    });

    stdweb::event_loop();
}