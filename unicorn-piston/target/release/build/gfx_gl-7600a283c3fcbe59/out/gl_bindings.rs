
        mod __gl_imports {
            pub use std::mem;
            pub use std::marker::Send;
            pub use std::os::raw;
        }
    

        pub mod types {
            #![allow(non_camel_case_types, non_snake_case, dead_code, missing_copy_implementations)]
    
// Common types from OpenGL 1.1
pub type GLenum = super::__gl_imports::raw::c_uint;
pub type GLboolean = super::__gl_imports::raw::c_uchar;
pub type GLbitfield = super::__gl_imports::raw::c_uint;
pub type GLvoid = super::__gl_imports::raw::c_void;
pub type GLbyte = super::__gl_imports::raw::c_char;
pub type GLshort = super::__gl_imports::raw::c_short;
pub type GLint = super::__gl_imports::raw::c_int;
pub type GLclampx = super::__gl_imports::raw::c_int;
pub type GLubyte = super::__gl_imports::raw::c_uchar;
pub type GLushort = super::__gl_imports::raw::c_ushort;
pub type GLuint = super::__gl_imports::raw::c_uint;
pub type GLsizei = super::__gl_imports::raw::c_int;
pub type GLfloat = super::__gl_imports::raw::c_float;
pub type GLclampf = super::__gl_imports::raw::c_float;
pub type GLdouble = super::__gl_imports::raw::c_double;
pub type GLclampd = super::__gl_imports::raw::c_double;
pub type GLeglImageOES = *const super::__gl_imports::raw::c_void;
pub type GLchar = super::__gl_imports::raw::c_char;
pub type GLcharARB = super::__gl_imports::raw::c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const super::__gl_imports::raw::c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = super::__gl_imports::raw::c_uint;

pub type GLhalfARB = super::__gl_imports::raw::c_ushort;
pub type GLhalf = super::__gl_imports::raw::c_ushort;

// Must be 32 bits
pub type GLfixed = GLint;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub enum __GLsync {}
pub type GLsync = *const __GLsync;

// compatible with OpenCL cl_context
pub enum _cl_context {}
pub enum _cl_event {}

pub type GLDEBUGPROC = extern "system" fn(source: GLenum,
                                          gltype: GLenum,
                                          id: GLuint,
                                          severity: GLenum,
                                          length: GLsizei,
                                          message: *const GLchar,
                                          userParam: *mut super::__gl_imports::raw::c_void);
pub type GLDEBUGPROCARB = extern "system" fn(source: GLenum,
                                             gltype: GLenum,
                                             id: GLuint,
                                             severity: GLenum,
                                             length: GLsizei,
                                             message: *const GLchar,
                                             userParam: *mut super::__gl_imports::raw::c_void);
pub type GLDEBUGPROCKHR = extern "system" fn(source: GLenum,
                                             gltype: GLenum,
                                             id: GLuint,
                                             severity: GLenum,
                                             length: GLsizei,
                                             message: *const GLchar,
                                             userParam: *mut super::__gl_imports::raw::c_void);

// GLES 1 types
// "pub type GLclampx = i32;",

// GLES 1/2 types (tagged for GLES 1)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 1/2 types (tagged for GLES 2 - attribute syntax is limited)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLint64EXT = i64;",
// "pub type GLuint64EXT = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 2 types (none currently)

// Vendor extension types
pub type GLDEBUGPROCAMD = extern "system" fn(id: GLuint,
                                             category: GLenum,
                                             severity: GLenum,
                                             length: GLsizei,
                                             message: *const GLchar,
                                             userParam: *mut super::__gl_imports::raw::c_void);
pub type GLhalfNV = super::__gl_imports::raw::c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;

}
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92D9;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_ATTRIBUTES: types::GLenum = 0x8B89;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: types::GLenum = 0x8B8A;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_PROGRAM: types::GLenum = 0x8259;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_RESOURCES: types::GLenum = 0x92F5;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_SUBROUTINES: types::GLenum = 0x8DE5;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_SUBROUTINE_MAX_LENGTH: types::GLenum = 0x8E48;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_SUBROUTINE_UNIFORMS: types::GLenum = 0x8DE6;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: types::GLenum = 0x8E47;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: types::GLenum = 0x8E49;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_TEXTURE: types::GLenum = 0x84E0;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORMS: types::GLenum = 0x8B86;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORM_BLOCKS: types::GLenum = 0x8A36;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: types::GLenum = 0x8A35;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORM_MAX_LENGTH: types::GLenum = 0x8B87;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_VARIABLES: types::GLenum = 0x9305;
#[allow(dead_code, non_upper_case_globals)] pub const ALIASED_LINE_WIDTH_RANGE: types::GLenum = 0x846E;
#[allow(dead_code, non_upper_case_globals)] pub const ALL_BARRIER_BITS: types::GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const ALL_SHADER_BITS: types::GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA: types::GLenum = 0x1906;
#[allow(dead_code, non_upper_case_globals)] pub const ALREADY_SIGNALED: types::GLenum = 0x911A;
#[allow(dead_code, non_upper_case_globals)] pub const ALWAYS: types::GLenum = 0x0207;
#[allow(dead_code, non_upper_case_globals)] pub const AND: types::GLenum = 0x1501;
#[allow(dead_code, non_upper_case_globals)] pub const AND_INVERTED: types::GLenum = 0x1504;
#[allow(dead_code, non_upper_case_globals)] pub const AND_REVERSE: types::GLenum = 0x1502;
#[allow(dead_code, non_upper_case_globals)] pub const ANY_SAMPLES_PASSED: types::GLenum = 0x8C2F;
#[allow(dead_code, non_upper_case_globals)] pub const ANY_SAMPLES_PASSED_CONSERVATIVE: types::GLenum = 0x8D6A;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_BUFFER: types::GLenum = 0x8892;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_BUFFER_BINDING: types::GLenum = 0x8894;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_SIZE: types::GLenum = 0x92FB;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_STRIDE: types::GLenum = 0x92FE;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BARRIER_BIT: types::GLenum = 0x00001000;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER: types::GLenum = 0x92C0;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: types::GLenum = 0x92C5;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: types::GLenum = 0x92C6;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_BINDING: types::GLenum = 0x92C1;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_DATA_SIZE: types::GLenum = 0x92C4;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_INDEX: types::GLenum = 0x9301;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: types::GLenum = 0x90ED;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: types::GLenum = 0x92CB;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: types::GLenum = 0x92CA;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: types::GLenum = 0x92C8;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: types::GLenum = 0x92C9;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: types::GLenum = 0x92C7;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_SIZE: types::GLenum = 0x92C3;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_START: types::GLenum = 0x92C2;
#[allow(dead_code, non_upper_case_globals)] pub const ATTACHED_SHADERS: types::GLenum = 0x8B85;
#[allow(dead_code, non_upper_case_globals)] pub const AUTO_GENERATE_MIPMAP: types::GLenum = 0x8295;
#[allow(dead_code, non_upper_case_globals)] pub const BACK: types::GLenum = 0x0405;
#[allow(dead_code, non_upper_case_globals)] pub const BACK_LEFT: types::GLenum = 0x0402;
#[allow(dead_code, non_upper_case_globals)] pub const BACK_RIGHT: types::GLenum = 0x0403;
#[allow(dead_code, non_upper_case_globals)] pub const BGR: types::GLenum = 0x80E0;
#[allow(dead_code, non_upper_case_globals)] pub const BGRA: types::GLenum = 0x80E1;
#[allow(dead_code, non_upper_case_globals)] pub const BGRA_INTEGER: types::GLenum = 0x8D9B;
#[allow(dead_code, non_upper_case_globals)] pub const BGR_INTEGER: types::GLenum = 0x8D9A;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND: types::GLenum = 0x0BE2;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_COLOR: types::GLenum = 0x8005;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST: types::GLenum = 0x0BE0;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST_ALPHA: types::GLenum = 0x80CA;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST_RGB: types::GLenum = 0x80C8;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION: types::GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION_ALPHA: types::GLenum = 0x883D;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION_RGB: types::GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC: types::GLenum = 0x0BE1;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC_ALPHA: types::GLenum = 0x80CB;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC_RGB: types::GLenum = 0x80C9;
#[allow(dead_code, non_upper_case_globals)] pub const BLOCK_INDEX: types::GLenum = 0x92FD;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE: types::GLenum = 0x1905;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE_INTEGER: types::GLenum = 0x8D96;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL: types::GLenum = 0x8B56;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC2: types::GLenum = 0x8B57;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC3: types::GLenum = 0x8B58;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC4: types::GLenum = 0x8B59;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER: types::GLenum = 0x82E0;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_ACCESS: types::GLenum = 0x88BB;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_ACCESS_FLAGS: types::GLenum = 0x911F;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_BINDING: types::GLenum = 0x9302;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_DATA_SIZE: types::GLenum = 0x9303;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_IMMUTABLE_STORAGE: types::GLenum = 0x821F;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAPPED: types::GLenum = 0x88BC;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAP_LENGTH: types::GLenum = 0x9120;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAP_OFFSET: types::GLenum = 0x9121;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAP_POINTER: types::GLenum = 0x88BD;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_SIZE: types::GLenum = 0x8764;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_STORAGE_FLAGS: types::GLenum = 0x8220;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_UPDATE_BARRIER_BIT: types::GLenum = 0x00000200;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_USAGE: types::GLenum = 0x8765;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_VARIABLE: types::GLenum = 0x92E5;
#[allow(dead_code, non_upper_case_globals)] pub const BYTE: types::GLenum = 0x1400;
#[allow(dead_code, non_upper_case_globals)] pub const CAVEAT_SUPPORT: types::GLenum = 0x82B8;
#[allow(dead_code, non_upper_case_globals)] pub const CCW: types::GLenum = 0x0901;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_READ_COLOR: types::GLenum = 0x891C;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_TO_BORDER: types::GLenum = 0x812D;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_TO_EDGE: types::GLenum = 0x812F;
#[allow(dead_code, non_upper_case_globals)] pub const CLEAR: types::GLenum = 0x1500;
#[allow(dead_code, non_upper_case_globals)] pub const CLEAR_BUFFER: types::GLenum = 0x82B4;
#[allow(dead_code, non_upper_case_globals)] pub const CLEAR_TEXTURE: types::GLenum = 0x9365;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_MAPPED_BUFFER_BARRIER_BIT: types::GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_STORAGE_BIT: types::GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DEPTH_MODE: types::GLenum = 0x935D;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE0: types::GLenum = 0x3000;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE1: types::GLenum = 0x3001;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE2: types::GLenum = 0x3002;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE3: types::GLenum = 0x3003;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE4: types::GLenum = 0x3004;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE5: types::GLenum = 0x3005;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE6: types::GLenum = 0x3006;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE7: types::GLenum = 0x3007;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_ORIGIN: types::GLenum = 0x935C;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR: types::GLenum = 0x1800;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT0: types::GLenum = 0x8CE0;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT1: types::GLenum = 0x8CE1;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT10: types::GLenum = 0x8CEA;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT11: types::GLenum = 0x8CEB;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT12: types::GLenum = 0x8CEC;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT13: types::GLenum = 0x8CED;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT14: types::GLenum = 0x8CEE;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT15: types::GLenum = 0x8CEF;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT16: types::GLenum = 0x8CF0;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT17: types::GLenum = 0x8CF1;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT18: types::GLenum = 0x8CF2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT19: types::GLenum = 0x8CF3;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT2: types::GLenum = 0x8CE2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT20: types::GLenum = 0x8CF4;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT21: types::GLenum = 0x8CF5;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT22: types::GLenum = 0x8CF6;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT23: types::GLenum = 0x8CF7;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT24: types::GLenum = 0x8CF8;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT25: types::GLenum = 0x8CF9;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT26: types::GLenum = 0x8CFA;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT27: types::GLenum = 0x8CFB;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT28: types::GLenum = 0x8CFC;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT29: types::GLenum = 0x8CFD;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT3: types::GLenum = 0x8CE3;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT30: types::GLenum = 0x8CFE;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT31: types::GLenum = 0x8CFF;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT4: types::GLenum = 0x8CE4;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT5: types::GLenum = 0x8CE5;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT6: types::GLenum = 0x8CE6;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT7: types::GLenum = 0x8CE7;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT8: types::GLenum = 0x8CE8;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT9: types::GLenum = 0x8CE9;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_BUFFER_BIT: types::GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_CLEAR_VALUE: types::GLenum = 0x0C22;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_COMPONENTS: types::GLenum = 0x8283;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ENCODING: types::GLenum = 0x8296;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_LOGIC_OP: types::GLenum = 0x0BF2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_RENDERABLE: types::GLenum = 0x8286;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_WRITEMASK: types::GLenum = 0x0C23;
#[allow(dead_code, non_upper_case_globals)] pub const COMMAND_BARRIER_BIT: types::GLenum = 0x00000040;
#[allow(dead_code, non_upper_case_globals)] pub const COMPARE_REF_TO_TEXTURE: types::GLenum = 0x884E;
#[allow(dead_code, non_upper_case_globals)] pub const COMPATIBLE_SUBROUTINES: types::GLenum = 0x8E4B;
#[allow(dead_code, non_upper_case_globals)] pub const COMPILE_STATUS: types::GLenum = 0x8B81;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_R11_EAC: types::GLenum = 0x9270;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RED: types::GLenum = 0x8225;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RED_RGTC1: types::GLenum = 0x8DBB;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RG: types::GLenum = 0x8226;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RG11_EAC: types::GLenum = 0x9272;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB: types::GLenum = 0x84ED;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB8_ETC2: types::GLenum = 0x9274;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: types::GLenum = 0x9276;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA: types::GLenum = 0x84EE;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA8_ETC2_EAC: types::GLenum = 0x9278;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_BPTC_UNORM: types::GLenum = 0x8E8C;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB_BPTC_SIGNED_FLOAT: types::GLenum = 0x8E8E;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: types::GLenum = 0x8E8F;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RG_RGTC2: types::GLenum = 0x8DBD;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SIGNED_R11_EAC: types::GLenum = 0x9271;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SIGNED_RED_RGTC1: types::GLenum = 0x8DBC;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SIGNED_RG11_EAC: types::GLenum = 0x9273;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SIGNED_RG_RGTC2: types::GLenum = 0x8DBE;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB: types::GLenum = 0x8C48;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: types::GLenum = 0x9279;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ETC2: types::GLenum = 0x9275;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: types::GLenum = 0x9277;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB_ALPHA: types::GLenum = 0x8C49;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB_ALPHA_BPTC_UNORM: types::GLenum = 0x8E8D;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A3;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_SHADER: types::GLenum = 0x91B9;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_SHADER_BIT: types::GLenum = 0x00000020;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_SUBROUTINE: types::GLenum = 0x92ED;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_SUBROUTINE_UNIFORM: types::GLenum = 0x92F3;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_TEXTURE: types::GLenum = 0x82A0;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_WORK_GROUP_SIZE: types::GLenum = 0x8267;
#[allow(dead_code, non_upper_case_globals)] pub const CONDITION_SATISFIED: types::GLenum = 0x911C;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT_ALPHA: types::GLenum = 0x8003;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT_COLOR: types::GLenum = 0x8001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_CORE_PROFILE_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAGS: types::GLenum = 0x821E;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAG_DEBUG_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT: types::GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_LOST: types::GLenum = 0x0507;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_PROFILE_MASK: types::GLenum = 0x9126;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_RELEASE_BEHAVIOR: types::GLenum = 0x82FB;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_RELEASE_BEHAVIOR_FLUSH: types::GLenum = 0x82FC;
#[allow(dead_code, non_upper_case_globals)] pub const COPY: types::GLenum = 0x1503;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_INVERTED: types::GLenum = 0x150C;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_READ_BUFFER: types::GLenum = 0x8F36;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_READ_BUFFER_BINDING: types::GLenum = 0x8F36;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_WRITE_BUFFER: types::GLenum = 0x8F37;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_WRITE_BUFFER_BINDING: types::GLenum = 0x8F37;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE: types::GLenum = 0x0B44;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE_MODE: types::GLenum = 0x0B45;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_PROGRAM: types::GLenum = 0x8B8D;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_QUERY: types::GLenum = 0x8865;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_VERTEX_ATTRIB: types::GLenum = 0x8626;
#[allow(dead_code, non_upper_case_globals)] pub const CW: types::GLenum = 0x0900;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_CALLBACK_FUNCTION: types::GLenum = 0x8244;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_CALLBACK_USER_PARAM: types::GLenum = 0x8245;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_GROUP_STACK_DEPTH: types::GLenum = 0x826D;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_LOGGED_MESSAGES: types::GLenum = 0x9145;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: types::GLenum = 0x8243;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_OUTPUT: types::GLenum = 0x92E0;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_OUTPUT_SYNCHRONOUS: types::GLenum = 0x8242;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_HIGH: types::GLenum = 0x9146;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_LOW: types::GLenum = 0x9148;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_MEDIUM: types::GLenum = 0x9147;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_NOTIFICATION: types::GLenum = 0x826B;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_API: types::GLenum = 0x8246;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_APPLICATION: types::GLenum = 0x824A;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_OTHER: types::GLenum = 0x824B;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_SHADER_COMPILER: types::GLenum = 0x8248;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_THIRD_PARTY: types::GLenum = 0x8249;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_WINDOW_SYSTEM: types::GLenum = 0x8247;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: types::GLenum = 0x824D;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_ERROR: types::GLenum = 0x824C;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_MARKER: types::GLenum = 0x8268;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_OTHER: types::GLenum = 0x8251;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PERFORMANCE: types::GLenum = 0x8250;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_POP_GROUP: types::GLenum = 0x826A;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PORTABILITY: types::GLenum = 0x824F;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PUSH_GROUP: types::GLenum = 0x8269;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: types::GLenum = 0x824E;
#[allow(dead_code, non_upper_case_globals)] pub const DECR: types::GLenum = 0x1E03;
#[allow(dead_code, non_upper_case_globals)] pub const DECR_WRAP: types::GLenum = 0x8508;
#[allow(dead_code, non_upper_case_globals)] pub const DELETE_STATUS: types::GLenum = 0x8B80;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH: types::GLenum = 0x1801;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH24_STENCIL8: types::GLenum = 0x88F0;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH32F_STENCIL8: types::GLenum = 0x8CAD;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_ATTACHMENT: types::GLenum = 0x8D00;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_BUFFER_BIT: types::GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_CLAMP: types::GLenum = 0x864F;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_CLEAR_VALUE: types::GLenum = 0x0B73;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT: types::GLenum = 0x1902;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT16: types::GLenum = 0x81A5;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT24: types::GLenum = 0x81A6;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT32: types::GLenum = 0x81A7;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT32F: types::GLenum = 0x8CAC;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENTS: types::GLenum = 0x8284;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_FUNC: types::GLenum = 0x0B74;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_RANGE: types::GLenum = 0x0B70;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_RENDERABLE: types::GLenum = 0x8287;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_STENCIL: types::GLenum = 0x84F9;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_STENCIL_ATTACHMENT: types::GLenum = 0x821A;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_STENCIL_TEXTURE_MODE: types::GLenum = 0x90EA;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_TEST: types::GLenum = 0x0B71;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_WRITEMASK: types::GLenum = 0x0B72;
#[allow(dead_code, non_upper_case_globals)] pub const DISPATCH_INDIRECT_BUFFER: types::GLenum = 0x90EE;
#[allow(dead_code, non_upper_case_globals)] pub const DISPATCH_INDIRECT_BUFFER_BINDING: types::GLenum = 0x90EF;
#[allow(dead_code, non_upper_case_globals)] pub const DISPLAY_LIST: types::GLenum = 0x82E7;
#[allow(dead_code, non_upper_case_globals)] pub const DITHER: types::GLenum = 0x0BD0;
#[allow(dead_code, non_upper_case_globals)] pub const DONT_CARE: types::GLenum = 0x1100;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE: types::GLenum = 0x140A;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLEBUFFER: types::GLenum = 0x0C32;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT2: types::GLenum = 0x8F46;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT2x3: types::GLenum = 0x8F49;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT2x4: types::GLenum = 0x8F4A;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT3: types::GLenum = 0x8F47;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT3x2: types::GLenum = 0x8F4B;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT3x4: types::GLenum = 0x8F4C;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT4: types::GLenum = 0x8F48;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT4x2: types::GLenum = 0x8F4D;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT4x3: types::GLenum = 0x8F4E;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_VEC2: types::GLenum = 0x8FFC;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_VEC3: types::GLenum = 0x8FFD;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_VEC4: types::GLenum = 0x8FFE;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER: types::GLenum = 0x0C01;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER0: types::GLenum = 0x8825;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER1: types::GLenum = 0x8826;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER10: types::GLenum = 0x882F;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER11: types::GLenum = 0x8830;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER12: types::GLenum = 0x8831;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER13: types::GLenum = 0x8832;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER14: types::GLenum = 0x8833;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER15: types::GLenum = 0x8834;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER2: types::GLenum = 0x8827;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER3: types::GLenum = 0x8828;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER4: types::GLenum = 0x8829;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER5: types::GLenum = 0x882A;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER6: types::GLenum = 0x882B;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER7: types::GLenum = 0x882C;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER8: types::GLenum = 0x882D;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER9: types::GLenum = 0x882E;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_FRAMEBUFFER: types::GLenum = 0x8CA9;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_FRAMEBUFFER_BINDING: types::GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_INDIRECT_BUFFER: types::GLenum = 0x8F3F;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_INDIRECT_BUFFER_BINDING: types::GLenum = 0x8F43;
#[allow(dead_code, non_upper_case_globals)] pub const DST_ALPHA: types::GLenum = 0x0304;
#[allow(dead_code, non_upper_case_globals)] pub const DST_COLOR: types::GLenum = 0x0306;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_COPY: types::GLenum = 0x88EA;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_DRAW: types::GLenum = 0x88E8;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_READ: types::GLenum = 0x88E9;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_STORAGE_BIT: types::GLenum = 0x0100;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BARRIER_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BUFFER: types::GLenum = 0x8893;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BUFFER_BINDING: types::GLenum = 0x8895;
#[allow(dead_code, non_upper_case_globals)] pub const EQUAL: types::GLenum = 0x0202;
#[allow(dead_code, non_upper_case_globals)] pub const EQUIV: types::GLenum = 0x1509;
#[allow(dead_code, non_upper_case_globals)] pub const EXTENSIONS: types::GLenum = 0x1F03;
#[allow(dead_code, non_upper_case_globals)] pub const FALSE: types::GLboolean = 0;
#[allow(dead_code, non_upper_case_globals)] pub const FASTEST: types::GLenum = 0x1101;
#[allow(dead_code, non_upper_case_globals)] pub const FILL: types::GLenum = 0x1B02;
#[allow(dead_code, non_upper_case_globals)] pub const FILTER: types::GLenum = 0x829A;
#[allow(dead_code, non_upper_case_globals)] pub const FIRST_VERTEX_CONVENTION: types::GLenum = 0x8E4D;
#[allow(dead_code, non_upper_case_globals)] pub const FIXED: types::GLenum = 0x140C;
#[allow(dead_code, non_upper_case_globals)] pub const FIXED_ONLY: types::GLenum = 0x891D;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT: types::GLenum = 0x1406;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_32_UNSIGNED_INT_24_8_REV: types::GLenum = 0x8DAD;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT2: types::GLenum = 0x8B5A;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT2x3: types::GLenum = 0x8B65;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT2x4: types::GLenum = 0x8B66;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT3: types::GLenum = 0x8B5B;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT3x2: types::GLenum = 0x8B67;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT3x4: types::GLenum = 0x8B68;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT4: types::GLenum = 0x8B5C;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT4x2: types::GLenum = 0x8B69;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT4x3: types::GLenum = 0x8B6A;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC2: types::GLenum = 0x8B50;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC3: types::GLenum = 0x8B51;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC4: types::GLenum = 0x8B52;
#[allow(dead_code, non_upper_case_globals)] pub const FRACTIONAL_EVEN: types::GLenum = 0x8E7C;
#[allow(dead_code, non_upper_case_globals)] pub const FRACTIONAL_ODD: types::GLenum = 0x8E7B;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_INTERPOLATION_OFFSET_BITS: types::GLenum = 0x8E5D;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SHADER: types::GLenum = 0x8B30;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SHADER_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SHADER_DERIVATIVE_HINT: types::GLenum = 0x8B8B;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SUBROUTINE: types::GLenum = 0x92EC;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SUBROUTINE_UNIFORM: types::GLenum = 0x92F2;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_TEXTURE: types::GLenum = 0x829F;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER: types::GLenum = 0x8D40;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: types::GLenum = 0x8215;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: types::GLenum = 0x8214;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: types::GLenum = 0x8210;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: types::GLenum = 0x8211;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: types::GLenum = 0x8216;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: types::GLenum = 0x8213;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_LAYERED: types::GLenum = 0x8DA7;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: types::GLenum = 0x8CD1;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: types::GLenum = 0x8CD0;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: types::GLenum = 0x8212;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: types::GLenum = 0x8217;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: types::GLenum = 0x8CD3;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: types::GLenum = 0x8CD4;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: types::GLenum = 0x8CD2;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_BARRIER_BIT: types::GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_BINDING: types::GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_BLEND: types::GLenum = 0x828B;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_COMPLETE: types::GLenum = 0x8CD5;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT: types::GLenum = 0x8218;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: types::GLenum = 0x9314;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_HEIGHT: types::GLenum = 0x9311;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_LAYERS: types::GLenum = 0x9312;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_SAMPLES: types::GLenum = 0x9313;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_WIDTH: types::GLenum = 0x9310;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: types::GLenum = 0x8CD6;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: types::GLenum = 0x8CDB;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: types::GLenum = 0x8DA8;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: types::GLenum = 0x8CD7;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: types::GLenum = 0x8D56;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: types::GLenum = 0x8CDC;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_RENDERABLE: types::GLenum = 0x8289;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_RENDERABLE_LAYERED: types::GLenum = 0x828A;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_SRGB: types::GLenum = 0x8DB9;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_UNDEFINED: types::GLenum = 0x8219;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_UNSUPPORTED: types::GLenum = 0x8CDD;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT: types::GLenum = 0x0404;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_AND_BACK: types::GLenum = 0x0408;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_FACE: types::GLenum = 0x0B46;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_LEFT: types::GLenum = 0x0400;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_RIGHT: types::GLenum = 0x0401;
#[allow(dead_code, non_upper_case_globals)] pub const FULL_SUPPORT: types::GLenum = 0x82B7;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_ADD: types::GLenum = 0x8006;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_REVERSE_SUBTRACT: types::GLenum = 0x800B;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_SUBTRACT: types::GLenum = 0x800A;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_INPUT_TYPE: types::GLenum = 0x8917;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_OUTPUT_TYPE: types::GLenum = 0x8918;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SHADER: types::GLenum = 0x8DD9;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SHADER_BIT: types::GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SHADER_INVOCATIONS: types::GLenum = 0x887F;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SUBROUTINE: types::GLenum = 0x92EB;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SUBROUTINE_UNIFORM: types::GLenum = 0x92F1;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_TEXTURE: types::GLenum = 0x829E;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_VERTICES_OUT: types::GLenum = 0x8916;
#[allow(dead_code, non_upper_case_globals)] pub const GEQUAL: types::GLenum = 0x0206;
#[allow(dead_code, non_upper_case_globals)] pub const GET_TEXTURE_IMAGE_FORMAT: types::GLenum = 0x8291;
#[allow(dead_code, non_upper_case_globals)] pub const GET_TEXTURE_IMAGE_TYPE: types::GLenum = 0x8292;
#[allow(dead_code, non_upper_case_globals)] pub const GREATER: types::GLenum = 0x0204;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN: types::GLenum = 0x1904;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN_INTEGER: types::GLenum = 0x8D95;
#[allow(dead_code, non_upper_case_globals)] pub const GUILTY_CONTEXT_RESET: types::GLenum = 0x8253;
#[allow(dead_code, non_upper_case_globals)] pub const HALF_FLOAT: types::GLenum = 0x140B;
#[allow(dead_code, non_upper_case_globals)] pub const HIGH_FLOAT: types::GLenum = 0x8DF2;
#[allow(dead_code, non_upper_case_globals)] pub const HIGH_INT: types::GLenum = 0x8DF5;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_1D: types::GLenum = 0x904C;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_1D_ARRAY: types::GLenum = 0x9052;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D: types::GLenum = 0x904D;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D_ARRAY: types::GLenum = 0x9053;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D_MULTISAMPLE: types::GLenum = 0x9055;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9056;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D_RECT: types::GLenum = 0x904F;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_3D: types::GLenum = 0x904E;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_ACCESS: types::GLenum = 0x8F3E;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_FORMAT: types::GLenum = 0x906E;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_LAYER: types::GLenum = 0x8F3D;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_LAYERED: types::GLenum = 0x8F3C;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_LEVEL: types::GLenum = 0x8F3B;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_NAME: types::GLenum = 0x8F3A;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BUFFER: types::GLenum = 0x9051;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_10_10_10_2: types::GLenum = 0x82C3;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_11_11_10: types::GLenum = 0x82C2;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_1_X_16: types::GLenum = 0x82BE;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_1_X_32: types::GLenum = 0x82BB;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_1_X_8: types::GLenum = 0x82C1;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_2_X_16: types::GLenum = 0x82BD;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_2_X_32: types::GLenum = 0x82BA;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_2_X_8: types::GLenum = 0x82C0;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_4_X_16: types::GLenum = 0x82BC;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_4_X_32: types::GLenum = 0x82B9;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_4_X_8: types::GLenum = 0x82BF;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_COMPATIBILITY_CLASS: types::GLenum = 0x82A8;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CUBE: types::GLenum = 0x9050;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CUBE_MAP_ARRAY: types::GLenum = 0x9054;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: types::GLenum = 0x90C9;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: types::GLenum = 0x90C8;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_FORMAT_COMPATIBILITY_TYPE: types::GLenum = 0x90C7;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_PIXEL_FORMAT: types::GLenum = 0x82A9;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_PIXEL_TYPE: types::GLenum = 0x82AA;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_TEXEL_SIZE: types::GLenum = 0x82A7;
#[allow(dead_code, non_upper_case_globals)] pub const IMPLEMENTATION_COLOR_READ_FORMAT: types::GLenum = 0x8B9B;
#[allow(dead_code, non_upper_case_globals)] pub const IMPLEMENTATION_COLOR_READ_TYPE: types::GLenum = 0x8B9A;
#[allow(dead_code, non_upper_case_globals)] pub const INCR: types::GLenum = 0x1E02;
#[allow(dead_code, non_upper_case_globals)] pub const INCR_WRAP: types::GLenum = 0x8507;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX: types::GLenum = 0x8222;
#[allow(dead_code, non_upper_case_globals)] pub const INFO_LOG_LENGTH: types::GLenum = 0x8B84;
#[allow(dead_code, non_upper_case_globals)] pub const INNOCENT_CONTEXT_RESET: types::GLenum = 0x8254;
#[allow(dead_code, non_upper_case_globals)] pub const INT: types::GLenum = 0x1404;
#[allow(dead_code, non_upper_case_globals)] pub const INTERLEAVED_ATTRIBS: types::GLenum = 0x8C8C;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_ALPHA_SIZE: types::GLenum = 0x8274;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_ALPHA_TYPE: types::GLenum = 0x827B;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_BLUE_SIZE: types::GLenum = 0x8273;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_BLUE_TYPE: types::GLenum = 0x827A;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_DEPTH_SIZE: types::GLenum = 0x8275;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_DEPTH_TYPE: types::GLenum = 0x827C;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_GREEN_SIZE: types::GLenum = 0x8272;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_GREEN_TYPE: types::GLenum = 0x8279;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_PREFERRED: types::GLenum = 0x8270;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_RED_SIZE: types::GLenum = 0x8271;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_RED_TYPE: types::GLenum = 0x8278;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_SHARED_SIZE: types::GLenum = 0x8277;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_STENCIL_SIZE: types::GLenum = 0x8276;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_STENCIL_TYPE: types::GLenum = 0x827D;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_SUPPORTED: types::GLenum = 0x826F;
#[allow(dead_code, non_upper_case_globals)] pub const INT_2_10_10_10_REV: types::GLenum = 0x8D9F;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_1D: types::GLenum = 0x9057;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_1D_ARRAY: types::GLenum = 0x905D;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D: types::GLenum = 0x9058;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D_ARRAY: types::GLenum = 0x905E;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D_MULTISAMPLE: types::GLenum = 0x9060;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9061;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D_RECT: types::GLenum = 0x905A;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_3D: types::GLenum = 0x9059;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_BUFFER: types::GLenum = 0x905C;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_CUBE: types::GLenum = 0x905B;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_CUBE_MAP_ARRAY: types::GLenum = 0x905F;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_1D: types::GLenum = 0x8DC9;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_1D_ARRAY: types::GLenum = 0x8DCE;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D: types::GLenum = 0x8DCA;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_ARRAY: types::GLenum = 0x8DCF;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x9109;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910C;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_RECT: types::GLenum = 0x8DCD;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_3D: types::GLenum = 0x8DCB;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_BUFFER: types::GLenum = 0x8DD0;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_CUBE: types::GLenum = 0x8DCC;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_CUBE_MAP_ARRAY: types::GLenum = 0x900E;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC2: types::GLenum = 0x8B53;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC3: types::GLenum = 0x8B54;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC4: types::GLenum = 0x8B55;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_ENUM: types::GLenum = 0x0500;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_FRAMEBUFFER_OPERATION: types::GLenum = 0x0506;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_INDEX: types::GLuint = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_OPERATION: types::GLenum = 0x0502;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_VALUE: types::GLenum = 0x0501;
#[allow(dead_code, non_upper_case_globals)] pub const INVERT: types::GLenum = 0x150A;
#[allow(dead_code, non_upper_case_globals)] pub const ISOLINES: types::GLenum = 0x8E7A;
#[allow(dead_code, non_upper_case_globals)] pub const IS_PER_PATCH: types::GLenum = 0x92E7;
#[allow(dead_code, non_upper_case_globals)] pub const IS_ROW_MAJOR: types::GLenum = 0x9300;
#[allow(dead_code, non_upper_case_globals)] pub const KEEP: types::GLenum = 0x1E00;
#[allow(dead_code, non_upper_case_globals)] pub const LAST_VERTEX_CONVENTION: types::GLenum = 0x8E4E;
#[allow(dead_code, non_upper_case_globals)] pub const LAYER_PROVOKING_VERTEX: types::GLenum = 0x825E;
#[allow(dead_code, non_upper_case_globals)] pub const LEFT: types::GLenum = 0x0406;
#[allow(dead_code, non_upper_case_globals)] pub const LEQUAL: types::GLenum = 0x0203;
#[allow(dead_code, non_upper_case_globals)] pub const LESS: types::GLenum = 0x0201;
#[allow(dead_code, non_upper_case_globals)] pub const LINE: types::GLenum = 0x1B01;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR: types::GLenum = 0x2601;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_LINEAR: types::GLenum = 0x2703;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_NEAREST: types::GLenum = 0x2701;
#[allow(dead_code, non_upper_case_globals)] pub const LINES: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const LINES_ADJACENCY: types::GLenum = 0x000A;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_LOOP: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_SMOOTH: types::GLenum = 0x0B20;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_SMOOTH_HINT: types::GLenum = 0x0C52;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STRIP: types::GLenum = 0x0003;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STRIP_ADJACENCY: types::GLenum = 0x000B;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH: types::GLenum = 0x0B21;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH_GRANULARITY: types::GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH_RANGE: types::GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)] pub const LINK_STATUS: types::GLenum = 0x8B82;
#[allow(dead_code, non_upper_case_globals)] pub const LOCATION: types::GLenum = 0x930E;
#[allow(dead_code, non_upper_case_globals)] pub const LOCATION_COMPONENT: types::GLenum = 0x934A;
#[allow(dead_code, non_upper_case_globals)] pub const LOCATION_INDEX: types::GLenum = 0x930F;
#[allow(dead_code, non_upper_case_globals)] pub const LOGIC_OP_MODE: types::GLenum = 0x0BF0;
#[allow(dead_code, non_upper_case_globals)] pub const LOSE_CONTEXT_ON_RESET: types::GLenum = 0x8252;
#[allow(dead_code, non_upper_case_globals)] pub const LOWER_LEFT: types::GLenum = 0x8CA1;
#[allow(dead_code, non_upper_case_globals)] pub const LOW_FLOAT: types::GLenum = 0x8DF0;
#[allow(dead_code, non_upper_case_globals)] pub const LOW_INT: types::GLenum = 0x8DF3;
#[allow(dead_code, non_upper_case_globals)] pub const MAJOR_VERSION: types::GLenum = 0x821B;
#[allow(dead_code, non_upper_case_globals)] pub const MANUAL_GENERATE_MIPMAP: types::GLenum = 0x8294;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_COHERENT_BIT: types::GLenum = 0x0080;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_FLUSH_EXPLICIT_BIT: types::GLenum = 0x0010;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_INVALIDATE_BUFFER_BIT: types::GLenum = 0x0008;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_INVALIDATE_RANGE_BIT: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_PERSISTENT_BIT: types::GLenum = 0x0040;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_READ_BIT: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_UNSYNCHRONIZED_BIT: types::GLenum = 0x0020;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_WRITE_BIT: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)] pub const MATRIX_STRIDE: types::GLenum = 0x92FF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX: types::GLenum = 0x8008;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_3D_TEXTURE_SIZE: types::GLenum = 0x8073;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ARRAY_TEXTURE_LAYERS: types::GLenum = 0x88FF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: types::GLenum = 0x92DC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ATOMIC_COUNTER_BUFFER_SIZE: types::GLenum = 0x92D8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CLIP_DISTANCES: types::GLenum = 0x0D32;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COLOR_ATTACHMENTS: types::GLenum = 0x8CDF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COLOR_TEXTURE_SAMPLES: types::GLenum = 0x910E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_ATOMIC_COUNTERS: types::GLenum = 0x92D7;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92D1;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_CLIP_AND_CULL_DISTANCES: types::GLenum = 0x82FA;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: types::GLenum = 0x8266;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_DIMENSIONS: types::GLenum = 0x8282;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: types::GLenum = 0x8A33;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: types::GLenum = 0x8A32;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_IMAGE_UNIFORMS: types::GLenum = 0x90CF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: types::GLenum = 0x8F39;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_SHADER_OUTPUT_RESOURCES: types::GLenum = 0x8F39;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90DC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: types::GLenum = 0x8E1E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: types::GLenum = 0x8E1F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8B4D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_UNIFORM_BLOCKS: types::GLenum = 0x8A2E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: types::GLenum = 0x8A31;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_ATOMIC_COUNTERS: types::GLenum = 0x8265;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x8264;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_IMAGE_UNIFORMS: types::GLenum = 0x91BD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90DB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_SHARED_MEMORY_SIZE: types::GLenum = 0x8262;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_TEXTURE_IMAGE_UNITS: types::GLenum = 0x91BC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_UNIFORM_BLOCKS: types::GLenum = 0x91BB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_UNIFORM_COMPONENTS: types::GLenum = 0x8263;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_WORK_GROUP_COUNT: types::GLenum = 0x91BE;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_WORK_GROUP_INVOCATIONS: types::GLenum = 0x90EB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_WORK_GROUP_SIZE: types::GLenum = 0x91BF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CUBE_MAP_TEXTURE_SIZE: types::GLenum = 0x851C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CULL_DISTANCES: types::GLenum = 0x82F9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_GROUP_STACK_DEPTH: types::GLenum = 0x826C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_LOGGED_MESSAGES: types::GLenum = 0x9144;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_MESSAGE_LENGTH: types::GLenum = 0x9143;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEPTH: types::GLenum = 0x8280;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEPTH_TEXTURE_SAMPLES: types::GLenum = 0x910F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DRAW_BUFFERS: types::GLenum = 0x8824;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: types::GLenum = 0x88FC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ELEMENTS_INDICES: types::GLenum = 0x80E9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ELEMENTS_VERTICES: types::GLenum = 0x80E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ELEMENT_INDEX: types::GLenum = 0x8D6B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_ATOMIC_COUNTERS: types::GLenum = 0x92D6;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92D0;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_IMAGE_UNIFORMS: types::GLenum = 0x90CE;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_INPUT_COMPONENTS: types::GLenum = 0x9125;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_INTERPOLATION_OFFSET: types::GLenum = 0x8E5C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90DA;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_UNIFORM_BLOCKS: types::GLenum = 0x8A2D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: types::GLenum = 0x8B49;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_UNIFORM_VECTORS: types::GLenum = 0x8DFD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAMEBUFFER_HEIGHT: types::GLenum = 0x9316;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAMEBUFFER_LAYERS: types::GLenum = 0x9317;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAMEBUFFER_SAMPLES: types::GLenum = 0x9318;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAMEBUFFER_WIDTH: types::GLenum = 0x9315;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_ATOMIC_COUNTERS: types::GLenum = 0x92D5;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92CF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_IMAGE_UNIFORMS: types::GLenum = 0x90CD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_INPUT_COMPONENTS: types::GLenum = 0x9123;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: types::GLenum = 0x9124;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_OUTPUT_VERTICES: types::GLenum = 0x8DE0;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_SHADER_INVOCATIONS: types::GLenum = 0x8E5A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90D7;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8C29;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: types::GLenum = 0x8DE1;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_UNIFORM_BLOCKS: types::GLenum = 0x8A2C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: types::GLenum = 0x8DDF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_HEIGHT: types::GLenum = 0x827F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_IMAGE_SAMPLES: types::GLenum = 0x906D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_IMAGE_UNITS: types::GLenum = 0x8F38;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_INTEGER_SAMPLES: types::GLenum = 0x9110;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_LABEL_LENGTH: types::GLenum = 0x82E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_LAYERS: types::GLenum = 0x8281;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_NAME_LENGTH: types::GLenum = 0x92F6;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_NUM_ACTIVE_VARIABLES: types::GLenum = 0x92F7;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_NUM_COMPATIBLE_SUBROUTINES: types::GLenum = 0x92F8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PATCH_VERTICES: types::GLenum = 0x8E7D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PROGRAM_TEXEL_OFFSET: types::GLenum = 0x8905;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PROGRAM_TEXTURE_GATHER_OFFSET: types::GLenum = 0x8E5F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_RECTANGLE_TEXTURE_SIZE: types::GLenum = 0x84F8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_RENDERBUFFER_SIZE: types::GLenum = 0x84E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SAMPLES: types::GLenum = 0x8D57;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SAMPLE_MASK_WORDS: types::GLenum = 0x8E59;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SERVER_WAIT_TIMEOUT: types::GLenum = 0x9111;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SHADER_STORAGE_BLOCK_SIZE: types::GLenum = 0x90DE;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: types::GLenum = 0x90DD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SUBROUTINES: types::GLenum = 0x8DE7;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SUBROUTINE_UNIFORM_LOCATIONS: types::GLenum = 0x8DE8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_ATOMIC_COUNTERS: types::GLenum = 0x92D3;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92CD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_IMAGE_UNIFORMS: types::GLenum = 0x90CB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_INPUT_COMPONENTS: types::GLenum = 0x886C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_OUTPUT_COMPONENTS: types::GLenum = 0x8E83;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90D8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8E81;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: types::GLenum = 0x8E85;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_UNIFORM_BLOCKS: types::GLenum = 0x8E89;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_UNIFORM_COMPONENTS: types::GLenum = 0x8E7F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_ATOMIC_COUNTERS: types::GLenum = 0x92D4;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92CE;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_IMAGE_UNIFORMS: types::GLenum = 0x90CC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_INPUT_COMPONENTS: types::GLenum = 0x886D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: types::GLenum = 0x8E86;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90D9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8E82;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_UNIFORM_BLOCKS: types::GLenum = 0x8E8A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: types::GLenum = 0x8E80;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_GEN_LEVEL: types::GLenum = 0x8E7E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_PATCH_COMPONENTS: types::GLenum = 0x8E84;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_BUFFER_SIZE: types::GLenum = 0x8C2B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8872;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_LOD_BIAS: types::GLenum = 0x84FD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_MAX_ANISOTROPY_EXT: types::GLenum = 0x84FF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_SIZE: types::GLenum = 0x0D33;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_BUFFERS: types::GLenum = 0x8E70;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: types::GLenum = 0x8C8A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: types::GLenum = 0x8C8B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: types::GLenum = 0x8C80;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_UNIFORM_BLOCK_SIZE: types::GLenum = 0x8A30;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_UNIFORM_BUFFER_BINDINGS: types::GLenum = 0x8A2F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_UNIFORM_LOCATIONS: types::GLenum = 0x826E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VARYING_COMPONENTS: types::GLenum = 0x8B4B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VARYING_FLOATS: types::GLenum = 0x8B4B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VARYING_VECTORS: types::GLenum = 0x8DFC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATOMIC_COUNTERS: types::GLenum = 0x92D2;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92CC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIBS: types::GLenum = 0x8869;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIB_BINDINGS: types::GLenum = 0x82DA;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: types::GLenum = 0x82D9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIB_STRIDE: types::GLenum = 0x82E5;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_IMAGE_UNIFORMS: types::GLenum = 0x90CA;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_OUTPUT_COMPONENTS: types::GLenum = 0x9122;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90D6;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_STREAMS: types::GLenum = 0x8E71;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8B4C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_UNIFORM_BLOCKS: types::GLenum = 0x8A2B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_UNIFORM_COMPONENTS: types::GLenum = 0x8B4A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_UNIFORM_VECTORS: types::GLenum = 0x8DFB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VIEWPORTS: types::GLenum = 0x825B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VIEWPORT_DIMS: types::GLenum = 0x0D3A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_WIDTH: types::GLenum = 0x827E;
#[allow(dead_code, non_upper_case_globals)] pub const MEDIUM_FLOAT: types::GLenum = 0x8DF1;
#[allow(dead_code, non_upper_case_globals)] pub const MEDIUM_INT: types::GLenum = 0x8DF4;
#[allow(dead_code, non_upper_case_globals)] pub const MIN: types::GLenum = 0x8007;
#[allow(dead_code, non_upper_case_globals)] pub const MINOR_VERSION: types::GLenum = 0x821C;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_FRAGMENT_INTERPOLATION_OFFSET: types::GLenum = 0x8E5B;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_MAP_BUFFER_ALIGNMENT: types::GLenum = 0x90BC;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_PROGRAM_TEXEL_OFFSET: types::GLenum = 0x8904;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_PROGRAM_TEXTURE_GATHER_OFFSET: types::GLenum = 0x8E5E;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_SAMPLE_SHADING_VALUE: types::GLenum = 0x8C37;
#[allow(dead_code, non_upper_case_globals)] pub const MIPMAP: types::GLenum = 0x8293;
#[allow(dead_code, non_upper_case_globals)] pub const MIRRORED_REPEAT: types::GLenum = 0x8370;
#[allow(dead_code, non_upper_case_globals)] pub const MIRROR_CLAMP_TO_EDGE: types::GLenum = 0x8743;
#[allow(dead_code, non_upper_case_globals)] pub const MULTISAMPLE: types::GLenum = 0x809D;
#[allow(dead_code, non_upper_case_globals)] pub const NAME_LENGTH: types::GLenum = 0x92F9;
#[allow(dead_code, non_upper_case_globals)] pub const NAND: types::GLenum = 0x150E;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST: types::GLenum = 0x2600;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_LINEAR: types::GLenum = 0x2702;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_NEAREST: types::GLenum = 0x2700;
#[allow(dead_code, non_upper_case_globals)] pub const NEGATIVE_ONE_TO_ONE: types::GLenum = 0x935E;
#[allow(dead_code, non_upper_case_globals)] pub const NEVER: types::GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)] pub const NICEST: types::GLenum = 0x1102;
#[allow(dead_code, non_upper_case_globals)] pub const NONE: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NOOP: types::GLenum = 0x1505;
#[allow(dead_code, non_upper_case_globals)] pub const NOR: types::GLenum = 0x1508;
#[allow(dead_code, non_upper_case_globals)] pub const NOTEQUAL: types::GLenum = 0x0205;
#[allow(dead_code, non_upper_case_globals)] pub const NO_ERROR: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NO_RESET_NOTIFICATION: types::GLenum = 0x8261;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_ACTIVE_VARIABLES: types::GLenum = 0x9304;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_COMPATIBLE_SUBROUTINES: types::GLenum = 0x8E4A;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A2;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_EXTENSIONS: types::GLenum = 0x821D;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_PROGRAM_BINARY_FORMATS: types::GLenum = 0x87FE;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_SAMPLE_COUNTS: types::GLenum = 0x9380;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_SHADER_BINARY_FORMATS: types::GLenum = 0x8DF9;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_SHADING_LANGUAGE_VERSIONS: types::GLenum = 0x82E9;
#[allow(dead_code, non_upper_case_globals)] pub const OBJECT_TYPE: types::GLenum = 0x9112;
#[allow(dead_code, non_upper_case_globals)] pub const OFFSET: types::GLenum = 0x92FC;
#[allow(dead_code, non_upper_case_globals)] pub const ONE: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_CONSTANT_ALPHA: types::GLenum = 0x8004;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_CONSTANT_COLOR: types::GLenum = 0x8002;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_ALPHA: types::GLenum = 0x0305;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_COLOR: types::GLenum = 0x0307;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC1_ALPHA: types::GLenum = 0x88FB;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC1_COLOR: types::GLenum = 0x88FA;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_ALPHA: types::GLenum = 0x0303;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_COLOR: types::GLenum = 0x0301;
#[allow(dead_code, non_upper_case_globals)] pub const OR: types::GLenum = 0x1507;
#[allow(dead_code, non_upper_case_globals)] pub const OR_INVERTED: types::GLenum = 0x150D;
#[allow(dead_code, non_upper_case_globals)] pub const OR_REVERSE: types::GLenum = 0x150B;
#[allow(dead_code, non_upper_case_globals)] pub const OUT_OF_MEMORY: types::GLenum = 0x0505;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_ALIGNMENT: types::GLenum = 0x0D05;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_COMPRESSED_BLOCK_DEPTH: types::GLenum = 0x912D;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_COMPRESSED_BLOCK_HEIGHT: types::GLenum = 0x912C;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_COMPRESSED_BLOCK_SIZE: types::GLenum = 0x912E;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_COMPRESSED_BLOCK_WIDTH: types::GLenum = 0x912B;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_IMAGE_HEIGHT: types::GLenum = 0x806C;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_LSB_FIRST: types::GLenum = 0x0D01;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_ROW_LENGTH: types::GLenum = 0x0D02;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SKIP_IMAGES: types::GLenum = 0x806B;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SKIP_PIXELS: types::GLenum = 0x0D04;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SKIP_ROWS: types::GLenum = 0x0D03;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SWAP_BYTES: types::GLenum = 0x0D00;
#[allow(dead_code, non_upper_case_globals)] pub const PATCHES: types::GLenum = 0x000E;
#[allow(dead_code, non_upper_case_globals)] pub const PATCH_DEFAULT_INNER_LEVEL: types::GLenum = 0x8E73;
#[allow(dead_code, non_upper_case_globals)] pub const PATCH_DEFAULT_OUTER_LEVEL: types::GLenum = 0x8E74;
#[allow(dead_code, non_upper_case_globals)] pub const PATCH_VERTICES: types::GLenum = 0x8E72;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_BUFFER_BARRIER_BIT: types::GLenum = 0x00000080;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_PACK_BUFFER: types::GLenum = 0x88EB;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_PACK_BUFFER_BINDING: types::GLenum = 0x88ED;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_UNPACK_BUFFER: types::GLenum = 0x88EC;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_UNPACK_BUFFER_BINDING: types::GLenum = 0x88EF;
#[allow(dead_code, non_upper_case_globals)] pub const POINT: types::GLenum = 0x1B00;
#[allow(dead_code, non_upper_case_globals)] pub const POINTS: types::GLenum = 0x0000;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_FADE_THRESHOLD_SIZE: types::GLenum = 0x8128;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE: types::GLenum = 0x0B11;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_GRANULARITY: types::GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_RANGE: types::GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SPRITE_COORD_ORIGIN: types::GLenum = 0x8CA0;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_MODE: types::GLenum = 0x0B40;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FACTOR: types::GLenum = 0x8038;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FILL: types::GLenum = 0x8037;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_LINE: types::GLenum = 0x2A02;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_POINT: types::GLenum = 0x2A01;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_UNITS: types::GLenum = 0x2A00;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_SMOOTH: types::GLenum = 0x0B41;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_SMOOTH_HINT: types::GLenum = 0x0C53;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVES_GENERATED: types::GLenum = 0x8C87;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_RESTART: types::GLenum = 0x8F9D;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_RESTART_FIXED_INDEX: types::GLenum = 0x8D69;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: types::GLenum = 0x8221;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_RESTART_INDEX: types::GLenum = 0x8F9E;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM: types::GLenum = 0x82E2;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_BINARY_FORMATS: types::GLenum = 0x87FF;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_BINARY_LENGTH: types::GLenum = 0x8741;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_BINARY_RETRIEVABLE_HINT: types::GLenum = 0x8257;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_INPUT: types::GLenum = 0x92E3;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_OUTPUT: types::GLenum = 0x92E4;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_PIPELINE: types::GLenum = 0x82E4;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_PIPELINE_BINDING: types::GLenum = 0x825A;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_POINT_SIZE: types::GLenum = 0x8642;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_SEPARABLE: types::GLenum = 0x8258;
#[allow(dead_code, non_upper_case_globals)] pub const PROVOKING_VERTEX: types::GLenum = 0x8E4F;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_1D: types::GLenum = 0x8063;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_1D_ARRAY: types::GLenum = 0x8C19;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D: types::GLenum = 0x8064;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D_ARRAY: types::GLenum = 0x8C1B;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D_MULTISAMPLE: types::GLenum = 0x9101;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9103;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_3D: types::GLenum = 0x8070;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_CUBE_MAP: types::GLenum = 0x851B;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_CUBE_MAP_ARRAY: types::GLenum = 0x900B;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_RECTANGLE: types::GLenum = 0x84F7;
#[allow(dead_code, non_upper_case_globals)] pub const QUADS: types::GLenum = 0x0007;
#[allow(dead_code, non_upper_case_globals)] pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: types::GLenum = 0x8E4C;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY: types::GLenum = 0x82E3;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BUFFER: types::GLenum = 0x9192;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BUFFER_BARRIER_BIT: types::GLenum = 0x00008000;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BUFFER_BINDING: types::GLenum = 0x9193;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BY_REGION_NO_WAIT: types::GLenum = 0x8E16;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BY_REGION_NO_WAIT_INVERTED: types::GLenum = 0x8E1A;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BY_REGION_WAIT: types::GLenum = 0x8E15;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BY_REGION_WAIT_INVERTED: types::GLenum = 0x8E19;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_COUNTER_BITS: types::GLenum = 0x8864;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_NO_WAIT: types::GLenum = 0x8E14;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_NO_WAIT_INVERTED: types::GLenum = 0x8E18;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_RESULT: types::GLenum = 0x8866;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_RESULT_AVAILABLE: types::GLenum = 0x8867;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_RESULT_NO_WAIT: types::GLenum = 0x9194;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_TARGET: types::GLenum = 0x82EA;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_WAIT: types::GLenum = 0x8E13;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_WAIT_INVERTED: types::GLenum = 0x8E17;
#[allow(dead_code, non_upper_case_globals)] pub const R11F_G11F_B10F: types::GLenum = 0x8C3A;
#[allow(dead_code, non_upper_case_globals)] pub const R16: types::GLenum = 0x822A;
#[allow(dead_code, non_upper_case_globals)] pub const R16F: types::GLenum = 0x822D;
#[allow(dead_code, non_upper_case_globals)] pub const R16I: types::GLenum = 0x8233;
#[allow(dead_code, non_upper_case_globals)] pub const R16UI: types::GLenum = 0x8234;
#[allow(dead_code, non_upper_case_globals)] pub const R16_SNORM: types::GLenum = 0x8F98;
#[allow(dead_code, non_upper_case_globals)] pub const R32F: types::GLenum = 0x822E;
#[allow(dead_code, non_upper_case_globals)] pub const R32I: types::GLenum = 0x8235;
#[allow(dead_code, non_upper_case_globals)] pub const R32UI: types::GLenum = 0x8236;
#[allow(dead_code, non_upper_case_globals)] pub const R3_G3_B2: types::GLenum = 0x2A10;
#[allow(dead_code, non_upper_case_globals)] pub const R8: types::GLenum = 0x8229;
#[allow(dead_code, non_upper_case_globals)] pub const R8I: types::GLenum = 0x8231;
#[allow(dead_code, non_upper_case_globals)] pub const R8UI: types::GLenum = 0x8232;
#[allow(dead_code, non_upper_case_globals)] pub const R8_SNORM: types::GLenum = 0x8F94;
#[allow(dead_code, non_upper_case_globals)] pub const RASTERIZER_DISCARD: types::GLenum = 0x8C89;
#[allow(dead_code, non_upper_case_globals)] pub const READ_BUFFER: types::GLenum = 0x0C02;
#[allow(dead_code, non_upper_case_globals)] pub const READ_FRAMEBUFFER: types::GLenum = 0x8CA8;
#[allow(dead_code, non_upper_case_globals)] pub const READ_FRAMEBUFFER_BINDING: types::GLenum = 0x8CAA;
#[allow(dead_code, non_upper_case_globals)] pub const READ_ONLY: types::GLenum = 0x88B8;
#[allow(dead_code, non_upper_case_globals)] pub const READ_PIXELS: types::GLenum = 0x828C;
#[allow(dead_code, non_upper_case_globals)] pub const READ_PIXELS_FORMAT: types::GLenum = 0x828D;
#[allow(dead_code, non_upper_case_globals)] pub const READ_PIXELS_TYPE: types::GLenum = 0x828E;
#[allow(dead_code, non_upper_case_globals)] pub const READ_WRITE: types::GLenum = 0x88BA;
#[allow(dead_code, non_upper_case_globals)] pub const RED: types::GLenum = 0x1903;
#[allow(dead_code, non_upper_case_globals)] pub const RED_INTEGER: types::GLenum = 0x8D94;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_COMPUTE_SHADER: types::GLenum = 0x930B;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_FRAGMENT_SHADER: types::GLenum = 0x930A;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_GEOMETRY_SHADER: types::GLenum = 0x9309;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_TESS_CONTROL_SHADER: types::GLenum = 0x9307;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_TESS_EVALUATION_SHADER: types::GLenum = 0x9308;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_VERTEX_SHADER: types::GLenum = 0x9306;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER: types::GLenum = 0x8D41;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_ALPHA_SIZE: types::GLenum = 0x8D53;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_BINDING: types::GLenum = 0x8CA7;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_BLUE_SIZE: types::GLenum = 0x8D52;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_DEPTH_SIZE: types::GLenum = 0x8D54;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_GREEN_SIZE: types::GLenum = 0x8D51;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_HEIGHT: types::GLenum = 0x8D43;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_INTERNAL_FORMAT: types::GLenum = 0x8D44;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_RED_SIZE: types::GLenum = 0x8D50;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_SAMPLES: types::GLenum = 0x8CAB;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_STENCIL_SIZE: types::GLenum = 0x8D55;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_WIDTH: types::GLenum = 0x8D42;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERER: types::GLenum = 0x1F01;
#[allow(dead_code, non_upper_case_globals)] pub const REPEAT: types::GLenum = 0x2901;
#[allow(dead_code, non_upper_case_globals)] pub const REPLACE: types::GLenum = 0x1E01;
#[allow(dead_code, non_upper_case_globals)] pub const RESET_NOTIFICATION_STRATEGY: types::GLenum = 0x8256;
#[allow(dead_code, non_upper_case_globals)] pub const RG: types::GLenum = 0x8227;
#[allow(dead_code, non_upper_case_globals)] pub const RG16: types::GLenum = 0x822C;
#[allow(dead_code, non_upper_case_globals)] pub const RG16F: types::GLenum = 0x822F;
#[allow(dead_code, non_upper_case_globals)] pub const RG16I: types::GLenum = 0x8239;
#[allow(dead_code, non_upper_case_globals)] pub const RG16UI: types::GLenum = 0x823A;
#[allow(dead_code, non_upper_case_globals)] pub const RG16_SNORM: types::GLenum = 0x8F99;
#[allow(dead_code, non_upper_case_globals)] pub const RG32F: types::GLenum = 0x8230;
#[allow(dead_code, non_upper_case_globals)] pub const RG32I: types::GLenum = 0x823B;
#[allow(dead_code, non_upper_case_globals)] pub const RG32UI: types::GLenum = 0x823C;
#[allow(dead_code, non_upper_case_globals)] pub const RG8: types::GLenum = 0x822B;
#[allow(dead_code, non_upper_case_globals)] pub const RG8I: types::GLenum = 0x8237;
#[allow(dead_code, non_upper_case_globals)] pub const RG8UI: types::GLenum = 0x8238;
#[allow(dead_code, non_upper_case_globals)] pub const RG8_SNORM: types::GLenum = 0x8F95;
#[allow(dead_code, non_upper_case_globals)] pub const RGB: types::GLenum = 0x1907;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10: types::GLenum = 0x8052;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10_A2: types::GLenum = 0x8059;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10_A2UI: types::GLenum = 0x906F;
#[allow(dead_code, non_upper_case_globals)] pub const RGB12: types::GLenum = 0x8053;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16: types::GLenum = 0x8054;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16F: types::GLenum = 0x881B;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16I: types::GLenum = 0x8D89;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16UI: types::GLenum = 0x8D77;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16_SNORM: types::GLenum = 0x8F9A;
#[allow(dead_code, non_upper_case_globals)] pub const RGB32F: types::GLenum = 0x8815;
#[allow(dead_code, non_upper_case_globals)] pub const RGB32I: types::GLenum = 0x8D83;
#[allow(dead_code, non_upper_case_globals)] pub const RGB32UI: types::GLenum = 0x8D71;
#[allow(dead_code, non_upper_case_globals)] pub const RGB4: types::GLenum = 0x804F;
#[allow(dead_code, non_upper_case_globals)] pub const RGB5: types::GLenum = 0x8050;
#[allow(dead_code, non_upper_case_globals)] pub const RGB565: types::GLenum = 0x8D62;
#[allow(dead_code, non_upper_case_globals)] pub const RGB5_A1: types::GLenum = 0x8057;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8: types::GLenum = 0x8051;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8I: types::GLenum = 0x8D8F;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8UI: types::GLenum = 0x8D7D;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8_SNORM: types::GLenum = 0x8F96;
#[allow(dead_code, non_upper_case_globals)] pub const RGB9_E5: types::GLenum = 0x8C3D;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA: types::GLenum = 0x1908;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA12: types::GLenum = 0x805A;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16: types::GLenum = 0x805B;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16F: types::GLenum = 0x881A;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16I: types::GLenum = 0x8D88;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16UI: types::GLenum = 0x8D76;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16_SNORM: types::GLenum = 0x8F9B;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA2: types::GLenum = 0x8055;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA32F: types::GLenum = 0x8814;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA32I: types::GLenum = 0x8D82;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA32UI: types::GLenum = 0x8D70;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA4: types::GLenum = 0x8056;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8: types::GLenum = 0x8058;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8I: types::GLenum = 0x8D8E;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8UI: types::GLenum = 0x8D7C;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8_SNORM: types::GLenum = 0x8F97;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA_INTEGER: types::GLenum = 0x8D99;
#[allow(dead_code, non_upper_case_globals)] pub const RGB_INTEGER: types::GLenum = 0x8D98;
#[allow(dead_code, non_upper_case_globals)] pub const RG_INTEGER: types::GLenum = 0x8228;
#[allow(dead_code, non_upper_case_globals)] pub const RIGHT: types::GLenum = 0x0407;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER: types::GLenum = 0x82E6;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_1D: types::GLenum = 0x8B5D;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_1D_ARRAY: types::GLenum = 0x8DC0;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_1D_ARRAY_SHADOW: types::GLenum = 0x8DC3;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_1D_SHADOW: types::GLenum = 0x8B61;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D: types::GLenum = 0x8B5E;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_ARRAY: types::GLenum = 0x8DC1;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_ARRAY_SHADOW: types::GLenum = 0x8DC4;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x9108;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910B;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_RECT: types::GLenum = 0x8B63;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_RECT_SHADOW: types::GLenum = 0x8B64;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_SHADOW: types::GLenum = 0x8B62;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_3D: types::GLenum = 0x8B5F;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_BINDING: types::GLenum = 0x8919;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_BUFFER: types::GLenum = 0x8DC2;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE: types::GLenum = 0x8B60;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE_MAP_ARRAY: types::GLenum = 0x900C;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE_MAP_ARRAY_SHADOW: types::GLenum = 0x900D;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE_SHADOW: types::GLenum = 0x8DC5;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLES: types::GLenum = 0x80A9;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLES_PASSED: types::GLenum = 0x8914;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_ALPHA_TO_COVERAGE: types::GLenum = 0x809E;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_ALPHA_TO_ONE: types::GLenum = 0x809F;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_BUFFERS: types::GLenum = 0x80A8;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE: types::GLenum = 0x80A0;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE_INVERT: types::GLenum = 0x80AB;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE_VALUE: types::GLenum = 0x80AA;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_MASK: types::GLenum = 0x8E51;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_MASK_VALUE: types::GLenum = 0x8E52;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_POSITION: types::GLenum = 0x8E50;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_SHADING: types::GLenum = 0x8C36;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_BOX: types::GLenum = 0x0C10;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_TEST: types::GLenum = 0x0C11;
#[allow(dead_code, non_upper_case_globals)] pub const SEPARATE_ATTRIBS: types::GLenum = 0x8C8D;
#[allow(dead_code, non_upper_case_globals)] pub const SET: types::GLenum = 0x150F;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER: types::GLenum = 0x82E1;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_BINARY_FORMATS: types::GLenum = 0x8DF8;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_COMPILER: types::GLenum = 0x8DFA;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_IMAGE_ACCESS_BARRIER_BIT: types::GLenum = 0x00000020;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_IMAGE_ATOMIC: types::GLenum = 0x82A6;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_IMAGE_LOAD: types::GLenum = 0x82A4;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_IMAGE_STORE: types::GLenum = 0x82A5;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_SOURCE_LENGTH: types::GLenum = 0x8B88;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BARRIER_BIT: types::GLenum = 0x00002000;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BLOCK: types::GLenum = 0x92E6;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER: types::GLenum = 0x90D2;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER_BINDING: types::GLenum = 0x90D3;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: types::GLenum = 0x90DF;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER_SIZE: types::GLenum = 0x90D5;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER_START: types::GLenum = 0x90D4;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_TYPE: types::GLenum = 0x8B4F;
#[allow(dead_code, non_upper_case_globals)] pub const SHADING_LANGUAGE_VERSION: types::GLenum = 0x8B8C;
#[allow(dead_code, non_upper_case_globals)] pub const SHORT: types::GLenum = 0x1402;
#[allow(dead_code, non_upper_case_globals)] pub const SIGNALED: types::GLenum = 0x9119;
#[allow(dead_code, non_upper_case_globals)] pub const SIGNED_NORMALIZED: types::GLenum = 0x8F9C;
#[allow(dead_code, non_upper_case_globals)] pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: types::GLenum = 0x82AC;
#[allow(dead_code, non_upper_case_globals)] pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: types::GLenum = 0x82AE;
#[allow(dead_code, non_upper_case_globals)] pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: types::GLenum = 0x82AD;
#[allow(dead_code, non_upper_case_globals)] pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: types::GLenum = 0x82AF;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_LINE_WIDTH_GRANULARITY: types::GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_LINE_WIDTH_RANGE: types::GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_POINT_SIZE_GRANULARITY: types::GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_POINT_SIZE_RANGE: types::GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)] pub const SRC1_ALPHA: types::GLenum = 0x8589;
#[allow(dead_code, non_upper_case_globals)] pub const SRC1_COLOR: types::GLenum = 0x88F9;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA: types::GLenum = 0x0302;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA_SATURATE: types::GLenum = 0x0308;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_COLOR: types::GLenum = 0x0300;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB: types::GLenum = 0x8C40;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB8: types::GLenum = 0x8C41;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB8_ALPHA8: types::GLenum = 0x8C43;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB_ALPHA: types::GLenum = 0x8C42;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB_READ: types::GLenum = 0x8297;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB_WRITE: types::GLenum = 0x8298;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_OVERFLOW: types::GLenum = 0x0503;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_UNDERFLOW: types::GLenum = 0x0504;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_COPY: types::GLenum = 0x88E6;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_DRAW: types::GLenum = 0x88E4;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_READ: types::GLenum = 0x88E5;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL: types::GLenum = 0x1802;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_ATTACHMENT: types::GLenum = 0x8D20;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_FAIL: types::GLenum = 0x8801;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_FUNC: types::GLenum = 0x8800;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_PASS_DEPTH_FAIL: types::GLenum = 0x8802;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_PASS_DEPTH_PASS: types::GLenum = 0x8803;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_REF: types::GLenum = 0x8CA3;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_VALUE_MASK: types::GLenum = 0x8CA4;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_WRITEMASK: types::GLenum = 0x8CA5;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BUFFER_BIT: types::GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_CLEAR_VALUE: types::GLenum = 0x0B91;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_COMPONENTS: types::GLenum = 0x8285;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FAIL: types::GLenum = 0x0B94;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FUNC: types::GLenum = 0x0B92;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX: types::GLenum = 0x1901;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX1: types::GLenum = 0x8D46;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX16: types::GLenum = 0x8D49;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX4: types::GLenum = 0x8D47;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX8: types::GLenum = 0x8D48;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_FAIL: types::GLenum = 0x0B95;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_PASS: types::GLenum = 0x0B96;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_REF: types::GLenum = 0x0B97;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_RENDERABLE: types::GLenum = 0x8288;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_TEST: types::GLenum = 0x0B90;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_VALUE_MASK: types::GLenum = 0x0B93;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_WRITEMASK: types::GLenum = 0x0B98;
#[allow(dead_code, non_upper_case_globals)] pub const STEREO: types::GLenum = 0x0C33;
#[allow(dead_code, non_upper_case_globals)] pub const STREAM_COPY: types::GLenum = 0x88E2;
#[allow(dead_code, non_upper_case_globals)] pub const STREAM_DRAW: types::GLenum = 0x88E0;
#[allow(dead_code, non_upper_case_globals)] pub const STREAM_READ: types::GLenum = 0x88E1;
#[allow(dead_code, non_upper_case_globals)] pub const SUBPIXEL_BITS: types::GLenum = 0x0D50;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_CONDITION: types::GLenum = 0x9113;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FENCE: types::GLenum = 0x9116;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FLAGS: types::GLenum = 0x9115;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FLUSH_COMMANDS_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_GPU_COMMANDS_COMPLETE: types::GLenum = 0x9117;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_STATUS: types::GLenum = 0x9114;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_OUTPUT_VERTICES: types::GLenum = 0x8E75;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_SHADER: types::GLenum = 0x8E88;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_SHADER_BIT: types::GLenum = 0x00000008;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_SUBROUTINE: types::GLenum = 0x92E9;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_SUBROUTINE_UNIFORM: types::GLenum = 0x92EF;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_TEXTURE: types::GLenum = 0x829C;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_SHADER: types::GLenum = 0x8E87;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_SHADER_BIT: types::GLenum = 0x00000010;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_SUBROUTINE: types::GLenum = 0x92EA;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_SUBROUTINE_UNIFORM: types::GLenum = 0x92F0;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_TEXTURE: types::GLenum = 0x829D;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_GEN_MODE: types::GLenum = 0x8E76;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_GEN_POINT_MODE: types::GLenum = 0x8E79;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_GEN_SPACING: types::GLenum = 0x8E77;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_GEN_VERTEX_ORDER: types::GLenum = 0x8E78;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE: types::GLenum = 0x1702;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE0: types::GLenum = 0x84C0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE1: types::GLenum = 0x84C1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE10: types::GLenum = 0x84CA;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE11: types::GLenum = 0x84CB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE12: types::GLenum = 0x84CC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE13: types::GLenum = 0x84CD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE14: types::GLenum = 0x84CE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE15: types::GLenum = 0x84CF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE16: types::GLenum = 0x84D0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE17: types::GLenum = 0x84D1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE18: types::GLenum = 0x84D2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE19: types::GLenum = 0x84D3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE2: types::GLenum = 0x84C2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE20: types::GLenum = 0x84D4;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE21: types::GLenum = 0x84D5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE22: types::GLenum = 0x84D6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE23: types::GLenum = 0x84D7;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE24: types::GLenum = 0x84D8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE25: types::GLenum = 0x84D9;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE26: types::GLenum = 0x84DA;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE27: types::GLenum = 0x84DB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE28: types::GLenum = 0x84DC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE29: types::GLenum = 0x84DD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE3: types::GLenum = 0x84C3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE30: types::GLenum = 0x84DE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE31: types::GLenum = 0x84DF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE4: types::GLenum = 0x84C4;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE5: types::GLenum = 0x84C5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE6: types::GLenum = 0x84C6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE7: types::GLenum = 0x84C7;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE8: types::GLenum = 0x84C8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE9: types::GLenum = 0x84C9;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_1D: types::GLenum = 0x0DE0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_1D_ARRAY: types::GLenum = 0x8C18;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D: types::GLenum = 0x0DE1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D_ARRAY: types::GLenum = 0x8C1A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D_MULTISAMPLE: types::GLenum = 0x9100;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9102;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_3D: types::GLenum = 0x806F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ALPHA_SIZE: types::GLenum = 0x805F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ALPHA_TYPE: types::GLenum = 0x8C13;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BASE_LEVEL: types::GLenum = 0x813C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_1D: types::GLenum = 0x8068;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_1D_ARRAY: types::GLenum = 0x8C1C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D: types::GLenum = 0x8069;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D_ARRAY: types::GLenum = 0x8C1D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D_MULTISAMPLE: types::GLenum = 0x9104;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9105;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_3D: types::GLenum = 0x806A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_BUFFER: types::GLenum = 0x8C2C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_CUBE_MAP: types::GLenum = 0x8514;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_CUBE_MAP_ARRAY: types::GLenum = 0x900A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_RECTANGLE: types::GLenum = 0x84F6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BLUE_SIZE: types::GLenum = 0x805E;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BLUE_TYPE: types::GLenum = 0x8C12;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BORDER_COLOR: types::GLenum = 0x1004;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER: types::GLenum = 0x8C2A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_BINDING: types::GLenum = 0x8C2A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_DATA_STORE_BINDING: types::GLenum = 0x8C2D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_OFFSET: types::GLenum = 0x919D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_OFFSET_ALIGNMENT: types::GLenum = 0x919F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_SIZE: types::GLenum = 0x919E;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPARE_FUNC: types::GLenum = 0x884D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPARE_MODE: types::GLenum = 0x884C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED: types::GLenum = 0x86A1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED_BLOCK_HEIGHT: types::GLenum = 0x82B2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED_BLOCK_SIZE: types::GLenum = 0x82B3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED_BLOCK_WIDTH: types::GLenum = 0x82B1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED_IMAGE_SIZE: types::GLenum = 0x86A0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSION_HINT: types::GLenum = 0x84EF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP: types::GLenum = 0x8513;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_ARRAY: types::GLenum = 0x9009;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_X: types::GLenum = 0x8516;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: types::GLenum = 0x8518;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: types::GLenum = 0x851A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_X: types::GLenum = 0x8515;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_Y: types::GLenum = 0x8517;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_Z: types::GLenum = 0x8519;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_SEAMLESS: types::GLenum = 0x884F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_DEPTH: types::GLenum = 0x8071;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_DEPTH_SIZE: types::GLenum = 0x884A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_DEPTH_TYPE: types::GLenum = 0x8C16;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_FETCH_BARRIER_BIT: types::GLenum = 0x00000008;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: types::GLenum = 0x9107;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GATHER: types::GLenum = 0x82A2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GATHER_SHADOW: types::GLenum = 0x82A3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GREEN_SIZE: types::GLenum = 0x805D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GREEN_TYPE: types::GLenum = 0x8C11;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_HEIGHT: types::GLenum = 0x1001;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_IMAGE_FORMAT: types::GLenum = 0x828F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_IMAGE_TYPE: types::GLenum = 0x8290;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_IMMUTABLE_FORMAT: types::GLenum = 0x912F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_IMMUTABLE_LEVELS: types::GLenum = 0x82DF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_INTERNAL_FORMAT: types::GLenum = 0x1003;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_LOD_BIAS: types::GLenum = 0x8501;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAG_FILTER: types::GLenum = 0x2800;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAX_ANISOTROPY_EXT: types::GLenum = 0x84FE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAX_LEVEL: types::GLenum = 0x813D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAX_LOD: types::GLenum = 0x813B;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MIN_FILTER: types::GLenum = 0x2801;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MIN_LOD: types::GLenum = 0x813A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RECTANGLE: types::GLenum = 0x84F5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RED_SIZE: types::GLenum = 0x805C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RED_TYPE: types::GLenum = 0x8C10;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SAMPLES: types::GLenum = 0x9106;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SHADOW: types::GLenum = 0x82A1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SHARED_SIZE: types::GLenum = 0x8C3F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_STENCIL_SIZE: types::GLenum = 0x88F1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_A: types::GLenum = 0x8E45;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_B: types::GLenum = 0x8E44;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_G: types::GLenum = 0x8E43;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_R: types::GLenum = 0x8E42;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_RGBA: types::GLenum = 0x8E46;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_TARGET: types::GLenum = 0x1006;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_UPDATE_BARRIER_BIT: types::GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_VIEW: types::GLenum = 0x82B5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_VIEW_MIN_LAYER: types::GLenum = 0x82DD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_VIEW_MIN_LEVEL: types::GLenum = 0x82DB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_VIEW_NUM_LAYERS: types::GLenum = 0x82DE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_VIEW_NUM_LEVELS: types::GLenum = 0x82DC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WIDTH: types::GLenum = 0x1000;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_R: types::GLenum = 0x8072;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_S: types::GLenum = 0x2802;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_T: types::GLenum = 0x2803;
#[allow(dead_code, non_upper_case_globals)] pub const TIMEOUT_EXPIRED: types::GLenum = 0x911B;
#[allow(dead_code, non_upper_case_globals)] pub const TIMEOUT_IGNORED: types::GLuint64 = 0xFFFFFFFFFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const TIMESTAMP: types::GLenum = 0x8E28;
#[allow(dead_code, non_upper_case_globals)] pub const TIME_ELAPSED: types::GLenum = 0x88BF;
#[allow(dead_code, non_upper_case_globals)] pub const TOP_LEVEL_ARRAY_SIZE: types::GLenum = 0x930C;
#[allow(dead_code, non_upper_case_globals)] pub const TOP_LEVEL_ARRAY_STRIDE: types::GLenum = 0x930D;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK: types::GLenum = 0x8E22;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_ACTIVE: types::GLenum = 0x8E24;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BARRIER_BIT: types::GLenum = 0x00000800;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BINDING: types::GLenum = 0x8E25;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER: types::GLenum = 0x8C8E;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_ACTIVE: types::GLenum = 0x8E24;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: types::GLenum = 0x8C8F;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_INDEX: types::GLenum = 0x934B;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_MODE: types::GLenum = 0x8C7F;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_PAUSED: types::GLenum = 0x8E23;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: types::GLenum = 0x8C85;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_START: types::GLenum = 0x8C84;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_STRIDE: types::GLenum = 0x934C;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_PAUSED: types::GLenum = 0x8E23;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: types::GLenum = 0x8C88;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_VARYING: types::GLenum = 0x92F4;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_VARYINGS: types::GLenum = 0x8C83;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: types::GLenum = 0x8C76;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLES: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLES_ADJACENCY: types::GLenum = 0x000C;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_FAN: types::GLenum = 0x0006;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_STRIP: types::GLenum = 0x0005;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_STRIP_ADJACENCY: types::GLenum = 0x000D;
#[allow(dead_code, non_upper_case_globals)] pub const TRUE: types::GLboolean = 1;
#[allow(dead_code, non_upper_case_globals)] pub const TYPE: types::GLenum = 0x92FA;
#[allow(dead_code, non_upper_case_globals)] pub const UNDEFINED_VERTEX: types::GLenum = 0x8260;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM: types::GLenum = 0x92E1;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_ARRAY_STRIDE: types::GLenum = 0x8A3C;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: types::GLenum = 0x92DA;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BARRIER_BIT: types::GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK: types::GLenum = 0x92E2;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: types::GLenum = 0x8A42;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: types::GLenum = 0x8A43;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_BINDING: types::GLenum = 0x8A3F;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_DATA_SIZE: types::GLenum = 0x8A40;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_INDEX: types::GLenum = 0x8A3A;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_NAME_LENGTH: types::GLenum = 0x8A41;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: types::GLenum = 0x90EC;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: types::GLenum = 0x8A46;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: types::GLenum = 0x8A45;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: types::GLenum = 0x84F0;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: types::GLenum = 0x84F1;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: types::GLenum = 0x8A44;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER: types::GLenum = 0x8A11;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_BINDING: types::GLenum = 0x8A28;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: types::GLenum = 0x8A34;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_SIZE: types::GLenum = 0x8A2A;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_START: types::GLenum = 0x8A29;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_IS_ROW_MAJOR: types::GLenum = 0x8A3E;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_MATRIX_STRIDE: types::GLenum = 0x8A3D;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_NAME_LENGTH: types::GLenum = 0x8A39;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_OFFSET: types::GLenum = 0x8A3B;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_SIZE: types::GLenum = 0x8A38;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_TYPE: types::GLenum = 0x8A37;
#[allow(dead_code, non_upper_case_globals)] pub const UNKNOWN_CONTEXT_RESET: types::GLenum = 0x8255;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_ALIGNMENT: types::GLenum = 0x0CF5;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_COMPRESSED_BLOCK_DEPTH: types::GLenum = 0x9129;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_COMPRESSED_BLOCK_HEIGHT: types::GLenum = 0x9128;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_COMPRESSED_BLOCK_SIZE: types::GLenum = 0x912A;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_COMPRESSED_BLOCK_WIDTH: types::GLenum = 0x9127;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_IMAGE_HEIGHT: types::GLenum = 0x806E;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_LSB_FIRST: types::GLenum = 0x0CF1;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_ROW_LENGTH: types::GLenum = 0x0CF2;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_IMAGES: types::GLenum = 0x806D;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_PIXELS: types::GLenum = 0x0CF4;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_ROWS: types::GLenum = 0x0CF3;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SWAP_BYTES: types::GLenum = 0x0CF0;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNALED: types::GLenum = 0x9118;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE: types::GLenum = 0x1401;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE_2_3_3_REV: types::GLenum = 0x8362;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE_3_3_2: types::GLenum = 0x8032;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT: types::GLenum = 0x1405;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_10F_11F_11F_REV: types::GLenum = 0x8C3B;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_10_10_10_2: types::GLenum = 0x8036;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_24_8: types::GLenum = 0x84FA;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_2_10_10_10_REV: types::GLenum = 0x8368;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_5_9_9_9_REV: types::GLenum = 0x8C3E;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_8_8_8_8: types::GLenum = 0x8035;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_8_8_8_8_REV: types::GLenum = 0x8367;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_ATOMIC_COUNTER: types::GLenum = 0x92DB;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_1D: types::GLenum = 0x9062;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_1D_ARRAY: types::GLenum = 0x9068;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D: types::GLenum = 0x9063;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D_ARRAY: types::GLenum = 0x9069;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: types::GLenum = 0x906B;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x906C;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D_RECT: types::GLenum = 0x9065;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_3D: types::GLenum = 0x9064;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_BUFFER: types::GLenum = 0x9067;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_CUBE: types::GLenum = 0x9066;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: types::GLenum = 0x906A;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_1D: types::GLenum = 0x8DD1;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: types::GLenum = 0x8DD6;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D: types::GLenum = 0x8DD2;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: types::GLenum = 0x8DD7;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x910A;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910D;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_RECT: types::GLenum = 0x8DD5;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_3D: types::GLenum = 0x8DD3;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_BUFFER: types::GLenum = 0x8DD8;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_CUBE: types::GLenum = 0x8DD4;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: types::GLenum = 0x900F;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_VEC2: types::GLenum = 0x8DC6;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_VEC3: types::GLenum = 0x8DC7;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_VEC4: types::GLenum = 0x8DC8;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_NORMALIZED: types::GLenum = 0x8C17;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT: types::GLenum = 0x1403;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_1_5_5_5_REV: types::GLenum = 0x8366;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_4_4_4_4: types::GLenum = 0x8033;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_4_4_4_4_REV: types::GLenum = 0x8365;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_5_5_1: types::GLenum = 0x8034;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_6_5: types::GLenum = 0x8363;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_6_5_REV: types::GLenum = 0x8364;
#[allow(dead_code, non_upper_case_globals)] pub const UPPER_LEFT: types::GLenum = 0x8CA2;
#[allow(dead_code, non_upper_case_globals)] pub const VALIDATE_STATUS: types::GLenum = 0x8B83;
#[allow(dead_code, non_upper_case_globals)] pub const VENDOR: types::GLenum = 0x1F00;
#[allow(dead_code, non_upper_case_globals)] pub const VERSION: types::GLenum = 0x1F02;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY: types::GLenum = 0x8074;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_BINDING: types::GLenum = 0x85B5;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: types::GLenum = 0x889F;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_DIVISOR: types::GLenum = 0x88FE;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_ENABLED: types::GLenum = 0x8622;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_INTEGER: types::GLenum = 0x88FD;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_LONG: types::GLenum = 0x874E;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: types::GLenum = 0x886A;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_POINTER: types::GLenum = 0x8645;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_SIZE: types::GLenum = 0x8623;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_STRIDE: types::GLenum = 0x8624;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_TYPE: types::GLenum = 0x8625;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_BINDING: types::GLenum = 0x82D4;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_RELATIVE_OFFSET: types::GLenum = 0x82D5;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_BINDING_BUFFER: types::GLenum = 0x8F4F;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_BINDING_DIVISOR: types::GLenum = 0x82D6;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_BINDING_OFFSET: types::GLenum = 0x82D7;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_BINDING_STRIDE: types::GLenum = 0x82D8;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_PROGRAM_POINT_SIZE: types::GLenum = 0x8642;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_SHADER: types::GLenum = 0x8B31;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_SHADER_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_SUBROUTINE: types::GLenum = 0x92E8;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_SUBROUTINE_UNIFORM: types::GLenum = 0x92EE;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_TEXTURE: types::GLenum = 0x829B;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT: types::GLenum = 0x0BA2;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT_BOUNDS_RANGE: types::GLenum = 0x825D;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT_INDEX_PROVOKING_VERTEX: types::GLenum = 0x825F;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT_SUBPIXEL_BITS: types::GLenum = 0x825C;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_128_BITS: types::GLenum = 0x82C4;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_16_BITS: types::GLenum = 0x82CA;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_24_BITS: types::GLenum = 0x82C9;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_32_BITS: types::GLenum = 0x82C8;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_48_BITS: types::GLenum = 0x82C7;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_64_BITS: types::GLenum = 0x82C6;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_8_BITS: types::GLenum = 0x82CB;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_96_BITS: types::GLenum = 0x82C5;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_BPTC_FLOAT: types::GLenum = 0x82D3;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_BPTC_UNORM: types::GLenum = 0x82D2;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_RGTC1_RED: types::GLenum = 0x82D0;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_RGTC2_RG: types::GLenum = 0x82D1;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_S3TC_DXT1_RGB: types::GLenum = 0x82CC;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_S3TC_DXT1_RGBA: types::GLenum = 0x82CD;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_S3TC_DXT3_RGBA: types::GLenum = 0x82CE;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_S3TC_DXT5_RGBA: types::GLenum = 0x82CF;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_COMPATIBILITY_CLASS: types::GLenum = 0x82B6;
#[allow(dead_code, non_upper_case_globals)] pub const WAIT_FAILED: types::GLenum = 0x911D;
#[allow(dead_code, non_upper_case_globals)] pub const WRITE_ONLY: types::GLenum = 0x88B9;
#[allow(dead_code, non_upper_case_globals)] pub const XOR: types::GLenum = 0x1506;
#[allow(dead_code, non_upper_case_globals)] pub const ZERO: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const ZERO_TO_ONE: types::GLenum = 0x935F;

        #[allow(dead_code, missing_copy_implementations)]
        #[derive(Clone)]
        pub struct FnPtr {
            /// The function pointer that will be used when calling the function.
            f: *const __gl_imports::raw::c_void,
            /// True if the pointer points to a real function, false if points to a `panic!` fn.
            is_loaded: bool,
        }

        impl FnPtr {
            /// Creates a `FnPtr` from a load attempt.
            fn new(ptr: *const __gl_imports::raw::c_void) -> FnPtr {
                if ptr.is_null() {
                    FnPtr {
                        f: missing_fn_panic as *const __gl_imports::raw::c_void,
                        is_loaded: false
                    }
                } else {
                    FnPtr { f: ptr, is_loaded: true }
                }
            }

            /// Returns `true` if the function has been successfully loaded.
            ///
            /// If it returns `false`, calling the corresponding function will fail.
            #[inline]
            #[allow(dead_code)]
            pub fn is_loaded(&self) -> bool {
                self.is_loaded
            }
        }
    
#[inline(never)]
        fn missing_fn_panic() -> ! {
            panic!("gl function was not loaded")
        }

        #[allow(non_camel_case_types, non_snake_case, dead_code)]
        #[derive(Clone)]
        pub struct Gl {
pub ActiveShaderProgram: FnPtr,
/// Fallbacks: ActiveTextureARB
pub ActiveTexture: FnPtr,
/// Fallbacks: AttachObjectARB
pub AttachShader: FnPtr,
/// Fallbacks: BeginConditionalRenderNV
pub BeginConditionalRender: FnPtr,
/// Fallbacks: BeginQueryARB
pub BeginQuery: FnPtr,
pub BeginQueryIndexed: FnPtr,
/// Fallbacks: BeginTransformFeedbackEXT, BeginTransformFeedbackNV
pub BeginTransformFeedback: FnPtr,
/// Fallbacks: BindAttribLocationARB
pub BindAttribLocation: FnPtr,
/// Fallbacks: BindBufferARB
pub BindBuffer: FnPtr,
/// Fallbacks: BindBufferBaseEXT, BindBufferBaseNV
pub BindBufferBase: FnPtr,
/// Fallbacks: BindBufferRangeEXT, BindBufferRangeNV
pub BindBufferRange: FnPtr,
pub BindBuffersBase: FnPtr,
pub BindBuffersRange: FnPtr,
/// Fallbacks: BindFragDataLocationEXT
pub BindFragDataLocation: FnPtr,
/// Fallbacks: BindFragDataLocationIndexedEXT
pub BindFragDataLocationIndexed: FnPtr,
pub BindFramebuffer: FnPtr,
pub BindImageTexture: FnPtr,
pub BindImageTextures: FnPtr,
pub BindProgramPipeline: FnPtr,
pub BindRenderbuffer: FnPtr,
pub BindSampler: FnPtr,
pub BindSamplers: FnPtr,
/// Fallbacks: BindTextureEXT
pub BindTexture: FnPtr,
pub BindTextureUnit: FnPtr,
pub BindTextures: FnPtr,
pub BindTransformFeedback: FnPtr,
/// Fallbacks: BindVertexArrayOES
pub BindVertexArray: FnPtr,
pub BindVertexBuffer: FnPtr,
pub BindVertexBuffers: FnPtr,
/// Fallbacks: BlendColorEXT
pub BlendColor: FnPtr,
/// Fallbacks: BlendEquationEXT
pub BlendEquation: FnPtr,
/// Fallbacks: BlendEquationSeparateEXT
pub BlendEquationSeparate: FnPtr,
/// Fallbacks: BlendEquationSeparateIndexedAMD, BlendEquationSeparateiARB, BlendEquationSeparateiEXT, BlendEquationSeparateiOES
pub BlendEquationSeparatei: FnPtr,
pub BlendEquationSeparateiARB: FnPtr,
/// Fallbacks: BlendEquationIndexedAMD, BlendEquationiARB, BlendEquationiEXT, BlendEquationiOES
pub BlendEquationi: FnPtr,
pub BlendEquationiARB: FnPtr,
pub BlendFunc: FnPtr,
/// Fallbacks: BlendFuncSeparateEXT, BlendFuncSeparateINGR
pub BlendFuncSeparate: FnPtr,
/// Fallbacks: BlendFuncSeparateIndexedAMD, BlendFuncSeparateiARB, BlendFuncSeparateiEXT, BlendFuncSeparateiOES
pub BlendFuncSeparatei: FnPtr,
pub BlendFuncSeparateiARB: FnPtr,
/// Fallbacks: BlendFuncIndexedAMD, BlendFunciARB, BlendFunciEXT, BlendFunciOES
pub BlendFunci: FnPtr,
pub BlendFunciARB: FnPtr,
/// Fallbacks: BlitFramebufferEXT, BlitFramebufferNV
pub BlitFramebuffer: FnPtr,
pub BlitNamedFramebuffer: FnPtr,
/// Fallbacks: BufferDataARB
pub BufferData: FnPtr,
/// Fallbacks: BufferStorageEXT
pub BufferStorage: FnPtr,
/// Fallbacks: BufferSubDataARB
pub BufferSubData: FnPtr,
/// Fallbacks: CheckFramebufferStatusEXT
pub CheckFramebufferStatus: FnPtr,
pub CheckNamedFramebufferStatus: FnPtr,
/// Fallbacks: ClampColorARB
pub ClampColor: FnPtr,
pub Clear: FnPtr,
pub ClearBufferData: FnPtr,
pub ClearBufferSubData: FnPtr,
pub ClearBufferfi: FnPtr,
pub ClearBufferfv: FnPtr,
pub ClearBufferiv: FnPtr,
pub ClearBufferuiv: FnPtr,
pub ClearColor: FnPtr,
pub ClearDepth: FnPtr,
/// Fallbacks: ClearDepthfOES
pub ClearDepthf: FnPtr,
pub ClearNamedBufferData: FnPtr,
pub ClearNamedBufferSubData: FnPtr,
pub ClearNamedFramebufferfi: FnPtr,
pub ClearNamedFramebufferfv: FnPtr,
pub ClearNamedFramebufferiv: FnPtr,
pub ClearNamedFramebufferuiv: FnPtr,
pub ClearStencil: FnPtr,
/// Fallbacks: ClearTexImageEXT
pub ClearTexImage: FnPtr,
/// Fallbacks: ClearTexSubImageEXT
pub ClearTexSubImage: FnPtr,
/// Fallbacks: ClientWaitSyncAPPLE
pub ClientWaitSync: FnPtr,
pub ClipControl: FnPtr,
pub ColorMask: FnPtr,
/// Fallbacks: ColorMaskIndexedEXT, ColorMaskiEXT, ColorMaskiOES
pub ColorMaski: FnPtr,
pub ColorP3ui: FnPtr,
pub ColorP3uiv: FnPtr,
pub ColorP4ui: FnPtr,
pub ColorP4uiv: FnPtr,
/// Fallbacks: CompileShaderARB
pub CompileShader: FnPtr,
/// Fallbacks: CompressedTexImage1DARB
pub CompressedTexImage1D: FnPtr,
/// Fallbacks: CompressedTexImage2DARB
pub CompressedTexImage2D: FnPtr,
/// Fallbacks: CompressedTexImage3DARB
pub CompressedTexImage3D: FnPtr,
/// Fallbacks: CompressedTexSubImage1DARB
pub CompressedTexSubImage1D: FnPtr,
/// Fallbacks: CompressedTexSubImage2DARB
pub CompressedTexSubImage2D: FnPtr,
/// Fallbacks: CompressedTexSubImage3DARB
pub CompressedTexSubImage3D: FnPtr,
pub CompressedTextureSubImage1D: FnPtr,
pub CompressedTextureSubImage2D: FnPtr,
pub CompressedTextureSubImage3D: FnPtr,
/// Fallbacks: CopyBufferSubDataNV
pub CopyBufferSubData: FnPtr,
/// Fallbacks: CopyImageSubDataEXT, CopyImageSubDataOES
pub CopyImageSubData: FnPtr,
pub CopyNamedBufferSubData: FnPtr,
/// Fallbacks: CopyTexImage1DEXT
pub CopyTexImage1D: FnPtr,
/// Fallbacks: CopyTexImage2DEXT
pub CopyTexImage2D: FnPtr,
/// Fallbacks: CopyTexSubImage1DEXT
pub CopyTexSubImage1D: FnPtr,
/// Fallbacks: CopyTexSubImage2DEXT
pub CopyTexSubImage2D: FnPtr,
/// Fallbacks: CopyTexSubImage3DEXT
pub CopyTexSubImage3D: FnPtr,
pub CopyTextureSubImage1D: FnPtr,
pub CopyTextureSubImage2D: FnPtr,
pub CopyTextureSubImage3D: FnPtr,
pub CreateBuffers: FnPtr,
pub CreateFramebuffers: FnPtr,
/// Fallbacks: CreateProgramObjectARB
pub CreateProgram: FnPtr,
pub CreateProgramPipelines: FnPtr,
pub CreateQueries: FnPtr,
pub CreateRenderbuffers: FnPtr,
pub CreateSamplers: FnPtr,
/// Fallbacks: CreateShaderObjectARB
pub CreateShader: FnPtr,
pub CreateShaderProgramv: FnPtr,
pub CreateTextures: FnPtr,
pub CreateTransformFeedbacks: FnPtr,
pub CreateVertexArrays: FnPtr,
pub CullFace: FnPtr,
/// Fallbacks: DebugMessageCallbackARB, DebugMessageCallbackKHR
pub DebugMessageCallback: FnPtr,
/// Fallbacks: DebugMessageControlARB, DebugMessageControlKHR
pub DebugMessageControl: FnPtr,
/// Fallbacks: DebugMessageInsertARB, DebugMessageInsertKHR
pub DebugMessageInsert: FnPtr,
/// Fallbacks: DeleteBuffersARB
pub DeleteBuffers: FnPtr,
/// Fallbacks: DeleteFramebuffersEXT
pub DeleteFramebuffers: FnPtr,
pub DeleteProgram: FnPtr,
pub DeleteProgramPipelines: FnPtr,
/// Fallbacks: DeleteQueriesARB
pub DeleteQueries: FnPtr,
/// Fallbacks: DeleteRenderbuffersEXT
pub DeleteRenderbuffers: FnPtr,
pub DeleteSamplers: FnPtr,
pub DeleteShader: FnPtr,
/// Fallbacks: DeleteSyncAPPLE
pub DeleteSync: FnPtr,
pub DeleteTextures: FnPtr,
/// Fallbacks: DeleteTransformFeedbacksNV
pub DeleteTransformFeedbacks: FnPtr,
/// Fallbacks: DeleteVertexArraysAPPLE, DeleteVertexArraysOES
pub DeleteVertexArrays: FnPtr,
pub DepthFunc: FnPtr,
pub DepthMask: FnPtr,
pub DepthRange: FnPtr,
pub DepthRangeArrayv: FnPtr,
pub DepthRangeIndexed: FnPtr,
/// Fallbacks: DepthRangefOES
pub DepthRangef: FnPtr,
/// Fallbacks: DetachObjectARB
pub DetachShader: FnPtr,
pub Disable: FnPtr,
pub DisableVertexArrayAttrib: FnPtr,
/// Fallbacks: DisableVertexAttribArrayARB
pub DisableVertexAttribArray: FnPtr,
/// Fallbacks: DisableIndexedEXT, DisableiEXT, DisableiNV, DisableiOES
pub Disablei: FnPtr,
pub DispatchCompute: FnPtr,
pub DispatchComputeIndirect: FnPtr,
/// Fallbacks: DrawArraysEXT
pub DrawArrays: FnPtr,
pub DrawArraysIndirect: FnPtr,
/// Fallbacks: DrawArraysInstancedANGLE, DrawArraysInstancedARB, DrawArraysInstancedEXT, DrawArraysInstancedNV
pub DrawArraysInstanced: FnPtr,
/// Fallbacks: DrawArraysInstancedBaseInstanceEXT
pub DrawArraysInstancedBaseInstance: FnPtr,
pub DrawBuffer: FnPtr,
/// Fallbacks: DrawBuffersARB, DrawBuffersATI, DrawBuffersEXT
pub DrawBuffers: FnPtr,
pub DrawElements: FnPtr,
/// Fallbacks: DrawElementsBaseVertexEXT, DrawElementsBaseVertexOES
pub DrawElementsBaseVertex: FnPtr,
pub DrawElementsIndirect: FnPtr,
/// Fallbacks: DrawElementsInstancedANGLE, DrawElementsInstancedARB, DrawElementsInstancedEXT, DrawElementsInstancedNV
pub DrawElementsInstanced: FnPtr,
/// Fallbacks: DrawElementsInstancedBaseInstanceEXT
pub DrawElementsInstancedBaseInstance: FnPtr,
/// Fallbacks: DrawElementsInstancedBaseVertexEXT, DrawElementsInstancedBaseVertexOES
pub DrawElementsInstancedBaseVertex: FnPtr,
/// Fallbacks: DrawElementsInstancedBaseVertexBaseInstanceEXT
pub DrawElementsInstancedBaseVertexBaseInstance: FnPtr,
/// Fallbacks: DrawRangeElementsEXT
pub DrawRangeElements: FnPtr,
/// Fallbacks: DrawRangeElementsBaseVertexEXT, DrawRangeElementsBaseVertexOES
pub DrawRangeElementsBaseVertex: FnPtr,
/// Fallbacks: DrawTransformFeedbackEXT, DrawTransformFeedbackNV
pub DrawTransformFeedback: FnPtr,
/// Fallbacks: DrawTransformFeedbackInstancedEXT
pub DrawTransformFeedbackInstanced: FnPtr,
pub DrawTransformFeedbackStream: FnPtr,
pub DrawTransformFeedbackStreamInstanced: FnPtr,
pub Enable: FnPtr,
pub EnableVertexArrayAttrib: FnPtr,
/// Fallbacks: EnableVertexAttribArrayARB
pub EnableVertexAttribArray: FnPtr,
/// Fallbacks: EnableIndexedEXT, EnableiEXT, EnableiNV, EnableiOES
pub Enablei: FnPtr,
/// Fallbacks: EndConditionalRenderNV, EndConditionalRenderNVX
pub EndConditionalRender: FnPtr,
/// Fallbacks: EndQueryARB
pub EndQuery: FnPtr,
pub EndQueryIndexed: FnPtr,
/// Fallbacks: EndTransformFeedbackEXT, EndTransformFeedbackNV
pub EndTransformFeedback: FnPtr,
/// Fallbacks: FenceSyncAPPLE
pub FenceSync: FnPtr,
pub Finish: FnPtr,
pub Flush: FnPtr,
/// Fallbacks: FlushMappedBufferRangeAPPLE, FlushMappedBufferRangeEXT
pub FlushMappedBufferRange: FnPtr,
pub FlushMappedNamedBufferRange: FnPtr,
pub FramebufferParameteri: FnPtr,
/// Fallbacks: FramebufferRenderbufferEXT
pub FramebufferRenderbuffer: FnPtr,
/// Fallbacks: FramebufferTextureARB, FramebufferTextureEXT, FramebufferTextureOES
pub FramebufferTexture: FnPtr,
/// Fallbacks: FramebufferTexture1DEXT
pub FramebufferTexture1D: FnPtr,
/// Fallbacks: FramebufferTexture2DEXT
pub FramebufferTexture2D: FnPtr,
/// Fallbacks: FramebufferTexture3DEXT
pub FramebufferTexture3D: FnPtr,
/// Fallbacks: FramebufferTextureLayerARB, FramebufferTextureLayerEXT
pub FramebufferTextureLayer: FnPtr,
pub FrontFace: FnPtr,
/// Fallbacks: GenBuffersARB
pub GenBuffers: FnPtr,
/// Fallbacks: GenFramebuffersEXT
pub GenFramebuffers: FnPtr,
pub GenProgramPipelines: FnPtr,
/// Fallbacks: GenQueriesARB
pub GenQueries: FnPtr,
/// Fallbacks: GenRenderbuffersEXT
pub GenRenderbuffers: FnPtr,
pub GenSamplers: FnPtr,
pub GenTextures: FnPtr,
/// Fallbacks: GenTransformFeedbacksNV
pub GenTransformFeedbacks: FnPtr,
/// Fallbacks: GenVertexArraysAPPLE, GenVertexArraysOES
pub GenVertexArrays: FnPtr,
/// Fallbacks: GenerateMipmapEXT
pub GenerateMipmap: FnPtr,
pub GenerateTextureMipmap: FnPtr,
pub GetActiveAtomicCounterBufferiv: FnPtr,
/// Fallbacks: GetActiveAttribARB
pub GetActiveAttrib: FnPtr,
pub GetActiveSubroutineName: FnPtr,
pub GetActiveSubroutineUniformName: FnPtr,
pub GetActiveSubroutineUniformiv: FnPtr,
/// Fallbacks: GetActiveUniformARB
pub GetActiveUniform: FnPtr,
pub GetActiveUniformBlockName: FnPtr,
pub GetActiveUniformBlockiv: FnPtr,
pub GetActiveUniformName: FnPtr,
pub GetActiveUniformsiv: FnPtr,
pub GetAttachedShaders: FnPtr,
/// Fallbacks: GetAttribLocationARB
pub GetAttribLocation: FnPtr,
/// Fallbacks: GetBooleanIndexedvEXT
pub GetBooleani_v: FnPtr,
pub GetBooleanv: FnPtr,
pub GetBufferParameteri64v: FnPtr,
/// Fallbacks: GetBufferParameterivARB
pub GetBufferParameteriv: FnPtr,
/// Fallbacks: GetBufferPointervARB, GetBufferPointervOES
pub GetBufferPointerv: FnPtr,
/// Fallbacks: GetBufferSubDataARB
pub GetBufferSubData: FnPtr,
/// Fallbacks: GetCompressedTexImageARB
pub GetCompressedTexImage: FnPtr,
pub GetCompressedTextureImage: FnPtr,
pub GetCompressedTextureSubImage: FnPtr,
/// Fallbacks: GetDebugMessageLogARB, GetDebugMessageLogKHR
pub GetDebugMessageLog: FnPtr,
/// Fallbacks: GetDoubleIndexedvEXT, GetDoublei_vEXT
pub GetDoublei_v: FnPtr,
pub GetDoublev: FnPtr,
pub GetError: FnPtr,
/// Fallbacks: GetFloatIndexedvEXT, GetFloati_vEXT, GetFloati_vNV, GetFloati_vOES
pub GetFloati_v: FnPtr,
pub GetFloatv: FnPtr,
/// Fallbacks: GetFragDataIndexEXT
pub GetFragDataIndex: FnPtr,
/// Fallbacks: GetFragDataLocationEXT
pub GetFragDataLocation: FnPtr,
/// Fallbacks: GetFramebufferAttachmentParameterivEXT
pub GetFramebufferAttachmentParameteriv: FnPtr,
pub GetFramebufferParameteriv: FnPtr,
/// Fallbacks: GetGraphicsResetStatusKHR
pub GetGraphicsResetStatus: FnPtr,
pub GetInteger64i_v: FnPtr,
/// Fallbacks: GetInteger64vAPPLE
pub GetInteger64v: FnPtr,
/// Fallbacks: GetIntegerIndexedvEXT
pub GetIntegeri_v: FnPtr,
pub GetIntegerv: FnPtr,
pub GetInternalformati64v: FnPtr,
pub GetInternalformativ: FnPtr,
/// Fallbacks: GetMultisamplefvNV
pub GetMultisamplefv: FnPtr,
pub GetNamedBufferParameteri64v: FnPtr,
pub GetNamedBufferParameteriv: FnPtr,
pub GetNamedBufferPointerv: FnPtr,
pub GetNamedBufferSubData: FnPtr,
pub GetNamedFramebufferAttachmentParameteriv: FnPtr,
pub GetNamedFramebufferParameteriv: FnPtr,
pub GetNamedRenderbufferParameteriv: FnPtr,
/// Fallbacks: GetObjectLabelKHR
pub GetObjectLabel: FnPtr,
/// Fallbacks: GetObjectPtrLabelKHR
pub GetObjectPtrLabel: FnPtr,
/// Fallbacks: GetPointervEXT, GetPointervKHR
pub GetPointerv: FnPtr,
/// Fallbacks: GetProgramBinaryOES
pub GetProgramBinary: FnPtr,
pub GetProgramInfoLog: FnPtr,
pub GetProgramInterfaceiv: FnPtr,
pub GetProgramPipelineInfoLog: FnPtr,
pub GetProgramPipelineiv: FnPtr,
pub GetProgramResourceIndex: FnPtr,
pub GetProgramResourceLocation: FnPtr,
pub GetProgramResourceLocationIndex: FnPtr,
pub GetProgramResourceName: FnPtr,
pub GetProgramResourceiv: FnPtr,
pub GetProgramStageiv: FnPtr,
pub GetProgramiv: FnPtr,
pub GetQueryBufferObjecti64v: FnPtr,
pub GetQueryBufferObjectiv: FnPtr,
pub GetQueryBufferObjectui64v: FnPtr,
pub GetQueryBufferObjectuiv: FnPtr,
pub GetQueryIndexediv: FnPtr,
/// Fallbacks: GetQueryObjecti64vEXT
pub GetQueryObjecti64v: FnPtr,
/// Fallbacks: GetQueryObjectivARB, GetQueryObjectivEXT
pub GetQueryObjectiv: FnPtr,
/// Fallbacks: GetQueryObjectui64vEXT
pub GetQueryObjectui64v: FnPtr,
/// Fallbacks: GetQueryObjectuivARB
pub GetQueryObjectuiv: FnPtr,
/// Fallbacks: GetQueryivARB
pub GetQueryiv: FnPtr,
/// Fallbacks: GetRenderbufferParameterivEXT
pub GetRenderbufferParameteriv: FnPtr,
/// Fallbacks: GetSamplerParameterIivEXT, GetSamplerParameterIivOES
pub GetSamplerParameterIiv: FnPtr,
/// Fallbacks: GetSamplerParameterIuivEXT, GetSamplerParameterIuivOES
pub GetSamplerParameterIuiv: FnPtr,
pub GetSamplerParameterfv: FnPtr,
pub GetSamplerParameteriv: FnPtr,
pub GetShaderInfoLog: FnPtr,
pub GetShaderPrecisionFormat: FnPtr,
/// Fallbacks: GetShaderSourceARB
pub GetShaderSource: FnPtr,
pub GetShaderiv: FnPtr,
pub GetString: FnPtr,
pub GetStringi: FnPtr,
pub GetSubroutineIndex: FnPtr,
pub GetSubroutineUniformLocation: FnPtr,
/// Fallbacks: GetSyncivAPPLE
pub GetSynciv: FnPtr,
pub GetTexImage: FnPtr,
pub GetTexLevelParameterfv: FnPtr,
pub GetTexLevelParameteriv: FnPtr,
/// Fallbacks: GetTexParameterIivEXT, GetTexParameterIivOES
pub GetTexParameterIiv: FnPtr,
/// Fallbacks: GetTexParameterIuivEXT, GetTexParameterIuivOES
pub GetTexParameterIuiv: FnPtr,
pub GetTexParameterfv: FnPtr,
pub GetTexParameteriv: FnPtr,
pub GetTextureImage: FnPtr,
pub GetTextureLevelParameterfv: FnPtr,
pub GetTextureLevelParameteriv: FnPtr,
pub GetTextureParameterIiv: FnPtr,
pub GetTextureParameterIuiv: FnPtr,
pub GetTextureParameterfv: FnPtr,
pub GetTextureParameteriv: FnPtr,
pub GetTextureSubImage: FnPtr,
/// Fallbacks: GetTransformFeedbackVaryingEXT
pub GetTransformFeedbackVarying: FnPtr,
pub GetTransformFeedbacki64_v: FnPtr,
pub GetTransformFeedbacki_v: FnPtr,
pub GetTransformFeedbackiv: FnPtr,
pub GetUniformBlockIndex: FnPtr,
pub GetUniformIndices: FnPtr,
/// Fallbacks: GetUniformLocationARB
pub GetUniformLocation: FnPtr,
pub GetUniformSubroutineuiv: FnPtr,
pub GetUniformdv: FnPtr,
/// Fallbacks: GetUniformfvARB
pub GetUniformfv: FnPtr,
/// Fallbacks: GetUniformivARB
pub GetUniformiv: FnPtr,
/// Fallbacks: GetUniformuivEXT
pub GetUniformuiv: FnPtr,
pub GetVertexArrayIndexed64iv: FnPtr,
pub GetVertexArrayIndexediv: FnPtr,
pub GetVertexArrayiv: FnPtr,
/// Fallbacks: GetVertexAttribIivEXT
pub GetVertexAttribIiv: FnPtr,
/// Fallbacks: GetVertexAttribIuivEXT
pub GetVertexAttribIuiv: FnPtr,
/// Fallbacks: GetVertexAttribLdvEXT
pub GetVertexAttribLdv: FnPtr,
/// Fallbacks: GetVertexAttribPointervARB, GetVertexAttribPointervNV
pub GetVertexAttribPointerv: FnPtr,
/// Fallbacks: GetVertexAttribdvARB, GetVertexAttribdvNV
pub GetVertexAttribdv: FnPtr,
/// Fallbacks: GetVertexAttribfvARB, GetVertexAttribfvNV
pub GetVertexAttribfv: FnPtr,
/// Fallbacks: GetVertexAttribivARB, GetVertexAttribivNV
pub GetVertexAttribiv: FnPtr,
pub GetnColorTable: FnPtr,
pub GetnCompressedTexImage: FnPtr,
pub GetnConvolutionFilter: FnPtr,
pub GetnHistogram: FnPtr,
pub GetnMapdv: FnPtr,
pub GetnMapfv: FnPtr,
pub GetnMapiv: FnPtr,
pub GetnMinmax: FnPtr,
pub GetnPixelMapfv: FnPtr,
pub GetnPixelMapuiv: FnPtr,
pub GetnPixelMapusv: FnPtr,
pub GetnPolygonStipple: FnPtr,
pub GetnSeparableFilter: FnPtr,
pub GetnTexImage: FnPtr,
pub GetnUniformdv: FnPtr,
/// Fallbacks: GetnUniformfvEXT, GetnUniformfvKHR
pub GetnUniformfv: FnPtr,
/// Fallbacks: GetnUniformivEXT, GetnUniformivKHR
pub GetnUniformiv: FnPtr,
/// Fallbacks: GetnUniformuivKHR
pub GetnUniformuiv: FnPtr,
pub Hint: FnPtr,
pub InvalidateBufferData: FnPtr,
pub InvalidateBufferSubData: FnPtr,
pub InvalidateFramebuffer: FnPtr,
pub InvalidateNamedFramebufferData: FnPtr,
pub InvalidateNamedFramebufferSubData: FnPtr,
pub InvalidateSubFramebuffer: FnPtr,
pub InvalidateTexImage: FnPtr,
pub InvalidateTexSubImage: FnPtr,
/// Fallbacks: IsBufferARB
pub IsBuffer: FnPtr,
pub IsEnabled: FnPtr,
/// Fallbacks: IsEnabledIndexedEXT, IsEnablediEXT, IsEnablediNV, IsEnablediOES
pub IsEnabledi: FnPtr,
/// Fallbacks: IsFramebufferEXT
pub IsFramebuffer: FnPtr,
pub IsProgram: FnPtr,
pub IsProgramPipeline: FnPtr,
/// Fallbacks: IsQueryARB
pub IsQuery: FnPtr,
/// Fallbacks: IsRenderbufferEXT
pub IsRenderbuffer: FnPtr,
pub IsSampler: FnPtr,
pub IsShader: FnPtr,
/// Fallbacks: IsSyncAPPLE
pub IsSync: FnPtr,
pub IsTexture: FnPtr,
/// Fallbacks: IsTransformFeedbackNV
pub IsTransformFeedback: FnPtr,
/// Fallbacks: IsVertexArrayAPPLE, IsVertexArrayOES
pub IsVertexArray: FnPtr,
pub LineWidth: FnPtr,
/// Fallbacks: LinkProgramARB
pub LinkProgram: FnPtr,
pub LogicOp: FnPtr,
/// Fallbacks: MapBufferARB, MapBufferOES
pub MapBuffer: FnPtr,
/// Fallbacks: MapBufferRangeEXT
pub MapBufferRange: FnPtr,
pub MapNamedBuffer: FnPtr,
pub MapNamedBufferRange: FnPtr,
/// Fallbacks: MemoryBarrierEXT
pub MemoryBarrier: FnPtr,
pub MemoryBarrierByRegion: FnPtr,
/// Fallbacks: MinSampleShadingARB, MinSampleShadingOES
pub MinSampleShading: FnPtr,
/// Fallbacks: MultiDrawArraysEXT
pub MultiDrawArrays: FnPtr,
/// Fallbacks: MultiDrawArraysIndirectAMD, MultiDrawArraysIndirectEXT
pub MultiDrawArraysIndirect: FnPtr,
/// Fallbacks: MultiDrawElementsEXT
pub MultiDrawElements: FnPtr,
/// Fallbacks: MultiDrawElementsBaseVertexEXT
pub MultiDrawElementsBaseVertex: FnPtr,
/// Fallbacks: MultiDrawElementsIndirectAMD, MultiDrawElementsIndirectEXT
pub MultiDrawElementsIndirect: FnPtr,
pub MultiTexCoordP1ui: FnPtr,
pub MultiTexCoordP1uiv: FnPtr,
pub MultiTexCoordP2ui: FnPtr,
pub MultiTexCoordP2uiv: FnPtr,
pub MultiTexCoordP3ui: FnPtr,
pub MultiTexCoordP3uiv: FnPtr,
pub MultiTexCoordP4ui: FnPtr,
pub MultiTexCoordP4uiv: FnPtr,
pub NamedBufferData: FnPtr,
/// Fallbacks: NamedBufferStorageEXT
pub NamedBufferStorage: FnPtr,
/// Fallbacks: NamedBufferSubDataEXT
pub NamedBufferSubData: FnPtr,
pub NamedFramebufferDrawBuffer: FnPtr,
pub NamedFramebufferDrawBuffers: FnPtr,
pub NamedFramebufferParameteri: FnPtr,
pub NamedFramebufferReadBuffer: FnPtr,
pub NamedFramebufferRenderbuffer: FnPtr,
pub NamedFramebufferTexture: FnPtr,
pub NamedFramebufferTextureLayer: FnPtr,
pub NamedRenderbufferStorage: FnPtr,
pub NamedRenderbufferStorageMultisample: FnPtr,
pub NormalP3ui: FnPtr,
pub NormalP3uiv: FnPtr,
/// Fallbacks: ObjectLabelKHR
pub ObjectLabel: FnPtr,
/// Fallbacks: ObjectPtrLabelKHR
pub ObjectPtrLabel: FnPtr,
pub PatchParameterfv: FnPtr,
/// Fallbacks: PatchParameteriEXT, PatchParameteriOES
pub PatchParameteri: FnPtr,
/// Fallbacks: PauseTransformFeedbackNV
pub PauseTransformFeedback: FnPtr,
pub PixelStoref: FnPtr,
pub PixelStorei: FnPtr,
/// Fallbacks: PointParameterfARB, PointParameterfEXT, PointParameterfSGIS
pub PointParameterf: FnPtr,
/// Fallbacks: PointParameterfvARB, PointParameterfvEXT, PointParameterfvSGIS
pub PointParameterfv: FnPtr,
/// Fallbacks: PointParameteriNV
pub PointParameteri: FnPtr,
/// Fallbacks: PointParameterivNV
pub PointParameteriv: FnPtr,
pub PointSize: FnPtr,
/// Fallbacks: PolygonModeNV
pub PolygonMode: FnPtr,
pub PolygonOffset: FnPtr,
/// Fallbacks: PopDebugGroupKHR
pub PopDebugGroup: FnPtr,
pub PrimitiveRestartIndex: FnPtr,
/// Fallbacks: ProgramBinaryOES
pub ProgramBinary: FnPtr,
/// Fallbacks: ProgramParameteriARB, ProgramParameteriEXT
pub ProgramParameteri: FnPtr,
pub ProgramUniform1d: FnPtr,
pub ProgramUniform1dv: FnPtr,
/// Fallbacks: ProgramUniform1fEXT
pub ProgramUniform1f: FnPtr,
/// Fallbacks: ProgramUniform1fvEXT
pub ProgramUniform1fv: FnPtr,
/// Fallbacks: ProgramUniform1iEXT
pub ProgramUniform1i: FnPtr,
/// Fallbacks: ProgramUniform1ivEXT
pub ProgramUniform1iv: FnPtr,
/// Fallbacks: ProgramUniform1uiEXT
pub ProgramUniform1ui: FnPtr,
/// Fallbacks: ProgramUniform1uivEXT
pub ProgramUniform1uiv: FnPtr,
pub ProgramUniform2d: FnPtr,
pub ProgramUniform2dv: FnPtr,
/// Fallbacks: ProgramUniform2fEXT
pub ProgramUniform2f: FnPtr,
/// Fallbacks: ProgramUniform2fvEXT
pub ProgramUniform2fv: FnPtr,
/// Fallbacks: ProgramUniform2iEXT
pub ProgramUniform2i: FnPtr,
/// Fallbacks: ProgramUniform2ivEXT
pub ProgramUniform2iv: FnPtr,
/// Fallbacks: ProgramUniform2uiEXT
pub ProgramUniform2ui: FnPtr,
/// Fallbacks: ProgramUniform2uivEXT
pub ProgramUniform2uiv: FnPtr,
pub ProgramUniform3d: FnPtr,
pub ProgramUniform3dv: FnPtr,
/// Fallbacks: ProgramUniform3fEXT
pub ProgramUniform3f: FnPtr,
/// Fallbacks: ProgramUniform3fvEXT
pub ProgramUniform3fv: FnPtr,
/// Fallbacks: ProgramUniform3iEXT
pub ProgramUniform3i: FnPtr,
/// Fallbacks: ProgramUniform3ivEXT
pub ProgramUniform3iv: FnPtr,
/// Fallbacks: ProgramUniform3uiEXT
pub ProgramUniform3ui: FnPtr,
/// Fallbacks: ProgramUniform3uivEXT
pub ProgramUniform3uiv: FnPtr,
pub ProgramUniform4d: FnPtr,
pub ProgramUniform4dv: FnPtr,
/// Fallbacks: ProgramUniform4fEXT
pub ProgramUniform4f: FnPtr,
/// Fallbacks: ProgramUniform4fvEXT
pub ProgramUniform4fv: FnPtr,
/// Fallbacks: ProgramUniform4iEXT
pub ProgramUniform4i: FnPtr,
/// Fallbacks: ProgramUniform4ivEXT
pub ProgramUniform4iv: FnPtr,
/// Fallbacks: ProgramUniform4uiEXT
pub ProgramUniform4ui: FnPtr,
/// Fallbacks: ProgramUniform4uivEXT
pub ProgramUniform4uiv: FnPtr,
pub ProgramUniformMatrix2dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix2fvEXT
pub ProgramUniformMatrix2fv: FnPtr,
pub ProgramUniformMatrix2x3dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix2x3fvEXT
pub ProgramUniformMatrix2x3fv: FnPtr,
pub ProgramUniformMatrix2x4dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix2x4fvEXT
pub ProgramUniformMatrix2x4fv: FnPtr,
pub ProgramUniformMatrix3dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix3fvEXT
pub ProgramUniformMatrix3fv: FnPtr,
pub ProgramUniformMatrix3x2dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix3x2fvEXT
pub ProgramUniformMatrix3x2fv: FnPtr,
pub ProgramUniformMatrix3x4dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix3x4fvEXT
pub ProgramUniformMatrix3x4fv: FnPtr,
pub ProgramUniformMatrix4dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix4fvEXT
pub ProgramUniformMatrix4fv: FnPtr,
pub ProgramUniformMatrix4x2dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix4x2fvEXT
pub ProgramUniformMatrix4x2fv: FnPtr,
pub ProgramUniformMatrix4x3dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix4x3fvEXT
pub ProgramUniformMatrix4x3fv: FnPtr,
/// Fallbacks: ProvokingVertexEXT
pub ProvokingVertex: FnPtr,
/// Fallbacks: PushDebugGroupKHR
pub PushDebugGroup: FnPtr,
/// Fallbacks: QueryCounterEXT
pub QueryCounter: FnPtr,
pub ReadBuffer: FnPtr,
pub ReadPixels: FnPtr,
/// Fallbacks: ReadnPixelsARB, ReadnPixelsEXT, ReadnPixelsKHR
pub ReadnPixels: FnPtr,
pub ReleaseShaderCompiler: FnPtr,
/// Fallbacks: RenderbufferStorageEXT
pub RenderbufferStorage: FnPtr,
/// Fallbacks: RenderbufferStorageMultisampleEXT, RenderbufferStorageMultisampleNV
pub RenderbufferStorageMultisample: FnPtr,
/// Fallbacks: ResumeTransformFeedbackNV
pub ResumeTransformFeedback: FnPtr,
/// Fallbacks: SampleCoverageARB
pub SampleCoverage: FnPtr,
pub SampleMaski: FnPtr,
/// Fallbacks: SamplerParameterIivEXT, SamplerParameterIivOES
pub SamplerParameterIiv: FnPtr,
/// Fallbacks: SamplerParameterIuivEXT, SamplerParameterIuivOES
pub SamplerParameterIuiv: FnPtr,
pub SamplerParameterf: FnPtr,
pub SamplerParameterfv: FnPtr,
pub SamplerParameteri: FnPtr,
pub SamplerParameteriv: FnPtr,
pub Scissor: FnPtr,
/// Fallbacks: ScissorArrayvNV, ScissorArrayvOES
pub ScissorArrayv: FnPtr,
/// Fallbacks: ScissorIndexedNV, ScissorIndexedOES
pub ScissorIndexed: FnPtr,
/// Fallbacks: ScissorIndexedvNV, ScissorIndexedvOES
pub ScissorIndexedv: FnPtr,
pub SecondaryColorP3ui: FnPtr,
pub SecondaryColorP3uiv: FnPtr,
pub ShaderBinary: FnPtr,
/// Fallbacks: ShaderSourceARB
pub ShaderSource: FnPtr,
pub ShaderStorageBlockBinding: FnPtr,
pub StencilFunc: FnPtr,
pub StencilFuncSeparate: FnPtr,
pub StencilMask: FnPtr,
pub StencilMaskSeparate: FnPtr,
pub StencilOp: FnPtr,
/// Fallbacks: StencilOpSeparateATI
pub StencilOpSeparate: FnPtr,
/// Fallbacks: TexBufferARB, TexBufferEXT, TexBufferOES
pub TexBuffer: FnPtr,
/// Fallbacks: TexBufferRangeEXT, TexBufferRangeOES
pub TexBufferRange: FnPtr,
pub TexCoordP1ui: FnPtr,
pub TexCoordP1uiv: FnPtr,
pub TexCoordP2ui: FnPtr,
pub TexCoordP2uiv: FnPtr,
pub TexCoordP3ui: FnPtr,
pub TexCoordP3uiv: FnPtr,
pub TexCoordP4ui: FnPtr,
pub TexCoordP4uiv: FnPtr,
pub TexImage1D: FnPtr,
pub TexImage2D: FnPtr,
pub TexImage2DMultisample: FnPtr,
/// Fallbacks: TexImage3DEXT
pub TexImage3D: FnPtr,
pub TexImage3DMultisample: FnPtr,
/// Fallbacks: TexParameterIivEXT, TexParameterIivOES
pub TexParameterIiv: FnPtr,
/// Fallbacks: TexParameterIuivEXT, TexParameterIuivOES
pub TexParameterIuiv: FnPtr,
pub TexParameterf: FnPtr,
pub TexParameterfv: FnPtr,
pub TexParameteri: FnPtr,
pub TexParameteriv: FnPtr,
/// Fallbacks: TexStorage1DEXT
pub TexStorage1D: FnPtr,
/// Fallbacks: TexStorage2DEXT
pub TexStorage2D: FnPtr,
pub TexStorage2DMultisample: FnPtr,
/// Fallbacks: TexStorage3DEXT
pub TexStorage3D: FnPtr,
/// Fallbacks: TexStorage3DMultisampleOES
pub TexStorage3DMultisample: FnPtr,
/// Fallbacks: TexSubImage1DEXT
pub TexSubImage1D: FnPtr,
/// Fallbacks: TexSubImage2DEXT
pub TexSubImage2D: FnPtr,
/// Fallbacks: TexSubImage3DEXT
pub TexSubImage3D: FnPtr,
pub TextureBarrier: FnPtr,
pub TextureBuffer: FnPtr,
pub TextureBufferRange: FnPtr,
pub TextureParameterIiv: FnPtr,
pub TextureParameterIuiv: FnPtr,
pub TextureParameterf: FnPtr,
pub TextureParameterfv: FnPtr,
pub TextureParameteri: FnPtr,
pub TextureParameteriv: FnPtr,
pub TextureStorage1D: FnPtr,
pub TextureStorage2D: FnPtr,
pub TextureStorage2DMultisample: FnPtr,
pub TextureStorage3D: FnPtr,
pub TextureStorage3DMultisample: FnPtr,
pub TextureSubImage1D: FnPtr,
pub TextureSubImage2D: FnPtr,
pub TextureSubImage3D: FnPtr,
/// Fallbacks: TextureViewEXT, TextureViewOES
pub TextureView: FnPtr,
pub TransformFeedbackBufferBase: FnPtr,
pub TransformFeedbackBufferRange: FnPtr,
/// Fallbacks: TransformFeedbackVaryingsEXT
pub TransformFeedbackVaryings: FnPtr,
pub Uniform1d: FnPtr,
pub Uniform1dv: FnPtr,
/// Fallbacks: Uniform1fARB
pub Uniform1f: FnPtr,
/// Fallbacks: Uniform1fvARB
pub Uniform1fv: FnPtr,
/// Fallbacks: Uniform1iARB
pub Uniform1i: FnPtr,
/// Fallbacks: Uniform1ivARB
pub Uniform1iv: FnPtr,
/// Fallbacks: Uniform1uiEXT
pub Uniform1ui: FnPtr,
/// Fallbacks: Uniform1uivEXT
pub Uniform1uiv: FnPtr,
pub Uniform2d: FnPtr,
pub Uniform2dv: FnPtr,
/// Fallbacks: Uniform2fARB
pub Uniform2f: FnPtr,
/// Fallbacks: Uniform2fvARB
pub Uniform2fv: FnPtr,
/// Fallbacks: Uniform2iARB
pub Uniform2i: FnPtr,
/// Fallbacks: Uniform2ivARB
pub Uniform2iv: FnPtr,
/// Fallbacks: Uniform2uiEXT
pub Uniform2ui: FnPtr,
/// Fallbacks: Uniform2uivEXT
pub Uniform2uiv: FnPtr,
pub Uniform3d: FnPtr,
pub Uniform3dv: FnPtr,
/// Fallbacks: Uniform3fARB
pub Uniform3f: FnPtr,
/// Fallbacks: Uniform3fvARB
pub Uniform3fv: FnPtr,
/// Fallbacks: Uniform3iARB
pub Uniform3i: FnPtr,
/// Fallbacks: Uniform3ivARB
pub Uniform3iv: FnPtr,
/// Fallbacks: Uniform3uiEXT
pub Uniform3ui: FnPtr,
/// Fallbacks: Uniform3uivEXT
pub Uniform3uiv: FnPtr,
pub Uniform4d: FnPtr,
pub Uniform4dv: FnPtr,
/// Fallbacks: Uniform4fARB
pub Uniform4f: FnPtr,
/// Fallbacks: Uniform4fvARB
pub Uniform4fv: FnPtr,
/// Fallbacks: Uniform4iARB
pub Uniform4i: FnPtr,
/// Fallbacks: Uniform4ivARB
pub Uniform4iv: FnPtr,
/// Fallbacks: Uniform4uiEXT
pub Uniform4ui: FnPtr,
/// Fallbacks: Uniform4uivEXT
pub Uniform4uiv: FnPtr,
pub UniformBlockBinding: FnPtr,
pub UniformMatrix2dv: FnPtr,
/// Fallbacks: UniformMatrix2fvARB
pub UniformMatrix2fv: FnPtr,
pub UniformMatrix2x3dv: FnPtr,
/// Fallbacks: UniformMatrix2x3fvNV
pub UniformMatrix2x3fv: FnPtr,
pub UniformMatrix2x4dv: FnPtr,
/// Fallbacks: UniformMatrix2x4fvNV
pub UniformMatrix2x4fv: FnPtr,
pub UniformMatrix3dv: FnPtr,
/// Fallbacks: UniformMatrix3fvARB
pub UniformMatrix3fv: FnPtr,
pub UniformMatrix3x2dv: FnPtr,
/// Fallbacks: UniformMatrix3x2fvNV
pub UniformMatrix3x2fv: FnPtr,
pub UniformMatrix3x4dv: FnPtr,
/// Fallbacks: UniformMatrix3x4fvNV
pub UniformMatrix3x4fv: FnPtr,
pub UniformMatrix4dv: FnPtr,
/// Fallbacks: UniformMatrix4fvARB
pub UniformMatrix4fv: FnPtr,
pub UniformMatrix4x2dv: FnPtr,
/// Fallbacks: UniformMatrix4x2fvNV
pub UniformMatrix4x2fv: FnPtr,
pub UniformMatrix4x3dv: FnPtr,
/// Fallbacks: UniformMatrix4x3fvNV
pub UniformMatrix4x3fv: FnPtr,
pub UniformSubroutinesuiv: FnPtr,
/// Fallbacks: UnmapBufferARB, UnmapBufferOES
pub UnmapBuffer: FnPtr,
pub UnmapNamedBuffer: FnPtr,
/// Fallbacks: UseProgramObjectARB
pub UseProgram: FnPtr,
pub UseProgramStages: FnPtr,
/// Fallbacks: ValidateProgramARB
pub ValidateProgram: FnPtr,
pub ValidateProgramPipeline: FnPtr,
pub VertexArrayAttribBinding: FnPtr,
pub VertexArrayAttribFormat: FnPtr,
pub VertexArrayAttribIFormat: FnPtr,
pub VertexArrayAttribLFormat: FnPtr,
pub VertexArrayBindingDivisor: FnPtr,
pub VertexArrayElementBuffer: FnPtr,
pub VertexArrayVertexBuffer: FnPtr,
pub VertexArrayVertexBuffers: FnPtr,
/// Fallbacks: VertexAttrib1dARB, VertexAttrib1dNV
pub VertexAttrib1d: FnPtr,
/// Fallbacks: VertexAttrib1dvARB, VertexAttrib1dvNV
pub VertexAttrib1dv: FnPtr,
/// Fallbacks: VertexAttrib1fARB, VertexAttrib1fNV
pub VertexAttrib1f: FnPtr,
/// Fallbacks: VertexAttrib1fvARB, VertexAttrib1fvNV
pub VertexAttrib1fv: FnPtr,
/// Fallbacks: VertexAttrib1sARB, VertexAttrib1sNV
pub VertexAttrib1s: FnPtr,
/// Fallbacks: VertexAttrib1svARB, VertexAttrib1svNV
pub VertexAttrib1sv: FnPtr,
/// Fallbacks: VertexAttrib2dARB, VertexAttrib2dNV
pub VertexAttrib2d: FnPtr,
/// Fallbacks: VertexAttrib2dvARB, VertexAttrib2dvNV
pub VertexAttrib2dv: FnPtr,
/// Fallbacks: VertexAttrib2fARB, VertexAttrib2fNV
pub VertexAttrib2f: FnPtr,
/// Fallbacks: VertexAttrib2fvARB, VertexAttrib2fvNV
pub VertexAttrib2fv: FnPtr,
/// Fallbacks: VertexAttrib2sARB, VertexAttrib2sNV
pub VertexAttrib2s: FnPtr,
/// Fallbacks: VertexAttrib2svARB, VertexAttrib2svNV
pub VertexAttrib2sv: FnPtr,
/// Fallbacks: VertexAttrib3dARB, VertexAttrib3dNV
pub VertexAttrib3d: FnPtr,
/// Fallbacks: VertexAttrib3dvARB, VertexAttrib3dvNV
pub VertexAttrib3dv: FnPtr,
/// Fallbacks: VertexAttrib3fARB, VertexAttrib3fNV
pub VertexAttrib3f: FnPtr,
/// Fallbacks: VertexAttrib3fvARB, VertexAttrib3fvNV
pub VertexAttrib3fv: FnPtr,
/// Fallbacks: VertexAttrib3sARB, VertexAttrib3sNV
pub VertexAttrib3s: FnPtr,
/// Fallbacks: VertexAttrib3svARB, VertexAttrib3svNV
pub VertexAttrib3sv: FnPtr,
/// Fallbacks: VertexAttrib4NbvARB
pub VertexAttrib4Nbv: FnPtr,
/// Fallbacks: VertexAttrib4NivARB
pub VertexAttrib4Niv: FnPtr,
/// Fallbacks: VertexAttrib4NsvARB
pub VertexAttrib4Nsv: FnPtr,
/// Fallbacks: VertexAttrib4NubARB, VertexAttrib4ubNV
pub VertexAttrib4Nub: FnPtr,
/// Fallbacks: VertexAttrib4NubvARB, VertexAttrib4ubvNV
pub VertexAttrib4Nubv: FnPtr,
/// Fallbacks: VertexAttrib4NuivARB
pub VertexAttrib4Nuiv: FnPtr,
/// Fallbacks: VertexAttrib4NusvARB
pub VertexAttrib4Nusv: FnPtr,
/// Fallbacks: VertexAttrib4bvARB
pub VertexAttrib4bv: FnPtr,
/// Fallbacks: VertexAttrib4dARB, VertexAttrib4dNV
pub VertexAttrib4d: FnPtr,
/// Fallbacks: VertexAttrib4dvARB, VertexAttrib4dvNV
pub VertexAttrib4dv: FnPtr,
/// Fallbacks: VertexAttrib4fARB, VertexAttrib4fNV
pub VertexAttrib4f: FnPtr,
/// Fallbacks: VertexAttrib4fvARB, VertexAttrib4fvNV
pub VertexAttrib4fv: FnPtr,
/// Fallbacks: VertexAttrib4ivARB
pub VertexAttrib4iv: FnPtr,
/// Fallbacks: VertexAttrib4sARB, VertexAttrib4sNV
pub VertexAttrib4s: FnPtr,
/// Fallbacks: VertexAttrib4svARB, VertexAttrib4svNV
pub VertexAttrib4sv: FnPtr,
/// Fallbacks: VertexAttrib4ubvARB
pub VertexAttrib4ubv: FnPtr,
/// Fallbacks: VertexAttrib4uivARB
pub VertexAttrib4uiv: FnPtr,
/// Fallbacks: VertexAttrib4usvARB
pub VertexAttrib4usv: FnPtr,
pub VertexAttribBinding: FnPtr,
/// Fallbacks: VertexAttribDivisorANGLE, VertexAttribDivisorARB, VertexAttribDivisorEXT, VertexAttribDivisorNV
pub VertexAttribDivisor: FnPtr,
pub VertexAttribFormat: FnPtr,
/// Fallbacks: VertexAttribI1iEXT
pub VertexAttribI1i: FnPtr,
/// Fallbacks: VertexAttribI1ivEXT
pub VertexAttribI1iv: FnPtr,
/// Fallbacks: VertexAttribI1uiEXT
pub VertexAttribI1ui: FnPtr,
/// Fallbacks: VertexAttribI1uivEXT
pub VertexAttribI1uiv: FnPtr,
/// Fallbacks: VertexAttribI2iEXT
pub VertexAttribI2i: FnPtr,
/// Fallbacks: VertexAttribI2ivEXT
pub VertexAttribI2iv: FnPtr,
/// Fallbacks: VertexAttribI2uiEXT
pub VertexAttribI2ui: FnPtr,
/// Fallbacks: VertexAttribI2uivEXT
pub VertexAttribI2uiv: FnPtr,
/// Fallbacks: VertexAttribI3iEXT
pub VertexAttribI3i: FnPtr,
/// Fallbacks: VertexAttribI3ivEXT
pub VertexAttribI3iv: FnPtr,
/// Fallbacks: VertexAttribI3uiEXT
pub VertexAttribI3ui: FnPtr,
/// Fallbacks: VertexAttribI3uivEXT
pub VertexAttribI3uiv: FnPtr,
/// Fallbacks: VertexAttribI4bvEXT
pub VertexAttribI4bv: FnPtr,
/// Fallbacks: VertexAttribI4iEXT
pub VertexAttribI4i: FnPtr,
/// Fallbacks: VertexAttribI4ivEXT
pub VertexAttribI4iv: FnPtr,
/// Fallbacks: VertexAttribI4svEXT
pub VertexAttribI4sv: FnPtr,
/// Fallbacks: VertexAttribI4ubvEXT
pub VertexAttribI4ubv: FnPtr,
/// Fallbacks: VertexAttribI4uiEXT
pub VertexAttribI4ui: FnPtr,
/// Fallbacks: VertexAttribI4uivEXT
pub VertexAttribI4uiv: FnPtr,
/// Fallbacks: VertexAttribI4usvEXT
pub VertexAttribI4usv: FnPtr,
pub VertexAttribIFormat: FnPtr,
/// Fallbacks: VertexAttribIPointerEXT
pub VertexAttribIPointer: FnPtr,
/// Fallbacks: VertexAttribL1dEXT
pub VertexAttribL1d: FnPtr,
/// Fallbacks: VertexAttribL1dvEXT
pub VertexAttribL1dv: FnPtr,
/// Fallbacks: VertexAttribL2dEXT
pub VertexAttribL2d: FnPtr,
/// Fallbacks: VertexAttribL2dvEXT
pub VertexAttribL2dv: FnPtr,
/// Fallbacks: VertexAttribL3dEXT
pub VertexAttribL3d: FnPtr,
/// Fallbacks: VertexAttribL3dvEXT
pub VertexAttribL3dv: FnPtr,
/// Fallbacks: VertexAttribL4dEXT
pub VertexAttribL4d: FnPtr,
/// Fallbacks: VertexAttribL4dvEXT
pub VertexAttribL4dv: FnPtr,
pub VertexAttribLFormat: FnPtr,
/// Fallbacks: VertexAttribLPointerEXT
pub VertexAttribLPointer: FnPtr,
pub VertexAttribP1ui: FnPtr,
pub VertexAttribP1uiv: FnPtr,
pub VertexAttribP2ui: FnPtr,
pub VertexAttribP2uiv: FnPtr,
pub VertexAttribP3ui: FnPtr,
pub VertexAttribP3uiv: FnPtr,
pub VertexAttribP4ui: FnPtr,
pub VertexAttribP4uiv: FnPtr,
/// Fallbacks: VertexAttribPointerARB
pub VertexAttribPointer: FnPtr,
pub VertexBindingDivisor: FnPtr,
pub VertexP2ui: FnPtr,
pub VertexP2uiv: FnPtr,
pub VertexP3ui: FnPtr,
pub VertexP3uiv: FnPtr,
pub VertexP4ui: FnPtr,
pub VertexP4uiv: FnPtr,
pub Viewport: FnPtr,
/// Fallbacks: ViewportArrayvNV, ViewportArrayvOES
pub ViewportArrayv: FnPtr,
/// Fallbacks: ViewportIndexedfOES, ViewportIndexedfNV
pub ViewportIndexedf: FnPtr,
/// Fallbacks: ViewportIndexedfvOES, ViewportIndexedfvNV
pub ViewportIndexedfv: FnPtr,
/// Fallbacks: WaitSyncAPPLE
pub WaitSync: FnPtr,
_priv: ()
}
impl Gl {
            /// Load each OpenGL symbol using a custom load function. This allows for the
            /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
            ///
            /// ~~~ignore
            /// let gl = Gl::load_with(|s| glfw.get_proc_address(s));
            /// ~~~
            #[allow(dead_code, unused_variables)]
            pub fn load_with<F>(mut loadfn: F) -> Gl where F: FnMut(&'static str) -> *const __gl_imports::raw::c_void {
                #[inline(never)]
                fn do_metaloadfn(loadfn: &mut FnMut(&'static str) -> *const __gl_imports::raw::c_void,
                                 symbol: &'static str,
                                 symbols: &[&'static str])
                                 -> *const __gl_imports::raw::c_void {
                    let mut ptr = loadfn(symbol);
                    if ptr.is_null() {
                        for &sym in symbols {
                            ptr = loadfn(sym);
                            if !ptr.is_null() { break; }
                        }
                    }
                    ptr
                }
                let mut metaloadfn = |symbol: &'static str, symbols: &[&'static str]| {
                    do_metaloadfn(&mut loadfn, symbol, symbols)
                };
                Gl {
ActiveShaderProgram: FnPtr::new(metaloadfn("glActiveShaderProgram", &[])),
ActiveTexture: FnPtr::new(metaloadfn("glActiveTexture", &["glActiveTextureARB"])),
AttachShader: FnPtr::new(metaloadfn("glAttachShader", &["glAttachObjectARB"])),
BeginConditionalRender: FnPtr::new(metaloadfn("glBeginConditionalRender", &["glBeginConditionalRenderNV"])),
BeginQuery: FnPtr::new(metaloadfn("glBeginQuery", &["glBeginQueryARB"])),
BeginQueryIndexed: FnPtr::new(metaloadfn("glBeginQueryIndexed", &[])),
BeginTransformFeedback: FnPtr::new(metaloadfn("glBeginTransformFeedback", &["glBeginTransformFeedbackEXT", "glBeginTransformFeedbackNV"])),
BindAttribLocation: FnPtr::new(metaloadfn("glBindAttribLocation", &["glBindAttribLocationARB"])),
BindBuffer: FnPtr::new(metaloadfn("glBindBuffer", &["glBindBufferARB"])),
BindBufferBase: FnPtr::new(metaloadfn("glBindBufferBase", &["glBindBufferBaseEXT", "glBindBufferBaseNV"])),
BindBufferRange: FnPtr::new(metaloadfn("glBindBufferRange", &["glBindBufferRangeEXT", "glBindBufferRangeNV"])),
BindBuffersBase: FnPtr::new(metaloadfn("glBindBuffersBase", &[])),
BindBuffersRange: FnPtr::new(metaloadfn("glBindBuffersRange", &[])),
BindFragDataLocation: FnPtr::new(metaloadfn("glBindFragDataLocation", &["glBindFragDataLocationEXT"])),
BindFragDataLocationIndexed: FnPtr::new(metaloadfn("glBindFragDataLocationIndexed", &["glBindFragDataLocationIndexedEXT"])),
BindFramebuffer: FnPtr::new(metaloadfn("glBindFramebuffer", &[])),
BindImageTexture: FnPtr::new(metaloadfn("glBindImageTexture", &[])),
BindImageTextures: FnPtr::new(metaloadfn("glBindImageTextures", &[])),
BindProgramPipeline: FnPtr::new(metaloadfn("glBindProgramPipeline", &[])),
BindRenderbuffer: FnPtr::new(metaloadfn("glBindRenderbuffer", &[])),
BindSampler: FnPtr::new(metaloadfn("glBindSampler", &[])),
BindSamplers: FnPtr::new(metaloadfn("glBindSamplers", &[])),
BindTexture: FnPtr::new(metaloadfn("glBindTexture", &["glBindTextureEXT"])),
BindTextureUnit: FnPtr::new(metaloadfn("glBindTextureUnit", &[])),
BindTextures: FnPtr::new(metaloadfn("glBindTextures", &[])),
BindTransformFeedback: FnPtr::new(metaloadfn("glBindTransformFeedback", &[])),
BindVertexArray: FnPtr::new(metaloadfn("glBindVertexArray", &["glBindVertexArrayOES"])),
BindVertexBuffer: FnPtr::new(metaloadfn("glBindVertexBuffer", &[])),
BindVertexBuffers: FnPtr::new(metaloadfn("glBindVertexBuffers", &[])),
BlendColor: FnPtr::new(metaloadfn("glBlendColor", &["glBlendColorEXT"])),
BlendEquation: FnPtr::new(metaloadfn("glBlendEquation", &["glBlendEquationEXT"])),
BlendEquationSeparate: FnPtr::new(metaloadfn("glBlendEquationSeparate", &["glBlendEquationSeparateEXT"])),
BlendEquationSeparatei: FnPtr::new(metaloadfn("glBlendEquationSeparatei", &["glBlendEquationSeparateIndexedAMD", "glBlendEquationSeparateiARB", "glBlendEquationSeparateiEXT", "glBlendEquationSeparateiOES"])),
BlendEquationSeparateiARB: FnPtr::new(metaloadfn("glBlendEquationSeparateiARB", &[])),
BlendEquationi: FnPtr::new(metaloadfn("glBlendEquationi", &["glBlendEquationIndexedAMD", "glBlendEquationiARB", "glBlendEquationiEXT", "glBlendEquationiOES"])),
BlendEquationiARB: FnPtr::new(metaloadfn("glBlendEquationiARB", &[])),
BlendFunc: FnPtr::new(metaloadfn("glBlendFunc", &[])),
BlendFuncSeparate: FnPtr::new(metaloadfn("glBlendFuncSeparate", &["glBlendFuncSeparateEXT", "glBlendFuncSeparateINGR"])),
BlendFuncSeparatei: FnPtr::new(metaloadfn("glBlendFuncSeparatei", &["glBlendFuncSeparateIndexedAMD", "glBlendFuncSeparateiARB", "glBlendFuncSeparateiEXT", "glBlendFuncSeparateiOES"])),
BlendFuncSeparateiARB: FnPtr::new(metaloadfn("glBlendFuncSeparateiARB", &[])),
BlendFunci: FnPtr::new(metaloadfn("glBlendFunci", &["glBlendFuncIndexedAMD", "glBlendFunciARB", "glBlendFunciEXT", "glBlendFunciOES"])),
BlendFunciARB: FnPtr::new(metaloadfn("glBlendFunciARB", &[])),
BlitFramebuffer: FnPtr::new(metaloadfn("glBlitFramebuffer", &["glBlitFramebufferEXT", "glBlitFramebufferNV"])),
BlitNamedFramebuffer: FnPtr::new(metaloadfn("glBlitNamedFramebuffer", &[])),
BufferData: FnPtr::new(metaloadfn("glBufferData", &["glBufferDataARB"])),
BufferStorage: FnPtr::new(metaloadfn("glBufferStorage", &["glBufferStorageEXT"])),
BufferSubData: FnPtr::new(metaloadfn("glBufferSubData", &["glBufferSubDataARB"])),
CheckFramebufferStatus: FnPtr::new(metaloadfn("glCheckFramebufferStatus", &["glCheckFramebufferStatusEXT"])),
CheckNamedFramebufferStatus: FnPtr::new(metaloadfn("glCheckNamedFramebufferStatus", &[])),
ClampColor: FnPtr::new(metaloadfn("glClampColor", &["glClampColorARB"])),
Clear: FnPtr::new(metaloadfn("glClear", &[])),
ClearBufferData: FnPtr::new(metaloadfn("glClearBufferData", &[])),
ClearBufferSubData: FnPtr::new(metaloadfn("glClearBufferSubData", &[])),
ClearBufferfi: FnPtr::new(metaloadfn("glClearBufferfi", &[])),
ClearBufferfv: FnPtr::new(metaloadfn("glClearBufferfv", &[])),
ClearBufferiv: FnPtr::new(metaloadfn("glClearBufferiv", &[])),
ClearBufferuiv: FnPtr::new(metaloadfn("glClearBufferuiv", &[])),
ClearColor: FnPtr::new(metaloadfn("glClearColor", &[])),
ClearDepth: FnPtr::new(metaloadfn("glClearDepth", &[])),
ClearDepthf: FnPtr::new(metaloadfn("glClearDepthf", &["glClearDepthfOES"])),
ClearNamedBufferData: FnPtr::new(metaloadfn("glClearNamedBufferData", &[])),
ClearNamedBufferSubData: FnPtr::new(metaloadfn("glClearNamedBufferSubData", &[])),
ClearNamedFramebufferfi: FnPtr::new(metaloadfn("glClearNamedFramebufferfi", &[])),
ClearNamedFramebufferfv: FnPtr::new(metaloadfn("glClearNamedFramebufferfv", &[])),
ClearNamedFramebufferiv: FnPtr::new(metaloadfn("glClearNamedFramebufferiv", &[])),
ClearNamedFramebufferuiv: FnPtr::new(metaloadfn("glClearNamedFramebufferuiv", &[])),
ClearStencil: FnPtr::new(metaloadfn("glClearStencil", &[])),
ClearTexImage: FnPtr::new(metaloadfn("glClearTexImage", &["glClearTexImageEXT"])),
ClearTexSubImage: FnPtr::new(metaloadfn("glClearTexSubImage", &["glClearTexSubImageEXT"])),
ClientWaitSync: FnPtr::new(metaloadfn("glClientWaitSync", &["glClientWaitSyncAPPLE"])),
ClipControl: FnPtr::new(metaloadfn("glClipControl", &[])),
ColorMask: FnPtr::new(metaloadfn("glColorMask", &[])),
ColorMaski: FnPtr::new(metaloadfn("glColorMaski", &["glColorMaskIndexedEXT", "glColorMaskiEXT", "glColorMaskiOES"])),
ColorP3ui: FnPtr::new(metaloadfn("glColorP3ui", &[])),
ColorP3uiv: FnPtr::new(metaloadfn("glColorP3uiv", &[])),
ColorP4ui: FnPtr::new(metaloadfn("glColorP4ui", &[])),
ColorP4uiv: FnPtr::new(metaloadfn("glColorP4uiv", &[])),
CompileShader: FnPtr::new(metaloadfn("glCompileShader", &["glCompileShaderARB"])),
CompressedTexImage1D: FnPtr::new(metaloadfn("glCompressedTexImage1D", &["glCompressedTexImage1DARB"])),
CompressedTexImage2D: FnPtr::new(metaloadfn("glCompressedTexImage2D", &["glCompressedTexImage2DARB"])),
CompressedTexImage3D: FnPtr::new(metaloadfn("glCompressedTexImage3D", &["glCompressedTexImage3DARB"])),
CompressedTexSubImage1D: FnPtr::new(metaloadfn("glCompressedTexSubImage1D", &["glCompressedTexSubImage1DARB"])),
CompressedTexSubImage2D: FnPtr::new(metaloadfn("glCompressedTexSubImage2D", &["glCompressedTexSubImage2DARB"])),
CompressedTexSubImage3D: FnPtr::new(metaloadfn("glCompressedTexSubImage3D", &["glCompressedTexSubImage3DARB"])),
CompressedTextureSubImage1D: FnPtr::new(metaloadfn("glCompressedTextureSubImage1D", &[])),
CompressedTextureSubImage2D: FnPtr::new(metaloadfn("glCompressedTextureSubImage2D", &[])),
CompressedTextureSubImage3D: FnPtr::new(metaloadfn("glCompressedTextureSubImage3D", &[])),
CopyBufferSubData: FnPtr::new(metaloadfn("glCopyBufferSubData", &["glCopyBufferSubDataNV"])),
CopyImageSubData: FnPtr::new(metaloadfn("glCopyImageSubData", &["glCopyImageSubDataEXT", "glCopyImageSubDataOES"])),
CopyNamedBufferSubData: FnPtr::new(metaloadfn("glCopyNamedBufferSubData", &[])),
CopyTexImage1D: FnPtr::new(metaloadfn("glCopyTexImage1D", &["glCopyTexImage1DEXT"])),
CopyTexImage2D: FnPtr::new(metaloadfn("glCopyTexImage2D", &["glCopyTexImage2DEXT"])),
CopyTexSubImage1D: FnPtr::new(metaloadfn("glCopyTexSubImage1D", &["glCopyTexSubImage1DEXT"])),
CopyTexSubImage2D: FnPtr::new(metaloadfn("glCopyTexSubImage2D", &["glCopyTexSubImage2DEXT"])),
CopyTexSubImage3D: FnPtr::new(metaloadfn("glCopyTexSubImage3D", &["glCopyTexSubImage3DEXT"])),
CopyTextureSubImage1D: FnPtr::new(metaloadfn("glCopyTextureSubImage1D", &[])),
CopyTextureSubImage2D: FnPtr::new(metaloadfn("glCopyTextureSubImage2D", &[])),
CopyTextureSubImage3D: FnPtr::new(metaloadfn("glCopyTextureSubImage3D", &[])),
CreateBuffers: FnPtr::new(metaloadfn("glCreateBuffers", &[])),
CreateFramebuffers: FnPtr::new(metaloadfn("glCreateFramebuffers", &[])),
CreateProgram: FnPtr::new(metaloadfn("glCreateProgram", &["glCreateProgramObjectARB"])),
CreateProgramPipelines: FnPtr::new(metaloadfn("glCreateProgramPipelines", &[])),
CreateQueries: FnPtr::new(metaloadfn("glCreateQueries", &[])),
CreateRenderbuffers: FnPtr::new(metaloadfn("glCreateRenderbuffers", &[])),
CreateSamplers: FnPtr::new(metaloadfn("glCreateSamplers", &[])),
CreateShader: FnPtr::new(metaloadfn("glCreateShader", &["glCreateShaderObjectARB"])),
CreateShaderProgramv: FnPtr::new(metaloadfn("glCreateShaderProgramv", &[])),
CreateTextures: FnPtr::new(metaloadfn("glCreateTextures", &[])),
CreateTransformFeedbacks: FnPtr::new(metaloadfn("glCreateTransformFeedbacks", &[])),
CreateVertexArrays: FnPtr::new(metaloadfn("glCreateVertexArrays", &[])),
CullFace: FnPtr::new(metaloadfn("glCullFace", &[])),
DebugMessageCallback: FnPtr::new(metaloadfn("glDebugMessageCallback", &["glDebugMessageCallbackARB", "glDebugMessageCallbackKHR"])),
DebugMessageControl: FnPtr::new(metaloadfn("glDebugMessageControl", &["glDebugMessageControlARB", "glDebugMessageControlKHR"])),
DebugMessageInsert: FnPtr::new(metaloadfn("glDebugMessageInsert", &["glDebugMessageInsertARB", "glDebugMessageInsertKHR"])),
DeleteBuffers: FnPtr::new(metaloadfn("glDeleteBuffers", &["glDeleteBuffersARB"])),
DeleteFramebuffers: FnPtr::new(metaloadfn("glDeleteFramebuffers", &["glDeleteFramebuffersEXT"])),
DeleteProgram: FnPtr::new(metaloadfn("glDeleteProgram", &[])),
DeleteProgramPipelines: FnPtr::new(metaloadfn("glDeleteProgramPipelines", &[])),
DeleteQueries: FnPtr::new(metaloadfn("glDeleteQueries", &["glDeleteQueriesARB"])),
DeleteRenderbuffers: FnPtr::new(metaloadfn("glDeleteRenderbuffers", &["glDeleteRenderbuffersEXT"])),
DeleteSamplers: FnPtr::new(metaloadfn("glDeleteSamplers", &[])),
DeleteShader: FnPtr::new(metaloadfn("glDeleteShader", &[])),
DeleteSync: FnPtr::new(metaloadfn("glDeleteSync", &["glDeleteSyncAPPLE"])),
DeleteTextures: FnPtr::new(metaloadfn("glDeleteTextures", &[])),
DeleteTransformFeedbacks: FnPtr::new(metaloadfn("glDeleteTransformFeedbacks", &["glDeleteTransformFeedbacksNV"])),
DeleteVertexArrays: FnPtr::new(metaloadfn("glDeleteVertexArrays", &["glDeleteVertexArraysAPPLE", "glDeleteVertexArraysOES"])),
DepthFunc: FnPtr::new(metaloadfn("glDepthFunc", &[])),
DepthMask: FnPtr::new(metaloadfn("glDepthMask", &[])),
DepthRange: FnPtr::new(metaloadfn("glDepthRange", &[])),
DepthRangeArrayv: FnPtr::new(metaloadfn("glDepthRangeArrayv", &[])),
DepthRangeIndexed: FnPtr::new(metaloadfn("glDepthRangeIndexed", &[])),
DepthRangef: FnPtr::new(metaloadfn("glDepthRangef", &["glDepthRangefOES"])),
DetachShader: FnPtr::new(metaloadfn("glDetachShader", &["glDetachObjectARB"])),
Disable: FnPtr::new(metaloadfn("glDisable", &[])),
DisableVertexArrayAttrib: FnPtr::new(metaloadfn("glDisableVertexArrayAttrib", &[])),
DisableVertexAttribArray: FnPtr::new(metaloadfn("glDisableVertexAttribArray", &["glDisableVertexAttribArrayARB"])),
Disablei: FnPtr::new(metaloadfn("glDisablei", &["glDisableIndexedEXT", "glDisableiEXT", "glDisableiNV", "glDisableiOES"])),
DispatchCompute: FnPtr::new(metaloadfn("glDispatchCompute", &[])),
DispatchComputeIndirect: FnPtr::new(metaloadfn("glDispatchComputeIndirect", &[])),
DrawArrays: FnPtr::new(metaloadfn("glDrawArrays", &["glDrawArraysEXT"])),
DrawArraysIndirect: FnPtr::new(metaloadfn("glDrawArraysIndirect", &[])),
DrawArraysInstanced: FnPtr::new(metaloadfn("glDrawArraysInstanced", &["glDrawArraysInstancedANGLE", "glDrawArraysInstancedARB", "glDrawArraysInstancedEXT", "glDrawArraysInstancedNV"])),
DrawArraysInstancedBaseInstance: FnPtr::new(metaloadfn("glDrawArraysInstancedBaseInstance", &["glDrawArraysInstancedBaseInstanceEXT"])),
DrawBuffer: FnPtr::new(metaloadfn("glDrawBuffer", &[])),
DrawBuffers: FnPtr::new(metaloadfn("glDrawBuffers", &["glDrawBuffersARB", "glDrawBuffersATI", "glDrawBuffersEXT"])),
DrawElements: FnPtr::new(metaloadfn("glDrawElements", &[])),
DrawElementsBaseVertex: FnPtr::new(metaloadfn("glDrawElementsBaseVertex", &["glDrawElementsBaseVertexEXT", "glDrawElementsBaseVertexOES"])),
DrawElementsIndirect: FnPtr::new(metaloadfn("glDrawElementsIndirect", &[])),
DrawElementsInstanced: FnPtr::new(metaloadfn("glDrawElementsInstanced", &["glDrawElementsInstancedANGLE", "glDrawElementsInstancedARB", "glDrawElementsInstancedEXT", "glDrawElementsInstancedNV"])),
DrawElementsInstancedBaseInstance: FnPtr::new(metaloadfn("glDrawElementsInstancedBaseInstance", &["glDrawElementsInstancedBaseInstanceEXT"])),
DrawElementsInstancedBaseVertex: FnPtr::new(metaloadfn("glDrawElementsInstancedBaseVertex", &["glDrawElementsInstancedBaseVertexEXT", "glDrawElementsInstancedBaseVertexOES"])),
DrawElementsInstancedBaseVertexBaseInstance: FnPtr::new(metaloadfn("glDrawElementsInstancedBaseVertexBaseInstance", &["glDrawElementsInstancedBaseVertexBaseInstanceEXT"])),
DrawRangeElements: FnPtr::new(metaloadfn("glDrawRangeElements", &["glDrawRangeElementsEXT"])),
DrawRangeElementsBaseVertex: FnPtr::new(metaloadfn("glDrawRangeElementsBaseVertex", &["glDrawRangeElementsBaseVertexEXT", "glDrawRangeElementsBaseVertexOES"])),
DrawTransformFeedback: FnPtr::new(metaloadfn("glDrawTransformFeedback", &["glDrawTransformFeedbackEXT", "glDrawTransformFeedbackNV"])),
DrawTransformFeedbackInstanced: FnPtr::new(metaloadfn("glDrawTransformFeedbackInstanced", &["glDrawTransformFeedbackInstancedEXT"])),
DrawTransformFeedbackStream: FnPtr::new(metaloadfn("glDrawTransformFeedbackStream", &[])),
DrawTransformFeedbackStreamInstanced: FnPtr::new(metaloadfn("glDrawTransformFeedbackStreamInstanced", &[])),
Enable: FnPtr::new(metaloadfn("glEnable", &[])),
EnableVertexArrayAttrib: FnPtr::new(metaloadfn("glEnableVertexArrayAttrib", &[])),
EnableVertexAttribArray: FnPtr::new(metaloadfn("glEnableVertexAttribArray", &["glEnableVertexAttribArrayARB"])),
Enablei: FnPtr::new(metaloadfn("glEnablei", &["glEnableIndexedEXT", "glEnableiEXT", "glEnableiNV", "glEnableiOES"])),
EndConditionalRender: FnPtr::new(metaloadfn("glEndConditionalRender", &["glEndConditionalRenderNV", "glEndConditionalRenderNVX"])),
EndQuery: FnPtr::new(metaloadfn("glEndQuery", &["glEndQueryARB"])),
EndQueryIndexed: FnPtr::new(metaloadfn("glEndQueryIndexed", &[])),
EndTransformFeedback: FnPtr::new(metaloadfn("glEndTransformFeedback", &["glEndTransformFeedbackEXT", "glEndTransformFeedbackNV"])),
FenceSync: FnPtr::new(metaloadfn("glFenceSync", &["glFenceSyncAPPLE"])),
Finish: FnPtr::new(metaloadfn("glFinish", &[])),
Flush: FnPtr::new(metaloadfn("glFlush", &[])),
FlushMappedBufferRange: FnPtr::new(metaloadfn("glFlushMappedBufferRange", &["glFlushMappedBufferRangeAPPLE", "glFlushMappedBufferRangeEXT"])),
FlushMappedNamedBufferRange: FnPtr::new(metaloadfn("glFlushMappedNamedBufferRange", &[])),
FramebufferParameteri: FnPtr::new(metaloadfn("glFramebufferParameteri", &[])),
FramebufferRenderbuffer: FnPtr::new(metaloadfn("glFramebufferRenderbuffer", &["glFramebufferRenderbufferEXT"])),
FramebufferTexture: FnPtr::new(metaloadfn("glFramebufferTexture", &["glFramebufferTextureARB", "glFramebufferTextureEXT", "glFramebufferTextureOES"])),
FramebufferTexture1D: FnPtr::new(metaloadfn("glFramebufferTexture1D", &["glFramebufferTexture1DEXT"])),
FramebufferTexture2D: FnPtr::new(metaloadfn("glFramebufferTexture2D", &["glFramebufferTexture2DEXT"])),
FramebufferTexture3D: FnPtr::new(metaloadfn("glFramebufferTexture3D", &["glFramebufferTexture3DEXT"])),
FramebufferTextureLayer: FnPtr::new(metaloadfn("glFramebufferTextureLayer", &["glFramebufferTextureLayerARB", "glFramebufferTextureLayerEXT"])),
FrontFace: FnPtr::new(metaloadfn("glFrontFace", &[])),
GenBuffers: FnPtr::new(metaloadfn("glGenBuffers", &["glGenBuffersARB"])),
GenFramebuffers: FnPtr::new(metaloadfn("glGenFramebuffers", &["glGenFramebuffersEXT"])),
GenProgramPipelines: FnPtr::new(metaloadfn("glGenProgramPipelines", &[])),
GenQueries: FnPtr::new(metaloadfn("glGenQueries", &["glGenQueriesARB"])),
GenRenderbuffers: FnPtr::new(metaloadfn("glGenRenderbuffers", &["glGenRenderbuffersEXT"])),
GenSamplers: FnPtr::new(metaloadfn("glGenSamplers", &[])),
GenTextures: FnPtr::new(metaloadfn("glGenTextures", &[])),
GenTransformFeedbacks: FnPtr::new(metaloadfn("glGenTransformFeedbacks", &["glGenTransformFeedbacksNV"])),
GenVertexArrays: FnPtr::new(metaloadfn("glGenVertexArrays", &["glGenVertexArraysAPPLE", "glGenVertexArraysOES"])),
GenerateMipmap: FnPtr::new(metaloadfn("glGenerateMipmap", &["glGenerateMipmapEXT"])),
GenerateTextureMipmap: FnPtr::new(metaloadfn("glGenerateTextureMipmap", &[])),
GetActiveAtomicCounterBufferiv: FnPtr::new(metaloadfn("glGetActiveAtomicCounterBufferiv", &[])),
GetActiveAttrib: FnPtr::new(metaloadfn("glGetActiveAttrib", &["glGetActiveAttribARB"])),
GetActiveSubroutineName: FnPtr::new(metaloadfn("glGetActiveSubroutineName", &[])),
GetActiveSubroutineUniformName: FnPtr::new(metaloadfn("glGetActiveSubroutineUniformName", &[])),
GetActiveSubroutineUniformiv: FnPtr::new(metaloadfn("glGetActiveSubroutineUniformiv", &[])),
GetActiveUniform: FnPtr::new(metaloadfn("glGetActiveUniform", &["glGetActiveUniformARB"])),
GetActiveUniformBlockName: FnPtr::new(metaloadfn("glGetActiveUniformBlockName", &[])),
GetActiveUniformBlockiv: FnPtr::new(metaloadfn("glGetActiveUniformBlockiv", &[])),
GetActiveUniformName: FnPtr::new(metaloadfn("glGetActiveUniformName", &[])),
GetActiveUniformsiv: FnPtr::new(metaloadfn("glGetActiveUniformsiv", &[])),
GetAttachedShaders: FnPtr::new(metaloadfn("glGetAttachedShaders", &[])),
GetAttribLocation: FnPtr::new(metaloadfn("glGetAttribLocation", &["glGetAttribLocationARB"])),
GetBooleani_v: FnPtr::new(metaloadfn("glGetBooleani_v", &["glGetBooleanIndexedvEXT"])),
GetBooleanv: FnPtr::new(metaloadfn("glGetBooleanv", &[])),
GetBufferParameteri64v: FnPtr::new(metaloadfn("glGetBufferParameteri64v", &[])),
GetBufferParameteriv: FnPtr::new(metaloadfn("glGetBufferParameteriv", &["glGetBufferParameterivARB"])),
GetBufferPointerv: FnPtr::new(metaloadfn("glGetBufferPointerv", &["glGetBufferPointervARB", "glGetBufferPointervOES"])),
GetBufferSubData: FnPtr::new(metaloadfn("glGetBufferSubData", &["glGetBufferSubDataARB"])),
GetCompressedTexImage: FnPtr::new(metaloadfn("glGetCompressedTexImage", &["glGetCompressedTexImageARB"])),
GetCompressedTextureImage: FnPtr::new(metaloadfn("glGetCompressedTextureImage", &[])),
GetCompressedTextureSubImage: FnPtr::new(metaloadfn("glGetCompressedTextureSubImage", &[])),
GetDebugMessageLog: FnPtr::new(metaloadfn("glGetDebugMessageLog", &["glGetDebugMessageLogARB", "glGetDebugMessageLogKHR"])),
GetDoublei_v: FnPtr::new(metaloadfn("glGetDoublei_v", &["glGetDoubleIndexedvEXT", "glGetDoublei_vEXT"])),
GetDoublev: FnPtr::new(metaloadfn("glGetDoublev", &[])),
GetError: FnPtr::new(metaloadfn("glGetError", &[])),
GetFloati_v: FnPtr::new(metaloadfn("glGetFloati_v", &["glGetFloatIndexedvEXT", "glGetFloati_vEXT", "glGetFloati_vNV", "glGetFloati_vOES"])),
GetFloatv: FnPtr::new(metaloadfn("glGetFloatv", &[])),
GetFragDataIndex: FnPtr::new(metaloadfn("glGetFragDataIndex", &["glGetFragDataIndexEXT"])),
GetFragDataLocation: FnPtr::new(metaloadfn("glGetFragDataLocation", &["glGetFragDataLocationEXT"])),
GetFramebufferAttachmentParameteriv: FnPtr::new(metaloadfn("glGetFramebufferAttachmentParameteriv", &["glGetFramebufferAttachmentParameterivEXT"])),
GetFramebufferParameteriv: FnPtr::new(metaloadfn("glGetFramebufferParameteriv", &[])),
GetGraphicsResetStatus: FnPtr::new(metaloadfn("glGetGraphicsResetStatus", &["glGetGraphicsResetStatusKHR"])),
GetInteger64i_v: FnPtr::new(metaloadfn("glGetInteger64i_v", &[])),
GetInteger64v: FnPtr::new(metaloadfn("glGetInteger64v", &["glGetInteger64vAPPLE"])),
GetIntegeri_v: FnPtr::new(metaloadfn("glGetIntegeri_v", &["glGetIntegerIndexedvEXT"])),
GetIntegerv: FnPtr::new(metaloadfn("glGetIntegerv", &[])),
GetInternalformati64v: FnPtr::new(metaloadfn("glGetInternalformati64v", &[])),
GetInternalformativ: FnPtr::new(metaloadfn("glGetInternalformativ", &[])),
GetMultisamplefv: FnPtr::new(metaloadfn("glGetMultisamplefv", &["glGetMultisamplefvNV"])),
GetNamedBufferParameteri64v: FnPtr::new(metaloadfn("glGetNamedBufferParameteri64v", &[])),
GetNamedBufferParameteriv: FnPtr::new(metaloadfn("glGetNamedBufferParameteriv", &[])),
GetNamedBufferPointerv: FnPtr::new(metaloadfn("glGetNamedBufferPointerv", &[])),
GetNamedBufferSubData: FnPtr::new(metaloadfn("glGetNamedBufferSubData", &[])),
GetNamedFramebufferAttachmentParameteriv: FnPtr::new(metaloadfn("glGetNamedFramebufferAttachmentParameteriv", &[])),
GetNamedFramebufferParameteriv: FnPtr::new(metaloadfn("glGetNamedFramebufferParameteriv", &[])),
GetNamedRenderbufferParameteriv: FnPtr::new(metaloadfn("glGetNamedRenderbufferParameteriv", &[])),
GetObjectLabel: FnPtr::new(metaloadfn("glGetObjectLabel", &["glGetObjectLabelKHR"])),
GetObjectPtrLabel: FnPtr::new(metaloadfn("glGetObjectPtrLabel", &["glGetObjectPtrLabelKHR"])),
GetPointerv: FnPtr::new(metaloadfn("glGetPointerv", &["glGetPointervEXT", "glGetPointervKHR"])),
GetProgramBinary: FnPtr::new(metaloadfn("glGetProgramBinary", &["glGetProgramBinaryOES"])),
GetProgramInfoLog: FnPtr::new(metaloadfn("glGetProgramInfoLog", &[])),
GetProgramInterfaceiv: FnPtr::new(metaloadfn("glGetProgramInterfaceiv", &[])),
GetProgramPipelineInfoLog: FnPtr::new(metaloadfn("glGetProgramPipelineInfoLog", &[])),
GetProgramPipelineiv: FnPtr::new(metaloadfn("glGetProgramPipelineiv", &[])),
GetProgramResourceIndex: FnPtr::new(metaloadfn("glGetProgramResourceIndex", &[])),
GetProgramResourceLocation: FnPtr::new(metaloadfn("glGetProgramResourceLocation", &[])),
GetProgramResourceLocationIndex: FnPtr::new(metaloadfn("glGetProgramResourceLocationIndex", &[])),
GetProgramResourceName: FnPtr::new(metaloadfn("glGetProgramResourceName", &[])),
GetProgramResourceiv: FnPtr::new(metaloadfn("glGetProgramResourceiv", &[])),
GetProgramStageiv: FnPtr::new(metaloadfn("glGetProgramStageiv", &[])),
GetProgramiv: FnPtr::new(metaloadfn("glGetProgramiv", &[])),
GetQueryBufferObjecti64v: FnPtr::new(metaloadfn("glGetQueryBufferObjecti64v", &[])),
GetQueryBufferObjectiv: FnPtr::new(metaloadfn("glGetQueryBufferObjectiv", &[])),
GetQueryBufferObjectui64v: FnPtr::new(metaloadfn("glGetQueryBufferObjectui64v", &[])),
GetQueryBufferObjectuiv: FnPtr::new(metaloadfn("glGetQueryBufferObjectuiv", &[])),
GetQueryIndexediv: FnPtr::new(metaloadfn("glGetQueryIndexediv", &[])),
GetQueryObjecti64v: FnPtr::new(metaloadfn("glGetQueryObjecti64v", &["glGetQueryObjecti64vEXT"])),
GetQueryObjectiv: FnPtr::new(metaloadfn("glGetQueryObjectiv", &["glGetQueryObjectivARB", "glGetQueryObjectivEXT"])),
GetQueryObjectui64v: FnPtr::new(metaloadfn("glGetQueryObjectui64v", &["glGetQueryObjectui64vEXT"])),
GetQueryObjectuiv: FnPtr::new(metaloadfn("glGetQueryObjectuiv", &["glGetQueryObjectuivARB"])),
GetQueryiv: FnPtr::new(metaloadfn("glGetQueryiv", &["glGetQueryivARB"])),
GetRenderbufferParameteriv: FnPtr::new(metaloadfn("glGetRenderbufferParameteriv", &["glGetRenderbufferParameterivEXT"])),
GetSamplerParameterIiv: FnPtr::new(metaloadfn("glGetSamplerParameterIiv", &["glGetSamplerParameterIivEXT", "glGetSamplerParameterIivOES"])),
GetSamplerParameterIuiv: FnPtr::new(metaloadfn("glGetSamplerParameterIuiv", &["glGetSamplerParameterIuivEXT", "glGetSamplerParameterIuivOES"])),
GetSamplerParameterfv: FnPtr::new(metaloadfn("glGetSamplerParameterfv", &[])),
GetSamplerParameteriv: FnPtr::new(metaloadfn("glGetSamplerParameteriv", &[])),
GetShaderInfoLog: FnPtr::new(metaloadfn("glGetShaderInfoLog", &[])),
GetShaderPrecisionFormat: FnPtr::new(metaloadfn("glGetShaderPrecisionFormat", &[])),
GetShaderSource: FnPtr::new(metaloadfn("glGetShaderSource", &["glGetShaderSourceARB"])),
GetShaderiv: FnPtr::new(metaloadfn("glGetShaderiv", &[])),
GetString: FnPtr::new(metaloadfn("glGetString", &[])),
GetStringi: FnPtr::new(metaloadfn("glGetStringi", &[])),
GetSubroutineIndex: FnPtr::new(metaloadfn("glGetSubroutineIndex", &[])),
GetSubroutineUniformLocation: FnPtr::new(metaloadfn("glGetSubroutineUniformLocation", &[])),
GetSynciv: FnPtr::new(metaloadfn("glGetSynciv", &["glGetSyncivAPPLE"])),
GetTexImage: FnPtr::new(metaloadfn("glGetTexImage", &[])),
GetTexLevelParameterfv: FnPtr::new(metaloadfn("glGetTexLevelParameterfv", &[])),
GetTexLevelParameteriv: FnPtr::new(metaloadfn("glGetTexLevelParameteriv", &[])),
GetTexParameterIiv: FnPtr::new(metaloadfn("glGetTexParameterIiv", &["glGetTexParameterIivEXT", "glGetTexParameterIivOES"])),
GetTexParameterIuiv: FnPtr::new(metaloadfn("glGetTexParameterIuiv", &["glGetTexParameterIuivEXT", "glGetTexParameterIuivOES"])),
GetTexParameterfv: FnPtr::new(metaloadfn("glGetTexParameterfv", &[])),
GetTexParameteriv: FnPtr::new(metaloadfn("glGetTexParameteriv", &[])),
GetTextureImage: FnPtr::new(metaloadfn("glGetTextureImage", &[])),
GetTextureLevelParameterfv: FnPtr::new(metaloadfn("glGetTextureLevelParameterfv", &[])),
GetTextureLevelParameteriv: FnPtr::new(metaloadfn("glGetTextureLevelParameteriv", &[])),
GetTextureParameterIiv: FnPtr::new(metaloadfn("glGetTextureParameterIiv", &[])),
GetTextureParameterIuiv: FnPtr::new(metaloadfn("glGetTextureParameterIuiv", &[])),
GetTextureParameterfv: FnPtr::new(metaloadfn("glGetTextureParameterfv", &[])),
GetTextureParameteriv: FnPtr::new(metaloadfn("glGetTextureParameteriv", &[])),
GetTextureSubImage: FnPtr::new(metaloadfn("glGetTextureSubImage", &[])),
GetTransformFeedbackVarying: FnPtr::new(metaloadfn("glGetTransformFeedbackVarying", &["glGetTransformFeedbackVaryingEXT"])),
GetTransformFeedbacki64_v: FnPtr::new(metaloadfn("glGetTransformFeedbacki64_v", &[])),
GetTransformFeedbacki_v: FnPtr::new(metaloadfn("glGetTransformFeedbacki_v", &[])),
GetTransformFeedbackiv: FnPtr::new(metaloadfn("glGetTransformFeedbackiv", &[])),
GetUniformBlockIndex: FnPtr::new(metaloadfn("glGetUniformBlockIndex", &[])),
GetUniformIndices: FnPtr::new(metaloadfn("glGetUniformIndices", &[])),
GetUniformLocation: FnPtr::new(metaloadfn("glGetUniformLocation", &["glGetUniformLocationARB"])),
GetUniformSubroutineuiv: FnPtr::new(metaloadfn("glGetUniformSubroutineuiv", &[])),
GetUniformdv: FnPtr::new(metaloadfn("glGetUniformdv", &[])),
GetUniformfv: FnPtr::new(metaloadfn("glGetUniformfv", &["glGetUniformfvARB"])),
GetUniformiv: FnPtr::new(metaloadfn("glGetUniformiv", &["glGetUniformivARB"])),
GetUniformuiv: FnPtr::new(metaloadfn("glGetUniformuiv", &["glGetUniformuivEXT"])),
GetVertexArrayIndexed64iv: FnPtr::new(metaloadfn("glGetVertexArrayIndexed64iv", &[])),
GetVertexArrayIndexediv: FnPtr::new(metaloadfn("glGetVertexArrayIndexediv", &[])),
GetVertexArrayiv: FnPtr::new(metaloadfn("glGetVertexArrayiv", &[])),
GetVertexAttribIiv: FnPtr::new(metaloadfn("glGetVertexAttribIiv", &["glGetVertexAttribIivEXT"])),
GetVertexAttribIuiv: FnPtr::new(metaloadfn("glGetVertexAttribIuiv", &["glGetVertexAttribIuivEXT"])),
GetVertexAttribLdv: FnPtr::new(metaloadfn("glGetVertexAttribLdv", &["glGetVertexAttribLdvEXT"])),
GetVertexAttribPointerv: FnPtr::new(metaloadfn("glGetVertexAttribPointerv", &["glGetVertexAttribPointervARB", "glGetVertexAttribPointervNV"])),
GetVertexAttribdv: FnPtr::new(metaloadfn("glGetVertexAttribdv", &["glGetVertexAttribdvARB", "glGetVertexAttribdvNV"])),
GetVertexAttribfv: FnPtr::new(metaloadfn("glGetVertexAttribfv", &["glGetVertexAttribfvARB", "glGetVertexAttribfvNV"])),
GetVertexAttribiv: FnPtr::new(metaloadfn("glGetVertexAttribiv", &["glGetVertexAttribivARB", "glGetVertexAttribivNV"])),
GetnColorTable: FnPtr::new(metaloadfn("glGetnColorTable", &[])),
GetnCompressedTexImage: FnPtr::new(metaloadfn("glGetnCompressedTexImage", &[])),
GetnConvolutionFilter: FnPtr::new(metaloadfn("glGetnConvolutionFilter", &[])),
GetnHistogram: FnPtr::new(metaloadfn("glGetnHistogram", &[])),
GetnMapdv: FnPtr::new(metaloadfn("glGetnMapdv", &[])),
GetnMapfv: FnPtr::new(metaloadfn("glGetnMapfv", &[])),
GetnMapiv: FnPtr::new(metaloadfn("glGetnMapiv", &[])),
GetnMinmax: FnPtr::new(metaloadfn("glGetnMinmax", &[])),
GetnPixelMapfv: FnPtr::new(metaloadfn("glGetnPixelMapfv", &[])),
GetnPixelMapuiv: FnPtr::new(metaloadfn("glGetnPixelMapuiv", &[])),
GetnPixelMapusv: FnPtr::new(metaloadfn("glGetnPixelMapusv", &[])),
GetnPolygonStipple: FnPtr::new(metaloadfn("glGetnPolygonStipple", &[])),
GetnSeparableFilter: FnPtr::new(metaloadfn("glGetnSeparableFilter", &[])),
GetnTexImage: FnPtr::new(metaloadfn("glGetnTexImage", &[])),
GetnUniformdv: FnPtr::new(metaloadfn("glGetnUniformdv", &[])),
GetnUniformfv: FnPtr::new(metaloadfn("glGetnUniformfv", &["glGetnUniformfvEXT", "glGetnUniformfvKHR"])),
GetnUniformiv: FnPtr::new(metaloadfn("glGetnUniformiv", &["glGetnUniformivEXT", "glGetnUniformivKHR"])),
GetnUniformuiv: FnPtr::new(metaloadfn("glGetnUniformuiv", &["glGetnUniformuivKHR"])),
Hint: FnPtr::new(metaloadfn("glHint", &[])),
InvalidateBufferData: FnPtr::new(metaloadfn("glInvalidateBufferData", &[])),
InvalidateBufferSubData: FnPtr::new(metaloadfn("glInvalidateBufferSubData", &[])),
InvalidateFramebuffer: FnPtr::new(metaloadfn("glInvalidateFramebuffer", &[])),
InvalidateNamedFramebufferData: FnPtr::new(metaloadfn("glInvalidateNamedFramebufferData", &[])),
InvalidateNamedFramebufferSubData: FnPtr::new(metaloadfn("glInvalidateNamedFramebufferSubData", &[])),
InvalidateSubFramebuffer: FnPtr::new(metaloadfn("glInvalidateSubFramebuffer", &[])),
InvalidateTexImage: FnPtr::new(metaloadfn("glInvalidateTexImage", &[])),
InvalidateTexSubImage: FnPtr::new(metaloadfn("glInvalidateTexSubImage", &[])),
IsBuffer: FnPtr::new(metaloadfn("glIsBuffer", &["glIsBufferARB"])),
IsEnabled: FnPtr::new(metaloadfn("glIsEnabled", &[])),
IsEnabledi: FnPtr::new(metaloadfn("glIsEnabledi", &["glIsEnabledIndexedEXT", "glIsEnablediEXT", "glIsEnablediNV", "glIsEnablediOES"])),
IsFramebuffer: FnPtr::new(metaloadfn("glIsFramebuffer", &["glIsFramebufferEXT"])),
IsProgram: FnPtr::new(metaloadfn("glIsProgram", &[])),
IsProgramPipeline: FnPtr::new(metaloadfn("glIsProgramPipeline", &[])),
IsQuery: FnPtr::new(metaloadfn("glIsQuery", &["glIsQueryARB"])),
IsRenderbuffer: FnPtr::new(metaloadfn("glIsRenderbuffer", &["glIsRenderbufferEXT"])),
IsSampler: FnPtr::new(metaloadfn("glIsSampler", &[])),
IsShader: FnPtr::new(metaloadfn("glIsShader", &[])),
IsSync: FnPtr::new(metaloadfn("glIsSync", &["glIsSyncAPPLE"])),
IsTexture: FnPtr::new(metaloadfn("glIsTexture", &[])),
IsTransformFeedback: FnPtr::new(metaloadfn("glIsTransformFeedback", &["glIsTransformFeedbackNV"])),
IsVertexArray: FnPtr::new(metaloadfn("glIsVertexArray", &["glIsVertexArrayAPPLE", "glIsVertexArrayOES"])),
LineWidth: FnPtr::new(metaloadfn("glLineWidth", &[])),
LinkProgram: FnPtr::new(metaloadfn("glLinkProgram", &["glLinkProgramARB"])),
LogicOp: FnPtr::new(metaloadfn("glLogicOp", &[])),
MapBuffer: FnPtr::new(metaloadfn("glMapBuffer", &["glMapBufferARB", "glMapBufferOES"])),
MapBufferRange: FnPtr::new(metaloadfn("glMapBufferRange", &["glMapBufferRangeEXT"])),
MapNamedBuffer: FnPtr::new(metaloadfn("glMapNamedBuffer", &[])),
MapNamedBufferRange: FnPtr::new(metaloadfn("glMapNamedBufferRange", &[])),
MemoryBarrier: FnPtr::new(metaloadfn("glMemoryBarrier", &["glMemoryBarrierEXT"])),
MemoryBarrierByRegion: FnPtr::new(metaloadfn("glMemoryBarrierByRegion", &[])),
MinSampleShading: FnPtr::new(metaloadfn("glMinSampleShading", &["glMinSampleShadingARB", "glMinSampleShadingOES"])),
MultiDrawArrays: FnPtr::new(metaloadfn("glMultiDrawArrays", &["glMultiDrawArraysEXT"])),
MultiDrawArraysIndirect: FnPtr::new(metaloadfn("glMultiDrawArraysIndirect", &["glMultiDrawArraysIndirectAMD", "glMultiDrawArraysIndirectEXT"])),
MultiDrawElements: FnPtr::new(metaloadfn("glMultiDrawElements", &["glMultiDrawElementsEXT"])),
MultiDrawElementsBaseVertex: FnPtr::new(metaloadfn("glMultiDrawElementsBaseVertex", &["glMultiDrawElementsBaseVertexEXT"])),
MultiDrawElementsIndirect: FnPtr::new(metaloadfn("glMultiDrawElementsIndirect", &["glMultiDrawElementsIndirectAMD", "glMultiDrawElementsIndirectEXT"])),
MultiTexCoordP1ui: FnPtr::new(metaloadfn("glMultiTexCoordP1ui", &[])),
MultiTexCoordP1uiv: FnPtr::new(metaloadfn("glMultiTexCoordP1uiv", &[])),
MultiTexCoordP2ui: FnPtr::new(metaloadfn("glMultiTexCoordP2ui", &[])),
MultiTexCoordP2uiv: FnPtr::new(metaloadfn("glMultiTexCoordP2uiv", &[])),
MultiTexCoordP3ui: FnPtr::new(metaloadfn("glMultiTexCoordP3ui", &[])),
MultiTexCoordP3uiv: FnPtr::new(metaloadfn("glMultiTexCoordP3uiv", &[])),
MultiTexCoordP4ui: FnPtr::new(metaloadfn("glMultiTexCoordP4ui", &[])),
MultiTexCoordP4uiv: FnPtr::new(metaloadfn("glMultiTexCoordP4uiv", &[])),
NamedBufferData: FnPtr::new(metaloadfn("glNamedBufferData", &[])),
NamedBufferStorage: FnPtr::new(metaloadfn("glNamedBufferStorage", &["glNamedBufferStorageEXT"])),
NamedBufferSubData: FnPtr::new(metaloadfn("glNamedBufferSubData", &["glNamedBufferSubDataEXT"])),
NamedFramebufferDrawBuffer: FnPtr::new(metaloadfn("glNamedFramebufferDrawBuffer", &[])),
NamedFramebufferDrawBuffers: FnPtr::new(metaloadfn("glNamedFramebufferDrawBuffers", &[])),
NamedFramebufferParameteri: FnPtr::new(metaloadfn("glNamedFramebufferParameteri", &[])),
NamedFramebufferReadBuffer: FnPtr::new(metaloadfn("glNamedFramebufferReadBuffer", &[])),
NamedFramebufferRenderbuffer: FnPtr::new(metaloadfn("glNamedFramebufferRenderbuffer", &[])),
NamedFramebufferTexture: FnPtr::new(metaloadfn("glNamedFramebufferTexture", &[])),
NamedFramebufferTextureLayer: FnPtr::new(metaloadfn("glNamedFramebufferTextureLayer", &[])),
NamedRenderbufferStorage: FnPtr::new(metaloadfn("glNamedRenderbufferStorage", &[])),
NamedRenderbufferStorageMultisample: FnPtr::new(metaloadfn("glNamedRenderbufferStorageMultisample", &[])),
NormalP3ui: FnPtr::new(metaloadfn("glNormalP3ui", &[])),
NormalP3uiv: FnPtr::new(metaloadfn("glNormalP3uiv", &[])),
ObjectLabel: FnPtr::new(metaloadfn("glObjectLabel", &["glObjectLabelKHR"])),
ObjectPtrLabel: FnPtr::new(metaloadfn("glObjectPtrLabel", &["glObjectPtrLabelKHR"])),
PatchParameterfv: FnPtr::new(metaloadfn("glPatchParameterfv", &[])),
PatchParameteri: FnPtr::new(metaloadfn("glPatchParameteri", &["glPatchParameteriEXT", "glPatchParameteriOES"])),
PauseTransformFeedback: FnPtr::new(metaloadfn("glPauseTransformFeedback", &["glPauseTransformFeedbackNV"])),
PixelStoref: FnPtr::new(metaloadfn("glPixelStoref", &[])),
PixelStorei: FnPtr::new(metaloadfn("glPixelStorei", &[])),
PointParameterf: FnPtr::new(metaloadfn("glPointParameterf", &["glPointParameterfARB", "glPointParameterfEXT", "glPointParameterfSGIS"])),
PointParameterfv: FnPtr::new(metaloadfn("glPointParameterfv", &["glPointParameterfvARB", "glPointParameterfvEXT", "glPointParameterfvSGIS"])),
PointParameteri: FnPtr::new(metaloadfn("glPointParameteri", &["glPointParameteriNV"])),
PointParameteriv: FnPtr::new(metaloadfn("glPointParameteriv", &["glPointParameterivNV"])),
PointSize: FnPtr::new(metaloadfn("glPointSize", &[])),
PolygonMode: FnPtr::new(metaloadfn("glPolygonMode", &["glPolygonModeNV"])),
PolygonOffset: FnPtr::new(metaloadfn("glPolygonOffset", &[])),
PopDebugGroup: FnPtr::new(metaloadfn("glPopDebugGroup", &["glPopDebugGroupKHR"])),
PrimitiveRestartIndex: FnPtr::new(metaloadfn("glPrimitiveRestartIndex", &[])),
ProgramBinary: FnPtr::new(metaloadfn("glProgramBinary", &["glProgramBinaryOES"])),
ProgramParameteri: FnPtr::new(metaloadfn("glProgramParameteri", &["glProgramParameteriARB", "glProgramParameteriEXT"])),
ProgramUniform1d: FnPtr::new(metaloadfn("glProgramUniform1d", &[])),
ProgramUniform1dv: FnPtr::new(metaloadfn("glProgramUniform1dv", &[])),
ProgramUniform1f: FnPtr::new(metaloadfn("glProgramUniform1f", &["glProgramUniform1fEXT"])),
ProgramUniform1fv: FnPtr::new(metaloadfn("glProgramUniform1fv", &["glProgramUniform1fvEXT"])),
ProgramUniform1i: FnPtr::new(metaloadfn("glProgramUniform1i", &["glProgramUniform1iEXT"])),
ProgramUniform1iv: FnPtr::new(metaloadfn("glProgramUniform1iv", &["glProgramUniform1ivEXT"])),
ProgramUniform1ui: FnPtr::new(metaloadfn("glProgramUniform1ui", &["glProgramUniform1uiEXT"])),
ProgramUniform1uiv: FnPtr::new(metaloadfn("glProgramUniform1uiv", &["glProgramUniform1uivEXT"])),
ProgramUniform2d: FnPtr::new(metaloadfn("glProgramUniform2d", &[])),
ProgramUniform2dv: FnPtr::new(metaloadfn("glProgramUniform2dv", &[])),
ProgramUniform2f: FnPtr::new(metaloadfn("glProgramUniform2f", &["glProgramUniform2fEXT"])),
ProgramUniform2fv: FnPtr::new(metaloadfn("glProgramUniform2fv", &["glProgramUniform2fvEXT"])),
ProgramUniform2i: FnPtr::new(metaloadfn("glProgramUniform2i", &["glProgramUniform2iEXT"])),
ProgramUniform2iv: FnPtr::new(metaloadfn("glProgramUniform2iv", &["glProgramUniform2ivEXT"])),
ProgramUniform2ui: FnPtr::new(metaloadfn("glProgramUniform2ui", &["glProgramUniform2uiEXT"])),
ProgramUniform2uiv: FnPtr::new(metaloadfn("glProgramUniform2uiv", &["glProgramUniform2uivEXT"])),
ProgramUniform3d: FnPtr::new(metaloadfn("glProgramUniform3d", &[])),
ProgramUniform3dv: FnPtr::new(metaloadfn("glProgramUniform3dv", &[])),
ProgramUniform3f: FnPtr::new(metaloadfn("glProgramUniform3f", &["glProgramUniform3fEXT"])),
ProgramUniform3fv: FnPtr::new(metaloadfn("glProgramUniform3fv", &["glProgramUniform3fvEXT"])),
ProgramUniform3i: FnPtr::new(metaloadfn("glProgramUniform3i", &["glProgramUniform3iEXT"])),
ProgramUniform3iv: FnPtr::new(metaloadfn("glProgramUniform3iv", &["glProgramUniform3ivEXT"])),
ProgramUniform3ui: FnPtr::new(metaloadfn("glProgramUniform3ui", &["glProgramUniform3uiEXT"])),
ProgramUniform3uiv: FnPtr::new(metaloadfn("glProgramUniform3uiv", &["glProgramUniform3uivEXT"])),
ProgramUniform4d: FnPtr::new(metaloadfn("glProgramUniform4d", &[])),
ProgramUniform4dv: FnPtr::new(metaloadfn("glProgramUniform4dv", &[])),
ProgramUniform4f: FnPtr::new(metaloadfn("glProgramUniform4f", &["glProgramUniform4fEXT"])),
ProgramUniform4fv: FnPtr::new(metaloadfn("glProgramUniform4fv", &["glProgramUniform4fvEXT"])),
ProgramUniform4i: FnPtr::new(metaloadfn("glProgramUniform4i", &["glProgramUniform4iEXT"])),
ProgramUniform4iv: FnPtr::new(metaloadfn("glProgramUniform4iv", &["glProgramUniform4ivEXT"])),
ProgramUniform4ui: FnPtr::new(metaloadfn("glProgramUniform4ui", &["glProgramUniform4uiEXT"])),
ProgramUniform4uiv: FnPtr::new(metaloadfn("glProgramUniform4uiv", &["glProgramUniform4uivEXT"])),
ProgramUniformMatrix2dv: FnPtr::new(metaloadfn("glProgramUniformMatrix2dv", &[])),
ProgramUniformMatrix2fv: FnPtr::new(metaloadfn("glProgramUniformMatrix2fv", &["glProgramUniformMatrix2fvEXT"])),
ProgramUniformMatrix2x3dv: FnPtr::new(metaloadfn("glProgramUniformMatrix2x3dv", &[])),
ProgramUniformMatrix2x3fv: FnPtr::new(metaloadfn("glProgramUniformMatrix2x3fv", &["glProgramUniformMatrix2x3fvEXT"])),
ProgramUniformMatrix2x4dv: FnPtr::new(metaloadfn("glProgramUniformMatrix2x4dv", &[])),
ProgramUniformMatrix2x4fv: FnPtr::new(metaloadfn("glProgramUniformMatrix2x4fv", &["glProgramUniformMatrix2x4fvEXT"])),
ProgramUniformMatrix3dv: FnPtr::new(metaloadfn("glProgramUniformMatrix3dv", &[])),
ProgramUniformMatrix3fv: FnPtr::new(metaloadfn("glProgramUniformMatrix3fv", &["glProgramUniformMatrix3fvEXT"])),
ProgramUniformMatrix3x2dv: FnPtr::new(metaloadfn("glProgramUniformMatrix3x2dv", &[])),
ProgramUniformMatrix3x2fv: FnPtr::new(metaloadfn("glProgramUniformMatrix3x2fv", &["glProgramUniformMatrix3x2fvEXT"])),
ProgramUniformMatrix3x4dv: FnPtr::new(metaloadfn("glProgramUniformMatrix3x4dv", &[])),
ProgramUniformMatrix3x4fv: FnPtr::new(metaloadfn("glProgramUniformMatrix3x4fv", &["glProgramUniformMatrix3x4fvEXT"])),
ProgramUniformMatrix4dv: FnPtr::new(metaloadfn("glProgramUniformMatrix4dv", &[])),
ProgramUniformMatrix4fv: FnPtr::new(metaloadfn("glProgramUniformMatrix4fv", &["glProgramUniformMatrix4fvEXT"])),
ProgramUniformMatrix4x2dv: FnPtr::new(metaloadfn("glProgramUniformMatrix4x2dv", &[])),
ProgramUniformMatrix4x2fv: FnPtr::new(metaloadfn("glProgramUniformMatrix4x2fv", &["glProgramUniformMatrix4x2fvEXT"])),
ProgramUniformMatrix4x3dv: FnPtr::new(metaloadfn("glProgramUniformMatrix4x3dv", &[])),
ProgramUniformMatrix4x3fv: FnPtr::new(metaloadfn("glProgramUniformMatrix4x3fv", &["glProgramUniformMatrix4x3fvEXT"])),
ProvokingVertex: FnPtr::new(metaloadfn("glProvokingVertex", &["glProvokingVertexEXT"])),
PushDebugGroup: FnPtr::new(metaloadfn("glPushDebugGroup", &["glPushDebugGroupKHR"])),
QueryCounter: FnPtr::new(metaloadfn("glQueryCounter", &["glQueryCounterEXT"])),
ReadBuffer: FnPtr::new(metaloadfn("glReadBuffer", &[])),
ReadPixels: FnPtr::new(metaloadfn("glReadPixels", &[])),
ReadnPixels: FnPtr::new(metaloadfn("glReadnPixels", &["glReadnPixelsARB", "glReadnPixelsEXT", "glReadnPixelsKHR"])),
ReleaseShaderCompiler: FnPtr::new(metaloadfn("glReleaseShaderCompiler", &[])),
RenderbufferStorage: FnPtr::new(metaloadfn("glRenderbufferStorage", &["glRenderbufferStorageEXT"])),
RenderbufferStorageMultisample: FnPtr::new(metaloadfn("glRenderbufferStorageMultisample", &["glRenderbufferStorageMultisampleEXT", "glRenderbufferStorageMultisampleNV"])),
ResumeTransformFeedback: FnPtr::new(metaloadfn("glResumeTransformFeedback", &["glResumeTransformFeedbackNV"])),
SampleCoverage: FnPtr::new(metaloadfn("glSampleCoverage", &["glSampleCoverageARB"])),
SampleMaski: FnPtr::new(metaloadfn("glSampleMaski", &[])),
SamplerParameterIiv: FnPtr::new(metaloadfn("glSamplerParameterIiv", &["glSamplerParameterIivEXT", "glSamplerParameterIivOES"])),
SamplerParameterIuiv: FnPtr::new(metaloadfn("glSamplerParameterIuiv", &["glSamplerParameterIuivEXT", "glSamplerParameterIuivOES"])),
SamplerParameterf: FnPtr::new(metaloadfn("glSamplerParameterf", &[])),
SamplerParameterfv: FnPtr::new(metaloadfn("glSamplerParameterfv", &[])),
SamplerParameteri: FnPtr::new(metaloadfn("glSamplerParameteri", &[])),
SamplerParameteriv: FnPtr::new(metaloadfn("glSamplerParameteriv", &[])),
Scissor: FnPtr::new(metaloadfn("glScissor", &[])),
ScissorArrayv: FnPtr::new(metaloadfn("glScissorArrayv", &["glScissorArrayvNV", "glScissorArrayvOES"])),
ScissorIndexed: FnPtr::new(metaloadfn("glScissorIndexed", &["glScissorIndexedNV", "glScissorIndexedOES"])),
ScissorIndexedv: FnPtr::new(metaloadfn("glScissorIndexedv", &["glScissorIndexedvNV", "glScissorIndexedvOES"])),
SecondaryColorP3ui: FnPtr::new(metaloadfn("glSecondaryColorP3ui", &[])),
SecondaryColorP3uiv: FnPtr::new(metaloadfn("glSecondaryColorP3uiv", &[])),
ShaderBinary: FnPtr::new(metaloadfn("glShaderBinary", &[])),
ShaderSource: FnPtr::new(metaloadfn("glShaderSource", &["glShaderSourceARB"])),
ShaderStorageBlockBinding: FnPtr::new(metaloadfn("glShaderStorageBlockBinding", &[])),
StencilFunc: FnPtr::new(metaloadfn("glStencilFunc", &[])),
StencilFuncSeparate: FnPtr::new(metaloadfn("glStencilFuncSeparate", &[])),
StencilMask: FnPtr::new(metaloadfn("glStencilMask", &[])),
StencilMaskSeparate: FnPtr::new(metaloadfn("glStencilMaskSeparate", &[])),
StencilOp: FnPtr::new(metaloadfn("glStencilOp", &[])),
StencilOpSeparate: FnPtr::new(metaloadfn("glStencilOpSeparate", &["glStencilOpSeparateATI"])),
TexBuffer: FnPtr::new(metaloadfn("glTexBuffer", &["glTexBufferARB", "glTexBufferEXT", "glTexBufferOES"])),
TexBufferRange: FnPtr::new(metaloadfn("glTexBufferRange", &["glTexBufferRangeEXT", "glTexBufferRangeOES"])),
TexCoordP1ui: FnPtr::new(metaloadfn("glTexCoordP1ui", &[])),
TexCoordP1uiv: FnPtr::new(metaloadfn("glTexCoordP1uiv", &[])),
TexCoordP2ui: FnPtr::new(metaloadfn("glTexCoordP2ui", &[])),
TexCoordP2uiv: FnPtr::new(metaloadfn("glTexCoordP2uiv", &[])),
TexCoordP3ui: FnPtr::new(metaloadfn("glTexCoordP3ui", &[])),
TexCoordP3uiv: FnPtr::new(metaloadfn("glTexCoordP3uiv", &[])),
TexCoordP4ui: FnPtr::new(metaloadfn("glTexCoordP4ui", &[])),
TexCoordP4uiv: FnPtr::new(metaloadfn("glTexCoordP4uiv", &[])),
TexImage1D: FnPtr::new(metaloadfn("glTexImage1D", &[])),
TexImage2D: FnPtr::new(metaloadfn("glTexImage2D", &[])),
TexImage2DMultisample: FnPtr::new(metaloadfn("glTexImage2DMultisample", &[])),
TexImage3D: FnPtr::new(metaloadfn("glTexImage3D", &["glTexImage3DEXT"])),
TexImage3DMultisample: FnPtr::new(metaloadfn("glTexImage3DMultisample", &[])),
TexParameterIiv: FnPtr::new(metaloadfn("glTexParameterIiv", &["glTexParameterIivEXT", "glTexParameterIivOES"])),
TexParameterIuiv: FnPtr::new(metaloadfn("glTexParameterIuiv", &["glTexParameterIuivEXT", "glTexParameterIuivOES"])),
TexParameterf: FnPtr::new(metaloadfn("glTexParameterf", &[])),
TexParameterfv: FnPtr::new(metaloadfn("glTexParameterfv", &[])),
TexParameteri: FnPtr::new(metaloadfn("glTexParameteri", &[])),
TexParameteriv: FnPtr::new(metaloadfn("glTexParameteriv", &[])),
TexStorage1D: FnPtr::new(metaloadfn("glTexStorage1D", &["glTexStorage1DEXT"])),
TexStorage2D: FnPtr::new(metaloadfn("glTexStorage2D", &["glTexStorage2DEXT"])),
TexStorage2DMultisample: FnPtr::new(metaloadfn("glTexStorage2DMultisample", &[])),
TexStorage3D: FnPtr::new(metaloadfn("glTexStorage3D", &["glTexStorage3DEXT"])),
TexStorage3DMultisample: FnPtr::new(metaloadfn("glTexStorage3DMultisample", &["glTexStorage3DMultisampleOES"])),
TexSubImage1D: FnPtr::new(metaloadfn("glTexSubImage1D", &["glTexSubImage1DEXT"])),
TexSubImage2D: FnPtr::new(metaloadfn("glTexSubImage2D", &["glTexSubImage2DEXT"])),
TexSubImage3D: FnPtr::new(metaloadfn("glTexSubImage3D", &["glTexSubImage3DEXT"])),
TextureBarrier: FnPtr::new(metaloadfn("glTextureBarrier", &[])),
TextureBuffer: FnPtr::new(metaloadfn("glTextureBuffer", &[])),
TextureBufferRange: FnPtr::new(metaloadfn("glTextureBufferRange", &[])),
TextureParameterIiv: FnPtr::new(metaloadfn("glTextureParameterIiv", &[])),
TextureParameterIuiv: FnPtr::new(metaloadfn("glTextureParameterIuiv", &[])),
TextureParameterf: FnPtr::new(metaloadfn("glTextureParameterf", &[])),
TextureParameterfv: FnPtr::new(metaloadfn("glTextureParameterfv", &[])),
TextureParameteri: FnPtr::new(metaloadfn("glTextureParameteri", &[])),
TextureParameteriv: FnPtr::new(metaloadfn("glTextureParameteriv", &[])),
TextureStorage1D: FnPtr::new(metaloadfn("glTextureStorage1D", &[])),
TextureStorage2D: FnPtr::new(metaloadfn("glTextureStorage2D", &[])),
TextureStorage2DMultisample: FnPtr::new(metaloadfn("glTextureStorage2DMultisample", &[])),
TextureStorage3D: FnPtr::new(metaloadfn("glTextureStorage3D", &[])),
TextureStorage3DMultisample: FnPtr::new(metaloadfn("glTextureStorage3DMultisample", &[])),
TextureSubImage1D: FnPtr::new(metaloadfn("glTextureSubImage1D", &[])),
TextureSubImage2D: FnPtr::new(metaloadfn("glTextureSubImage2D", &[])),
TextureSubImage3D: FnPtr::new(metaloadfn("glTextureSubImage3D", &[])),
TextureView: FnPtr::new(metaloadfn("glTextureView", &["glTextureViewEXT", "glTextureViewOES"])),
TransformFeedbackBufferBase: FnPtr::new(metaloadfn("glTransformFeedbackBufferBase", &[])),
TransformFeedbackBufferRange: FnPtr::new(metaloadfn("glTransformFeedbackBufferRange", &[])),
TransformFeedbackVaryings: FnPtr::new(metaloadfn("glTransformFeedbackVaryings", &["glTransformFeedbackVaryingsEXT"])),
Uniform1d: FnPtr::new(metaloadfn("glUniform1d", &[])),
Uniform1dv: FnPtr::new(metaloadfn("glUniform1dv", &[])),
Uniform1f: FnPtr::new(metaloadfn("glUniform1f", &["glUniform1fARB"])),
Uniform1fv: FnPtr::new(metaloadfn("glUniform1fv", &["glUniform1fvARB"])),
Uniform1i: FnPtr::new(metaloadfn("glUniform1i", &["glUniform1iARB"])),
Uniform1iv: FnPtr::new(metaloadfn("glUniform1iv", &["glUniform1ivARB"])),
Uniform1ui: FnPtr::new(metaloadfn("glUniform1ui", &["glUniform1uiEXT"])),
Uniform1uiv: FnPtr::new(metaloadfn("glUniform1uiv", &["glUniform1uivEXT"])),
Uniform2d: FnPtr::new(metaloadfn("glUniform2d", &[])),
Uniform2dv: FnPtr::new(metaloadfn("glUniform2dv", &[])),
Uniform2f: FnPtr::new(metaloadfn("glUniform2f", &["glUniform2fARB"])),
Uniform2fv: FnPtr::new(metaloadfn("glUniform2fv", &["glUniform2fvARB"])),
Uniform2i: FnPtr::new(metaloadfn("glUniform2i", &["glUniform2iARB"])),
Uniform2iv: FnPtr::new(metaloadfn("glUniform2iv", &["glUniform2ivARB"])),
Uniform2ui: FnPtr::new(metaloadfn("glUniform2ui", &["glUniform2uiEXT"])),
Uniform2uiv: FnPtr::new(metaloadfn("glUniform2uiv", &["glUniform2uivEXT"])),
Uniform3d: FnPtr::new(metaloadfn("glUniform3d", &[])),
Uniform3dv: FnPtr::new(metaloadfn("glUniform3dv", &[])),
Uniform3f: FnPtr::new(metaloadfn("glUniform3f", &["glUniform3fARB"])),
Uniform3fv: FnPtr::new(metaloadfn("glUniform3fv", &["glUniform3fvARB"])),
Uniform3i: FnPtr::new(metaloadfn("glUniform3i", &["glUniform3iARB"])),
Uniform3iv: FnPtr::new(metaloadfn("glUniform3iv", &["glUniform3ivARB"])),
Uniform3ui: FnPtr::new(metaloadfn("glUniform3ui", &["glUniform3uiEXT"])),
Uniform3uiv: FnPtr::new(metaloadfn("glUniform3uiv", &["glUniform3uivEXT"])),
Uniform4d: FnPtr::new(metaloadfn("glUniform4d", &[])),
Uniform4dv: FnPtr::new(metaloadfn("glUniform4dv", &[])),
Uniform4f: FnPtr::new(metaloadfn("glUniform4f", &["glUniform4fARB"])),
Uniform4fv: FnPtr::new(metaloadfn("glUniform4fv", &["glUniform4fvARB"])),
Uniform4i: FnPtr::new(metaloadfn("glUniform4i", &["glUniform4iARB"])),
Uniform4iv: FnPtr::new(metaloadfn("glUniform4iv", &["glUniform4ivARB"])),
Uniform4ui: FnPtr::new(metaloadfn("glUniform4ui", &["glUniform4uiEXT"])),
Uniform4uiv: FnPtr::new(metaloadfn("glUniform4uiv", &["glUniform4uivEXT"])),
UniformBlockBinding: FnPtr::new(metaloadfn("glUniformBlockBinding", &[])),
UniformMatrix2dv: FnPtr::new(metaloadfn("glUniformMatrix2dv", &[])),
UniformMatrix2fv: FnPtr::new(metaloadfn("glUniformMatrix2fv", &["glUniformMatrix2fvARB"])),
UniformMatrix2x3dv: FnPtr::new(metaloadfn("glUniformMatrix2x3dv", &[])),
UniformMatrix2x3fv: FnPtr::new(metaloadfn("glUniformMatrix2x3fv", &["glUniformMatrix2x3fvNV"])),
UniformMatrix2x4dv: FnPtr::new(metaloadfn("glUniformMatrix2x4dv", &[])),
UniformMatrix2x4fv: FnPtr::new(metaloadfn("glUniformMatrix2x4fv", &["glUniformMatrix2x4fvNV"])),
UniformMatrix3dv: FnPtr::new(metaloadfn("glUniformMatrix3dv", &[])),
UniformMatrix3fv: FnPtr::new(metaloadfn("glUniformMatrix3fv", &["glUniformMatrix3fvARB"])),
UniformMatrix3x2dv: FnPtr::new(metaloadfn("glUniformMatrix3x2dv", &[])),
UniformMatrix3x2fv: FnPtr::new(metaloadfn("glUniformMatrix3x2fv", &["glUniformMatrix3x2fvNV"])),
UniformMatrix3x4dv: FnPtr::new(metaloadfn("glUniformMatrix3x4dv", &[])),
UniformMatrix3x4fv: FnPtr::new(metaloadfn("glUniformMatrix3x4fv", &["glUniformMatrix3x4fvNV"])),
UniformMatrix4dv: FnPtr::new(metaloadfn("glUniformMatrix4dv", &[])),
UniformMatrix4fv: FnPtr::new(metaloadfn("glUniformMatrix4fv", &["glUniformMatrix4fvARB"])),
UniformMatrix4x2dv: FnPtr::new(metaloadfn("glUniformMatrix4x2dv", &[])),
UniformMatrix4x2fv: FnPtr::new(metaloadfn("glUniformMatrix4x2fv", &["glUniformMatrix4x2fvNV"])),
UniformMatrix4x3dv: FnPtr::new(metaloadfn("glUniformMatrix4x3dv", &[])),
UniformMatrix4x3fv: FnPtr::new(metaloadfn("glUniformMatrix4x3fv", &["glUniformMatrix4x3fvNV"])),
UniformSubroutinesuiv: FnPtr::new(metaloadfn("glUniformSubroutinesuiv", &[])),
UnmapBuffer: FnPtr::new(metaloadfn("glUnmapBuffer", &["glUnmapBufferARB", "glUnmapBufferOES"])),
UnmapNamedBuffer: FnPtr::new(metaloadfn("glUnmapNamedBuffer", &[])),
UseProgram: FnPtr::new(metaloadfn("glUseProgram", &["glUseProgramObjectARB"])),
UseProgramStages: FnPtr::new(metaloadfn("glUseProgramStages", &[])),
ValidateProgram: FnPtr::new(metaloadfn("glValidateProgram", &["glValidateProgramARB"])),
ValidateProgramPipeline: FnPtr::new(metaloadfn("glValidateProgramPipeline", &[])),
VertexArrayAttribBinding: FnPtr::new(metaloadfn("glVertexArrayAttribBinding", &[])),
VertexArrayAttribFormat: FnPtr::new(metaloadfn("glVertexArrayAttribFormat", &[])),
VertexArrayAttribIFormat: FnPtr::new(metaloadfn("glVertexArrayAttribIFormat", &[])),
VertexArrayAttribLFormat: FnPtr::new(metaloadfn("glVertexArrayAttribLFormat", &[])),
VertexArrayBindingDivisor: FnPtr::new(metaloadfn("glVertexArrayBindingDivisor", &[])),
VertexArrayElementBuffer: FnPtr::new(metaloadfn("glVertexArrayElementBuffer", &[])),
VertexArrayVertexBuffer: FnPtr::new(metaloadfn("glVertexArrayVertexBuffer", &[])),
VertexArrayVertexBuffers: FnPtr::new(metaloadfn("glVertexArrayVertexBuffers", &[])),
VertexAttrib1d: FnPtr::new(metaloadfn("glVertexAttrib1d", &["glVertexAttrib1dARB", "glVertexAttrib1dNV"])),
VertexAttrib1dv: FnPtr::new(metaloadfn("glVertexAttrib1dv", &["glVertexAttrib1dvARB", "glVertexAttrib1dvNV"])),
VertexAttrib1f: FnPtr::new(metaloadfn("glVertexAttrib1f", &["glVertexAttrib1fARB", "glVertexAttrib1fNV"])),
VertexAttrib1fv: FnPtr::new(metaloadfn("glVertexAttrib1fv", &["glVertexAttrib1fvARB", "glVertexAttrib1fvNV"])),
VertexAttrib1s: FnPtr::new(metaloadfn("glVertexAttrib1s", &["glVertexAttrib1sARB", "glVertexAttrib1sNV"])),
VertexAttrib1sv: FnPtr::new(metaloadfn("glVertexAttrib1sv", &["glVertexAttrib1svARB", "glVertexAttrib1svNV"])),
VertexAttrib2d: FnPtr::new(metaloadfn("glVertexAttrib2d", &["glVertexAttrib2dARB", "glVertexAttrib2dNV"])),
VertexAttrib2dv: FnPtr::new(metaloadfn("glVertexAttrib2dv", &["glVertexAttrib2dvARB", "glVertexAttrib2dvNV"])),
VertexAttrib2f: FnPtr::new(metaloadfn("glVertexAttrib2f", &["glVertexAttrib2fARB", "glVertexAttrib2fNV"])),
VertexAttrib2fv: FnPtr::new(metaloadfn("glVertexAttrib2fv", &["glVertexAttrib2fvARB", "glVertexAttrib2fvNV"])),
VertexAttrib2s: FnPtr::new(metaloadfn("glVertexAttrib2s", &["glVertexAttrib2sARB", "glVertexAttrib2sNV"])),
VertexAttrib2sv: FnPtr::new(metaloadfn("glVertexAttrib2sv", &["glVertexAttrib2svARB", "glVertexAttrib2svNV"])),
VertexAttrib3d: FnPtr::new(metaloadfn("glVertexAttrib3d", &["glVertexAttrib3dARB", "glVertexAttrib3dNV"])),
VertexAttrib3dv: FnPtr::new(metaloadfn("glVertexAttrib3dv", &["glVertexAttrib3dvARB", "glVertexAttrib3dvNV"])),
VertexAttrib3f: FnPtr::new(metaloadfn("glVertexAttrib3f", &["glVertexAttrib3fARB", "glVertexAttrib3fNV"])),
VertexAttrib3fv: FnPtr::new(metaloadfn("glVertexAttrib3fv", &["glVertexAttrib3fvARB", "glVertexAttrib3fvNV"])),
VertexAttrib3s: FnPtr::new(metaloadfn("glVertexAttrib3s", &["glVertexAttrib3sARB", "glVertexAttrib3sNV"])),
VertexAttrib3sv: FnPtr::new(metaloadfn("glVertexAttrib3sv", &["glVertexAttrib3svARB", "glVertexAttrib3svNV"])),
VertexAttrib4Nbv: FnPtr::new(metaloadfn("glVertexAttrib4Nbv", &["glVertexAttrib4NbvARB"])),
VertexAttrib4Niv: FnPtr::new(metaloadfn("glVertexAttrib4Niv", &["glVertexAttrib4NivARB"])),
VertexAttrib4Nsv: FnPtr::new(metaloadfn("glVertexAttrib4Nsv", &["glVertexAttrib4NsvARB"])),
VertexAttrib4Nub: FnPtr::new(metaloadfn("glVertexAttrib4Nub", &["glVertexAttrib4NubARB", "glVertexAttrib4ubNV"])),
VertexAttrib4Nubv: FnPtr::new(metaloadfn("glVertexAttrib4Nubv", &["glVertexAttrib4NubvARB", "glVertexAttrib4ubvNV"])),
VertexAttrib4Nuiv: FnPtr::new(metaloadfn("glVertexAttrib4Nuiv", &["glVertexAttrib4NuivARB"])),
VertexAttrib4Nusv: FnPtr::new(metaloadfn("glVertexAttrib4Nusv", &["glVertexAttrib4NusvARB"])),
VertexAttrib4bv: FnPtr::new(metaloadfn("glVertexAttrib4bv", &["glVertexAttrib4bvARB"])),
VertexAttrib4d: FnPtr::new(metaloadfn("glVertexAttrib4d", &["glVertexAttrib4dARB", "glVertexAttrib4dNV"])),
VertexAttrib4dv: FnPtr::new(metaloadfn("glVertexAttrib4dv", &["glVertexAttrib4dvARB", "glVertexAttrib4dvNV"])),
VertexAttrib4f: FnPtr::new(metaloadfn("glVertexAttrib4f", &["glVertexAttrib4fARB", "glVertexAttrib4fNV"])),
VertexAttrib4fv: FnPtr::new(metaloadfn("glVertexAttrib4fv", &["glVertexAttrib4fvARB", "glVertexAttrib4fvNV"])),
VertexAttrib4iv: FnPtr::new(metaloadfn("glVertexAttrib4iv", &["glVertexAttrib4ivARB"])),
VertexAttrib4s: FnPtr::new(metaloadfn("glVertexAttrib4s", &["glVertexAttrib4sARB", "glVertexAttrib4sNV"])),
VertexAttrib4sv: FnPtr::new(metaloadfn("glVertexAttrib4sv", &["glVertexAttrib4svARB", "glVertexAttrib4svNV"])),
VertexAttrib4ubv: FnPtr::new(metaloadfn("glVertexAttrib4ubv", &["glVertexAttrib4ubvARB"])),
VertexAttrib4uiv: FnPtr::new(metaloadfn("glVertexAttrib4uiv", &["glVertexAttrib4uivARB"])),
VertexAttrib4usv: FnPtr::new(metaloadfn("glVertexAttrib4usv", &["glVertexAttrib4usvARB"])),
VertexAttribBinding: FnPtr::new(metaloadfn("glVertexAttribBinding", &[])),
VertexAttribDivisor: FnPtr::new(metaloadfn("glVertexAttribDivisor", &["glVertexAttribDivisorANGLE", "glVertexAttribDivisorARB", "glVertexAttribDivisorEXT", "glVertexAttribDivisorNV"])),
VertexAttribFormat: FnPtr::new(metaloadfn("glVertexAttribFormat", &[])),
VertexAttribI1i: FnPtr::new(metaloadfn("glVertexAttribI1i", &["glVertexAttribI1iEXT"])),
VertexAttribI1iv: FnPtr::new(metaloadfn("glVertexAttribI1iv", &["glVertexAttribI1ivEXT"])),
VertexAttribI1ui: FnPtr::new(metaloadfn("glVertexAttribI1ui", &["glVertexAttribI1uiEXT"])),
VertexAttribI1uiv: FnPtr::new(metaloadfn("glVertexAttribI1uiv", &["glVertexAttribI1uivEXT"])),
VertexAttribI2i: FnPtr::new(metaloadfn("glVertexAttribI2i", &["glVertexAttribI2iEXT"])),
VertexAttribI2iv: FnPtr::new(metaloadfn("glVertexAttribI2iv", &["glVertexAttribI2ivEXT"])),
VertexAttribI2ui: FnPtr::new(metaloadfn("glVertexAttribI2ui", &["glVertexAttribI2uiEXT"])),
VertexAttribI2uiv: FnPtr::new(metaloadfn("glVertexAttribI2uiv", &["glVertexAttribI2uivEXT"])),
VertexAttribI3i: FnPtr::new(metaloadfn("glVertexAttribI3i", &["glVertexAttribI3iEXT"])),
VertexAttribI3iv: FnPtr::new(metaloadfn("glVertexAttribI3iv", &["glVertexAttribI3ivEXT"])),
VertexAttribI3ui: FnPtr::new(metaloadfn("glVertexAttribI3ui", &["glVertexAttribI3uiEXT"])),
VertexAttribI3uiv: FnPtr::new(metaloadfn("glVertexAttribI3uiv", &["glVertexAttribI3uivEXT"])),
VertexAttribI4bv: FnPtr::new(metaloadfn("glVertexAttribI4bv", &["glVertexAttribI4bvEXT"])),
VertexAttribI4i: FnPtr::new(metaloadfn("glVertexAttribI4i", &["glVertexAttribI4iEXT"])),
VertexAttribI4iv: FnPtr::new(metaloadfn("glVertexAttribI4iv", &["glVertexAttribI4ivEXT"])),
VertexAttribI4sv: FnPtr::new(metaloadfn("glVertexAttribI4sv", &["glVertexAttribI4svEXT"])),
VertexAttribI4ubv: FnPtr::new(metaloadfn("glVertexAttribI4ubv", &["glVertexAttribI4ubvEXT"])),
VertexAttribI4ui: FnPtr::new(metaloadfn("glVertexAttribI4ui", &["glVertexAttribI4uiEXT"])),
VertexAttribI4uiv: FnPtr::new(metaloadfn("glVertexAttribI4uiv", &["glVertexAttribI4uivEXT"])),
VertexAttribI4usv: FnPtr::new(metaloadfn("glVertexAttribI4usv", &["glVertexAttribI4usvEXT"])),
VertexAttribIFormat: FnPtr::new(metaloadfn("glVertexAttribIFormat", &[])),
VertexAttribIPointer: FnPtr::new(metaloadfn("glVertexAttribIPointer", &["glVertexAttribIPointerEXT"])),
VertexAttribL1d: FnPtr::new(metaloadfn("glVertexAttribL1d", &["glVertexAttribL1dEXT"])),
VertexAttribL1dv: FnPtr::new(metaloadfn("glVertexAttribL1dv", &["glVertexAttribL1dvEXT"])),
VertexAttribL2d: FnPtr::new(metaloadfn("glVertexAttribL2d", &["glVertexAttribL2dEXT"])),
VertexAttribL2dv: FnPtr::new(metaloadfn("glVertexAttribL2dv", &["glVertexAttribL2dvEXT"])),
VertexAttribL3d: FnPtr::new(metaloadfn("glVertexAttribL3d", &["glVertexAttribL3dEXT"])),
VertexAttribL3dv: FnPtr::new(metaloadfn("glVertexAttribL3dv", &["glVertexAttribL3dvEXT"])),
VertexAttribL4d: FnPtr::new(metaloadfn("glVertexAttribL4d", &["glVertexAttribL4dEXT"])),
VertexAttribL4dv: FnPtr::new(metaloadfn("glVertexAttribL4dv", &["glVertexAttribL4dvEXT"])),
VertexAttribLFormat: FnPtr::new(metaloadfn("glVertexAttribLFormat", &[])),
VertexAttribLPointer: FnPtr::new(metaloadfn("glVertexAttribLPointer", &["glVertexAttribLPointerEXT"])),
VertexAttribP1ui: FnPtr::new(metaloadfn("glVertexAttribP1ui", &[])),
VertexAttribP1uiv: FnPtr::new(metaloadfn("glVertexAttribP1uiv", &[])),
VertexAttribP2ui: FnPtr::new(metaloadfn("glVertexAttribP2ui", &[])),
VertexAttribP2uiv: FnPtr::new(metaloadfn("glVertexAttribP2uiv", &[])),
VertexAttribP3ui: FnPtr::new(metaloadfn("glVertexAttribP3ui", &[])),
VertexAttribP3uiv: FnPtr::new(metaloadfn("glVertexAttribP3uiv", &[])),
VertexAttribP4ui: FnPtr::new(metaloadfn("glVertexAttribP4ui", &[])),
VertexAttribP4uiv: FnPtr::new(metaloadfn("glVertexAttribP4uiv", &[])),
VertexAttribPointer: FnPtr::new(metaloadfn("glVertexAttribPointer", &["glVertexAttribPointerARB"])),
VertexBindingDivisor: FnPtr::new(metaloadfn("glVertexBindingDivisor", &[])),
VertexP2ui: FnPtr::new(metaloadfn("glVertexP2ui", &[])),
VertexP2uiv: FnPtr::new(metaloadfn("glVertexP2uiv", &[])),
VertexP3ui: FnPtr::new(metaloadfn("glVertexP3ui", &[])),
VertexP3uiv: FnPtr::new(metaloadfn("glVertexP3uiv", &[])),
VertexP4ui: FnPtr::new(metaloadfn("glVertexP4ui", &[])),
VertexP4uiv: FnPtr::new(metaloadfn("glVertexP4uiv", &[])),
Viewport: FnPtr::new(metaloadfn("glViewport", &[])),
ViewportArrayv: FnPtr::new(metaloadfn("glViewportArrayv", &["glViewportArrayvNV", "glViewportArrayvOES"])),
ViewportIndexedf: FnPtr::new(metaloadfn("glViewportIndexedf", &["glViewportIndexedfOES", "glViewportIndexedfNV"])),
ViewportIndexedfv: FnPtr::new(metaloadfn("glViewportIndexedfv", &["glViewportIndexedfvOES", "glViewportIndexedfvNV"])),
WaitSync: FnPtr::new(metaloadfn("glWaitSync", &["glWaitSyncAPPLE"])),
_priv: ()
}
        }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ActiveShaderProgram(&self, pipeline: types::GLuint, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.ActiveShaderProgram.f)(pipeline, program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ActiveTexture(&self, texture: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.ActiveTexture.f)(texture) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn AttachShader(&self, program: types::GLuint, shader: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.AttachShader.f)(program, shader) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BeginConditionalRender(&self, id: types::GLuint, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(self.BeginConditionalRender.f)(id, mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BeginQuery(&self, target: types::GLenum, id: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.BeginQuery.f)(target, id) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BeginQueryIndexed(&self, target: types::GLenum, index: types::GLuint, id: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint) -> ()>(self.BeginQueryIndexed.f)(target, index, id) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BeginTransformFeedback(&self, primitiveMode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.BeginTransformFeedback.f)(primitiveMode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindAttribLocation(&self, program: types::GLuint, index: types::GLuint, name: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, *const types::GLchar) -> ()>(self.BindAttribLocation.f)(program, index, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindBuffer(&self, target: types::GLenum, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.BindBuffer.f)(target, buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindBufferBase(&self, target: types::GLenum, index: types::GLuint, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint) -> ()>(self.BindBufferBase.f)(target, index, buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindBufferRange(&self, target: types::GLenum, index: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(self.BindBufferRange.f)(target, index, buffer, offset, size) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindBuffersBase(&self, target: types::GLenum, first: types::GLuint, count: types::GLsizei, buffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *const types::GLuint) -> ()>(self.BindBuffersBase.f)(target, first, count, buffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindBuffersRange(&self, target: types::GLenum, first: types::GLuint, count: types::GLsizei, buffers: *const types::GLuint, offsets: *const types::GLintptr, sizes: *const types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *const types::GLuint, *const types::GLintptr, *const types::GLsizeiptr) -> ()>(self.BindBuffersRange.f)(target, first, count, buffers, offsets, sizes) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindFragDataLocation(&self, program: types::GLuint, color: types::GLuint, name: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, *const types::GLchar) -> ()>(self.BindFragDataLocation.f)(program, color, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindFragDataLocationIndexed(&self, program: types::GLuint, colorNumber: types::GLuint, index: types::GLuint, name: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, *const types::GLchar) -> ()>(self.BindFragDataLocationIndexed.f)(program, colorNumber, index, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindFramebuffer(&self, target: types::GLenum, framebuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.BindFramebuffer.f)(target, framebuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindImageTexture(&self, unit: types::GLuint, texture: types::GLuint, level: types::GLint, layered: types::GLboolean, layer: types::GLint, access: types::GLenum, format: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLint, types::GLboolean, types::GLint, types::GLenum, types::GLenum) -> ()>(self.BindImageTexture.f)(unit, texture, level, layered, layer, access, format) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindImageTextures(&self, first: types::GLuint, count: types::GLsizei, textures: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLuint) -> ()>(self.BindImageTextures.f)(first, count, textures) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindProgramPipeline(&self, pipeline: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.BindProgramPipeline.f)(pipeline) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindRenderbuffer(&self, target: types::GLenum, renderbuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.BindRenderbuffer.f)(target, renderbuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindSampler(&self, unit: types::GLuint, sampler: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.BindSampler.f)(unit, sampler) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindSamplers(&self, first: types::GLuint, count: types::GLsizei, samplers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLuint) -> ()>(self.BindSamplers.f)(first, count, samplers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindTexture(&self, target: types::GLenum, texture: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.BindTexture.f)(target, texture) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindTextureUnit(&self, unit: types::GLuint, texture: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.BindTextureUnit.f)(unit, texture) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindTextures(&self, first: types::GLuint, count: types::GLsizei, textures: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLuint) -> ()>(self.BindTextures.f)(first, count, textures) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindTransformFeedback(&self, target: types::GLenum, id: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.BindTransformFeedback.f)(target, id) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindVertexArray(&self, array: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.BindVertexArray.f)(array) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindVertexBuffer(&self, bindingindex: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, stride: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLintptr, types::GLsizei) -> ()>(self.BindVertexBuffer.f)(bindingindex, buffer, offset, stride) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindVertexBuffers(&self, first: types::GLuint, count: types::GLsizei, buffers: *const types::GLuint, offsets: *const types::GLintptr, strides: *const types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLuint, *const types::GLintptr, *const types::GLsizei) -> ()>(self.BindVertexBuffers.f)(first, count, buffers, offsets, strides) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendColor(&self, red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.BlendColor.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendEquation(&self, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.BlendEquation.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendEquationSeparate(&self, modeRGB: types::GLenum, modeAlpha: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(self.BlendEquationSeparate.f)(modeRGB, modeAlpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendEquationSeparatei(&self, buf: types::GLuint, modeRGB: types::GLenum, modeAlpha: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum) -> ()>(self.BlendEquationSeparatei.f)(buf, modeRGB, modeAlpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendEquationSeparateiARB(&self, buf: types::GLuint, modeRGB: types::GLenum, modeAlpha: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum) -> ()>(self.BlendEquationSeparateiARB.f)(buf, modeRGB, modeAlpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendEquationi(&self, buf: types::GLuint, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(self.BlendEquationi.f)(buf, mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendEquationiARB(&self, buf: types::GLuint, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(self.BlendEquationiARB.f)(buf, mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendFunc(&self, sfactor: types::GLenum, dfactor: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(self.BlendFunc.f)(sfactor, dfactor) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendFuncSeparate(&self, sfactorRGB: types::GLenum, dfactorRGB: types::GLenum, sfactorAlpha: types::GLenum, dfactorAlpha: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLenum) -> ()>(self.BlendFuncSeparate.f)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendFuncSeparatei(&self, buf: types::GLuint, srcRGB: types::GLenum, dstRGB: types::GLenum, srcAlpha: types::GLenum, dstAlpha: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum, types::GLenum, types::GLenum) -> ()>(self.BlendFuncSeparatei.f)(buf, srcRGB, dstRGB, srcAlpha, dstAlpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendFuncSeparateiARB(&self, buf: types::GLuint, srcRGB: types::GLenum, dstRGB: types::GLenum, srcAlpha: types::GLenum, dstAlpha: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum, types::GLenum, types::GLenum) -> ()>(self.BlendFuncSeparateiARB.f)(buf, srcRGB, dstRGB, srcAlpha, dstAlpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendFunci(&self, buf: types::GLuint, src: types::GLenum, dst: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum) -> ()>(self.BlendFunci.f)(buf, src, dst) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendFunciARB(&self, buf: types::GLuint, src: types::GLenum, dst: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum) -> ()>(self.BlendFunciARB.f)(buf, src, dst) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlitFramebuffer(&self, srcX0: types::GLint, srcY0: types::GLint, srcX1: types::GLint, srcY1: types::GLint, dstX0: types::GLint, dstY0: types::GLint, dstX1: types::GLint, dstY1: types::GLint, mask: types::GLbitfield, filter: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLbitfield, types::GLenum) -> ()>(self.BlitFramebuffer.f)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlitNamedFramebuffer(&self, readFramebuffer: types::GLuint, drawFramebuffer: types::GLuint, srcX0: types::GLint, srcY0: types::GLint, srcX1: types::GLint, srcY1: types::GLint, dstX0: types::GLint, dstY0: types::GLint, dstX1: types::GLint, dstY1: types::GLint, mask: types::GLbitfield, filter: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLbitfield, types::GLenum) -> ()>(self.BlitNamedFramebuffer.f)(readFramebuffer, drawFramebuffer, srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BufferData(&self, target: types::GLenum, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void, usage: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizeiptr, *const __gl_imports::raw::c_void, types::GLenum) -> ()>(self.BufferData.f)(target, size, data, usage) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BufferStorage(&self, target: types::GLenum, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void, flags: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizeiptr, *const __gl_imports::raw::c_void, types::GLbitfield) -> ()>(self.BufferStorage.f)(target, size, data, flags) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BufferSubData(&self, target: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLintptr, types::GLsizeiptr, *const __gl_imports::raw::c_void) -> ()>(self.BufferSubData.f)(target, offset, size, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CheckFramebufferStatus(&self, target: types::GLenum) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLenum>(self.CheckFramebufferStatus.f)(target) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CheckNamedFramebufferStatus(&self, framebuffer: types::GLuint, target: types::GLenum) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> types::GLenum>(self.CheckNamedFramebufferStatus.f)(framebuffer, target) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClampColor(&self, target: types::GLenum, clamp: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(self.ClampColor.f)(target, clamp) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Clear(&self, mask: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(self.Clear.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearBufferData(&self, target: types::GLenum, internalformat: types::GLenum, format: types::GLenum, type_: types::GLenum, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.ClearBufferData.f)(target, internalformat, format, type_, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearBufferSubData(&self, target: types::GLenum, internalformat: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, format: types::GLenum, type_: types::GLenum, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLintptr, types::GLsizeiptr, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.ClearBufferSubData.f)(target, internalformat, offset, size, format, type_, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearBufferfi(&self, buffer: types::GLenum, drawbuffer: types::GLint, depth: types::GLfloat, stencil: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLfloat, types::GLint) -> ()>(self.ClearBufferfi.f)(buffer, drawbuffer, depth, stencil) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearBufferfv(&self, buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, *const types::GLfloat) -> ()>(self.ClearBufferfv.f)(buffer, drawbuffer, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearBufferiv(&self, buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, *const types::GLint) -> ()>(self.ClearBufferiv.f)(buffer, drawbuffer, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearBufferuiv(&self, buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, *const types::GLuint) -> ()>(self.ClearBufferuiv.f)(buffer, drawbuffer, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearColor(&self, red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ClearColor.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearDepth(&self, depth: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble) -> ()>(self.ClearDepth.f)(depth) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearDepthf(&self, d: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.ClearDepthf.f)(d) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearNamedBufferData(&self, buffer: types::GLuint, internalformat: types::GLenum, format: types::GLenum, type_: types::GLenum, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.ClearNamedBufferData.f)(buffer, internalformat, format, type_, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearNamedBufferSubData(&self, buffer: types::GLuint, internalformat: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, format: types::GLenum, type_: types::GLenum, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLintptr, types::GLsizeiptr, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.ClearNamedBufferSubData.f)(buffer, internalformat, offset, size, format, type_, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearNamedFramebufferfi(&self, framebuffer: types::GLuint, buffer: types::GLenum, drawbuffer: types::GLint, depth: types::GLfloat, stencil: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint, types::GLfloat, types::GLint) -> ()>(self.ClearNamedFramebufferfi.f)(framebuffer, buffer, drawbuffer, depth, stencil) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearNamedFramebufferfv(&self, framebuffer: types::GLuint, buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint, *const types::GLfloat) -> ()>(self.ClearNamedFramebufferfv.f)(framebuffer, buffer, drawbuffer, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearNamedFramebufferiv(&self, framebuffer: types::GLuint, buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint, *const types::GLint) -> ()>(self.ClearNamedFramebufferiv.f)(framebuffer, buffer, drawbuffer, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearNamedFramebufferuiv(&self, framebuffer: types::GLuint, buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint, *const types::GLuint) -> ()>(self.ClearNamedFramebufferuiv.f)(framebuffer, buffer, drawbuffer, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearStencil(&self, s: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(self.ClearStencil.f)(s) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearTexImage(&self, texture: types::GLuint, level: types::GLint, format: types::GLenum, type_: types::GLenum, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.ClearTexImage.f)(texture, level, format, type_, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearTexSubImage(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, type_: types::GLenum, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.ClearTexSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClientWaitSync(&self, sync: types::GLsync, flags: types::GLbitfield, timeout: types::GLuint64) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync, types::GLbitfield, types::GLuint64) -> types::GLenum>(self.ClientWaitSync.f)(sync, flags, timeout) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClipControl(&self, origin: types::GLenum, depth: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(self.ClipControl.f)(origin, depth) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ColorMask(&self, red: types::GLboolean, green: types::GLboolean, blue: types::GLboolean, alpha: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean, types::GLboolean, types::GLboolean, types::GLboolean) -> ()>(self.ColorMask.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ColorMaski(&self, index: types::GLuint, r: types::GLboolean, g: types::GLboolean, b: types::GLboolean, a: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLboolean, types::GLboolean, types::GLboolean, types::GLboolean) -> ()>(self.ColorMaski.f)(index, r, g, b, a) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ColorP3ui(&self, type_: types::GLenum, color: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.ColorP3ui.f)(type_, color) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ColorP3uiv(&self, type_: types::GLenum, color: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(self.ColorP3uiv.f)(type_, color) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ColorP4ui(&self, type_: types::GLenum, color: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.ColorP4ui.f)(type_, color) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ColorP4uiv(&self, type_: types::GLenum, color: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(self.ColorP4uiv.f)(type_, color) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompileShader(&self, shader: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.CompileShader.f)(shader) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTexImage1D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLsizei, types::GLint, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.CompressedTexImage1D.f)(target, level, internalformat, width, border, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTexImage2D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLsizei, types::GLsizei, types::GLint, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.CompressedTexImage2D.f)(target, level, internalformat, width, height, border, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTexImage3D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLsizei, types::GLsizei, types::GLsizei, types::GLint, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.CompressedTexImage3D.f)(target, level, internalformat, width, height, depth, border, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTexSubImage1D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.CompressedTexSubImage1D.f)(target, level, xoffset, width, format, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTexSubImage2D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.CompressedTexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTexSubImage3D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.CompressedTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTextureSubImage1D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLsizei, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.CompressedTextureSubImage1D.f)(texture, level, xoffset, width, format, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTextureSubImage2D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.CompressedTextureSubImage2D.f)(texture, level, xoffset, yoffset, width, height, format, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTextureSubImage3D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.CompressedTextureSubImage3D.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyBufferSubData(&self, readTarget: types::GLenum, writeTarget: types::GLenum, readOffset: types::GLintptr, writeOffset: types::GLintptr, size: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLintptr, types::GLintptr, types::GLsizeiptr) -> ()>(self.CopyBufferSubData.f)(readTarget, writeTarget, readOffset, writeOffset, size) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyImageSubData(&self, srcName: types::GLuint, srcTarget: types::GLenum, srcLevel: types::GLint, srcX: types::GLint, srcY: types::GLint, srcZ: types::GLint, dstName: types::GLuint, dstTarget: types::GLenum, dstLevel: types::GLint, dstX: types::GLint, dstY: types::GLint, dstZ: types::GLint, srcWidth: types::GLsizei, srcHeight: types::GLsizei, srcDepth: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLuint, types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei) -> ()>(self.CopyImageSubData.f)(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyNamedBufferSubData(&self, readBuffer: types::GLuint, writeBuffer: types::GLuint, readOffset: types::GLintptr, writeOffset: types::GLintptr, size: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLintptr, types::GLintptr, types::GLsizeiptr) -> ()>(self.CopyNamedBufferSubData.f)(readBuffer, writeBuffer, readOffset, writeOffset, size) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTexImage1D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, border: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLint) -> ()>(self.CopyTexImage1D.f)(target, level, internalformat, x, y, width, border) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTexImage2D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLint) -> ()>(self.CopyTexImage2D.f)(target, level, internalformat, x, y, width, height, border) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTexSubImage1D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei) -> ()>(self.CopyTexSubImage1D.f)(target, level, xoffset, x, y, width) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTexSubImage2D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.CopyTexSubImage2D.f)(target, level, xoffset, yoffset, x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTexSubImage3D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.CopyTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTextureSubImage1D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei) -> ()>(self.CopyTextureSubImage1D.f)(texture, level, xoffset, x, y, width) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTextureSubImage2D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.CopyTextureSubImage2D.f)(texture, level, xoffset, yoffset, x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTextureSubImage3D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.CopyTextureSubImage3D.f)(texture, level, xoffset, yoffset, zoffset, x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateBuffers(&self, n: types::GLsizei, buffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.CreateBuffers.f)(n, buffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateFramebuffers(&self, n: types::GLsizei, framebuffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.CreateFramebuffers.f)(n, framebuffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateProgram(&self, ) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLuint>(self.CreateProgram.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateProgramPipelines(&self, n: types::GLsizei, pipelines: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.CreateProgramPipelines.f)(n, pipelines) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateQueries(&self, target: types::GLenum, n: types::GLsizei, ids: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *mut types::GLuint) -> ()>(self.CreateQueries.f)(target, n, ids) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateRenderbuffers(&self, n: types::GLsizei, renderbuffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.CreateRenderbuffers.f)(n, renderbuffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateSamplers(&self, n: types::GLsizei, samplers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.CreateSamplers.f)(n, samplers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateShader(&self, type_: types::GLenum) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLuint>(self.CreateShader.f)(type_) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateShaderProgramv(&self, type_: types::GLenum, count: types::GLsizei, strings: *const *const types::GLchar) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const *const types::GLchar) -> types::GLuint>(self.CreateShaderProgramv.f)(type_, count, strings) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateTextures(&self, target: types::GLenum, n: types::GLsizei, textures: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *mut types::GLuint) -> ()>(self.CreateTextures.f)(target, n, textures) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateTransformFeedbacks(&self, n: types::GLsizei, ids: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.CreateTransformFeedbacks.f)(n, ids) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateVertexArrays(&self, n: types::GLsizei, arrays: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.CreateVertexArrays.f)(n, arrays) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CullFace(&self, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.CullFace.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DebugMessageCallback(&self, callback: types::GLDEBUGPROC, userParam: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLDEBUGPROC, *const __gl_imports::raw::c_void) -> ()>(self.DebugMessageCallback.f)(callback, userParam) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DebugMessageControl(&self, source: types::GLenum, type_: types::GLenum, severity: types::GLenum, count: types::GLsizei, ids: *const types::GLuint, enabled: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLsizei, *const types::GLuint, types::GLboolean) -> ()>(self.DebugMessageControl.f)(source, type_, severity, count, ids, enabled) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DebugMessageInsert(&self, source: types::GLenum, type_: types::GLenum, id: types::GLuint, severity: types::GLenum, length: types::GLsizei, buf: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint, types::GLenum, types::GLsizei, *const types::GLchar) -> ()>(self.DebugMessageInsert.f)(source, type_, id, severity, length, buf) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteBuffers(&self, n: types::GLsizei, buffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteBuffers.f)(n, buffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteFramebuffers(&self, n: types::GLsizei, framebuffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteFramebuffers.f)(n, framebuffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteProgram(&self, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.DeleteProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteProgramPipelines(&self, n: types::GLsizei, pipelines: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteProgramPipelines.f)(n, pipelines) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteQueries(&self, n: types::GLsizei, ids: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteQueries.f)(n, ids) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteRenderbuffers(&self, n: types::GLsizei, renderbuffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteRenderbuffers.f)(n, renderbuffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteSamplers(&self, count: types::GLsizei, samplers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteSamplers.f)(count, samplers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteShader(&self, shader: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.DeleteShader.f)(shader) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteSync(&self, sync: types::GLsync) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync) -> ()>(self.DeleteSync.f)(sync) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteTextures(&self, n: types::GLsizei, textures: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteTextures.f)(n, textures) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteTransformFeedbacks(&self, n: types::GLsizei, ids: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteTransformFeedbacks.f)(n, ids) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteVertexArrays(&self, n: types::GLsizei, arrays: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteVertexArrays.f)(n, arrays) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DepthFunc(&self, func: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.DepthFunc.f)(func) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DepthMask(&self, flag: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean) -> ()>(self.DepthMask.f)(flag) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DepthRange(&self, near: types::GLdouble, far: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble) -> ()>(self.DepthRange.f)(near, far) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DepthRangeArrayv(&self, first: types::GLuint, count: types::GLsizei, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLdouble) -> ()>(self.DepthRangeArrayv.f)(first, count, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DepthRangeIndexed(&self, index: types::GLuint, n: types::GLdouble, f: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble) -> ()>(self.DepthRangeIndexed.f)(index, n, f) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DepthRangef(&self, n: types::GLfloat, f: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(self.DepthRangef.f)(n, f) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DetachShader(&self, program: types::GLuint, shader: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.DetachShader.f)(program, shader) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Disable(&self, cap: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.Disable.f)(cap) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DisableVertexArrayAttrib(&self, vaobj: types::GLuint, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.DisableVertexArrayAttrib.f)(vaobj, index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DisableVertexAttribArray(&self, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.DisableVertexAttribArray.f)(index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Disablei(&self, target: types::GLenum, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.Disablei.f)(target, index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DispatchCompute(&self, num_groups_x: types::GLuint, num_groups_y: types::GLuint, num_groups_z: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(self.DispatchCompute.f)(num_groups_x, num_groups_y, num_groups_z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DispatchComputeIndirect(&self, indirect: types::GLintptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLintptr) -> ()>(self.DispatchComputeIndirect.f)(indirect) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawArrays(&self, mode: types::GLenum, first: types::GLint, count: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLsizei) -> ()>(self.DrawArrays.f)(mode, first, count) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawArraysIndirect(&self, mode: types::GLenum, indirect: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.DrawArraysIndirect.f)(mode, indirect) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawArraysInstanced(&self, mode: types::GLenum, first: types::GLint, count: types::GLsizei, instancecount: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.DrawArraysInstanced.f)(mode, first, count, instancecount) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawArraysInstancedBaseInstance(&self, mode: types::GLenum, first: types::GLint, count: types::GLsizei, instancecount: types::GLsizei, baseinstance: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLsizei, types::GLsizei, types::GLuint) -> ()>(self.DrawArraysInstancedBaseInstance.f)(mode, first, count, instancecount, baseinstance) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawBuffer(&self, buf: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.DrawBuffer.f)(buf) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawBuffers(&self, n: types::GLsizei, bufs: *const types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLenum) -> ()>(self.DrawBuffers.f)(n, bufs) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawElements(&self, mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.DrawElements.f)(mode, count, type_, indices) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawElementsBaseVertex(&self, mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, basevertex: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void, types::GLint) -> ()>(self.DrawElementsBaseVertex.f)(mode, count, type_, indices, basevertex) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawElementsIndirect(&self, mode: types::GLenum, type_: types::GLenum, indirect: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.DrawElementsIndirect.f)(mode, type_, indirect) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawElementsInstanced(&self, mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, instancecount: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void, types::GLsizei) -> ()>(self.DrawElementsInstanced.f)(mode, count, type_, indices, instancecount) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawElementsInstancedBaseInstance(&self, mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, instancecount: types::GLsizei, baseinstance: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void, types::GLsizei, types::GLuint) -> ()>(self.DrawElementsInstancedBaseInstance.f)(mode, count, type_, indices, instancecount, baseinstance) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawElementsInstancedBaseVertex(&self, mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, instancecount: types::GLsizei, basevertex: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void, types::GLsizei, types::GLint) -> ()>(self.DrawElementsInstancedBaseVertex.f)(mode, count, type_, indices, instancecount, basevertex) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawElementsInstancedBaseVertexBaseInstance(&self, mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, instancecount: types::GLsizei, basevertex: types::GLint, baseinstance: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void, types::GLsizei, types::GLint, types::GLuint) -> ()>(self.DrawElementsInstancedBaseVertexBaseInstance.f)(mode, count, type_, indices, instancecount, basevertex, baseinstance) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawRangeElements(&self, mode: types::GLenum, start: types::GLuint, end: types::GLuint, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.DrawRangeElements.f)(mode, start, end, count, type_, indices) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawRangeElementsBaseVertex(&self, mode: types::GLenum, start: types::GLuint, end: types::GLuint, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, basevertex: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void, types::GLint) -> ()>(self.DrawRangeElementsBaseVertex.f)(mode, start, end, count, type_, indices, basevertex) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawTransformFeedback(&self, mode: types::GLenum, id: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.DrawTransformFeedback.f)(mode, id) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawTransformFeedbackInstanced(&self, mode: types::GLenum, id: types::GLuint, instancecount: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei) -> ()>(self.DrawTransformFeedbackInstanced.f)(mode, id, instancecount) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawTransformFeedbackStream(&self, mode: types::GLenum, id: types::GLuint, stream: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint) -> ()>(self.DrawTransformFeedbackStream.f)(mode, id, stream) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawTransformFeedbackStreamInstanced(&self, mode: types::GLenum, id: types::GLuint, stream: types::GLuint, instancecount: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint, types::GLsizei) -> ()>(self.DrawTransformFeedbackStreamInstanced.f)(mode, id, stream, instancecount) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Enable(&self, cap: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.Enable.f)(cap) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn EnableVertexArrayAttrib(&self, vaobj: types::GLuint, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.EnableVertexArrayAttrib.f)(vaobj, index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn EnableVertexAttribArray(&self, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.EnableVertexAttribArray.f)(index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Enablei(&self, target: types::GLenum, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.Enablei.f)(target, index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn EndConditionalRender(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.EndConditionalRender.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn EndQuery(&self, target: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.EndQuery.f)(target) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn EndQueryIndexed(&self, target: types::GLenum, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.EndQueryIndexed.f)(target, index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn EndTransformFeedback(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.EndTransformFeedback.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FenceSync(&self, condition: types::GLenum, flags: types::GLbitfield) -> types::GLsync { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLbitfield) -> types::GLsync>(self.FenceSync.f)(condition, flags) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Finish(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.Finish.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Flush(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.Flush.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FlushMappedBufferRange(&self, target: types::GLenum, offset: types::GLintptr, length: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLintptr, types::GLsizeiptr) -> ()>(self.FlushMappedBufferRange.f)(target, offset, length) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FlushMappedNamedBufferRange(&self, buffer: types::GLuint, offset: types::GLintptr, length: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(self.FlushMappedNamedBufferRange.f)(buffer, offset, length) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FramebufferParameteri(&self, target: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(self.FramebufferParameteri.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FramebufferRenderbuffer(&self, target: types::GLenum, attachment: types::GLenum, renderbuffertarget: types::GLenum, renderbuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLuint) -> ()>(self.FramebufferRenderbuffer.f)(target, attachment, renderbuffertarget, renderbuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FramebufferTexture(&self, target: types::GLenum, attachment: types::GLenum, texture: types::GLuint, level: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint, types::GLint) -> ()>(self.FramebufferTexture.f)(target, attachment, texture, level) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FramebufferTexture1D(&self, target: types::GLenum, attachment: types::GLenum, textarget: types::GLenum, texture: types::GLuint, level: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLuint, types::GLint) -> ()>(self.FramebufferTexture1D.f)(target, attachment, textarget, texture, level) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FramebufferTexture2D(&self, target: types::GLenum, attachment: types::GLenum, textarget: types::GLenum, texture: types::GLuint, level: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLuint, types::GLint) -> ()>(self.FramebufferTexture2D.f)(target, attachment, textarget, texture, level) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FramebufferTexture3D(&self, target: types::GLenum, attachment: types::GLenum, textarget: types::GLenum, texture: types::GLuint, level: types::GLint, zoffset: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLuint, types::GLint, types::GLint) -> ()>(self.FramebufferTexture3D.f)(target, attachment, textarget, texture, level, zoffset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FramebufferTextureLayer(&self, target: types::GLenum, attachment: types::GLenum, texture: types::GLuint, level: types::GLint, layer: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint, types::GLint, types::GLint) -> ()>(self.FramebufferTextureLayer.f)(target, attachment, texture, level, layer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FrontFace(&self, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.FrontFace.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenBuffers(&self, n: types::GLsizei, buffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenBuffers.f)(n, buffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenFramebuffers(&self, n: types::GLsizei, framebuffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenFramebuffers.f)(n, framebuffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenProgramPipelines(&self, n: types::GLsizei, pipelines: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenProgramPipelines.f)(n, pipelines) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenQueries(&self, n: types::GLsizei, ids: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenQueries.f)(n, ids) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenRenderbuffers(&self, n: types::GLsizei, renderbuffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenRenderbuffers.f)(n, renderbuffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenSamplers(&self, count: types::GLsizei, samplers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenSamplers.f)(count, samplers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenTextures(&self, n: types::GLsizei, textures: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenTextures.f)(n, textures) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenTransformFeedbacks(&self, n: types::GLsizei, ids: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenTransformFeedbacks.f)(n, ids) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenVertexArrays(&self, n: types::GLsizei, arrays: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenVertexArrays.f)(n, arrays) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenerateMipmap(&self, target: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.GenerateMipmap.f)(target) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenerateTextureMipmap(&self, texture: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.GenerateTextureMipmap.f)(texture) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveAtomicCounterBufferiv(&self, program: types::GLuint, bufferIndex: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetActiveAtomicCounterBufferiv.f)(program, bufferIndex, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveAttrib(&self, program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLint, type_: *mut types::GLenum, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLint, *mut types::GLenum, *mut types::GLchar) -> ()>(self.GetActiveAttrib.f)(program, index, bufSize, length, size, type_, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveSubroutineName(&self, program: types::GLuint, shadertype: types::GLenum, index: types::GLuint, bufsize: types::GLsizei, length: *mut types::GLsizei, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetActiveSubroutineName.f)(program, shadertype, index, bufsize, length, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveSubroutineUniformName(&self, program: types::GLuint, shadertype: types::GLenum, index: types::GLuint, bufsize: types::GLsizei, length: *mut types::GLsizei, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetActiveSubroutineUniformName.f)(program, shadertype, index, bufsize, length, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveSubroutineUniformiv(&self, program: types::GLuint, shadertype: types::GLenum, index: types::GLuint, pname: types::GLenum, values: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetActiveSubroutineUniformiv.f)(program, shadertype, index, pname, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveUniform(&self, program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLint, type_: *mut types::GLenum, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLint, *mut types::GLenum, *mut types::GLchar) -> ()>(self.GetActiveUniform.f)(program, index, bufSize, length, size, type_, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveUniformBlockName(&self, program: types::GLuint, uniformBlockIndex: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, uniformBlockName: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetActiveUniformBlockName.f)(program, uniformBlockIndex, bufSize, length, uniformBlockName) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveUniformBlockiv(&self, program: types::GLuint, uniformBlockIndex: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetActiveUniformBlockiv.f)(program, uniformBlockIndex, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveUniformName(&self, program: types::GLuint, uniformIndex: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, uniformName: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetActiveUniformName.f)(program, uniformIndex, bufSize, length, uniformName) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveUniformsiv(&self, program: types::GLuint, uniformCount: types::GLsizei, uniformIndices: *const types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetActiveUniformsiv.f)(program, uniformCount, uniformIndices, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetAttachedShaders(&self, program: types::GLuint, maxCount: types::GLsizei, count: *mut types::GLsizei, shaders: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLuint) -> ()>(self.GetAttachedShaders.f)(program, maxCount, count, shaders) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetAttribLocation(&self, program: types::GLuint, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(self.GetAttribLocation.f)(program, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetBooleani_v(&self, target: types::GLenum, index: types::GLuint, data: *mut types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLboolean) -> ()>(self.GetBooleani_v.f)(target, index, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetBooleanv(&self, pname: types::GLenum, data: *mut types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLboolean) -> ()>(self.GetBooleanv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetBufferParameteri64v(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint64) -> ()>(self.GetBufferParameteri64v.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetBufferParameteriv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetBufferParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetBufferPointerv(&self, target: types::GLenum, pname: types::GLenum, params: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const *mut __gl_imports::raw::c_void) -> ()>(self.GetBufferPointerv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetBufferSubData(&self, target: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, data: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLintptr, types::GLsizeiptr, *mut __gl_imports::raw::c_void) -> ()>(self.GetBufferSubData.f)(target, offset, size, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetCompressedTexImage(&self, target: types::GLenum, level: types::GLint, img: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, *mut __gl_imports::raw::c_void) -> ()>(self.GetCompressedTexImage.f)(target, level, img) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetCompressedTextureImage(&self, texture: types::GLuint, level: types::GLint, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.GetCompressedTextureImage.f)(texture, level, bufSize, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetCompressedTextureSubImage(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.GetCompressedTextureSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetDebugMessageLog(&self, count: types::GLuint, bufSize: types::GLsizei, sources: *mut types::GLenum, types: *mut types::GLenum, ids: *mut types::GLuint, severities: *mut types::GLenum, lengths: *mut types::GLsizei, messageLog: *mut types::GLchar) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLenum, *mut types::GLenum, *mut types::GLuint, *mut types::GLenum, *mut types::GLsizei, *mut types::GLchar) -> types::GLuint>(self.GetDebugMessageLog.f)(count, bufSize, sources, types, ids, severities, lengths, messageLog) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetDoublei_v(&self, target: types::GLenum, index: types::GLuint, data: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLdouble) -> ()>(self.GetDoublei_v.f)(target, index, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetDoublev(&self, pname: types::GLenum, data: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLdouble) -> ()>(self.GetDoublev.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetError(&self, ) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLenum>(self.GetError.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetFloati_v(&self, target: types::GLenum, index: types::GLuint, data: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLfloat) -> ()>(self.GetFloati_v.f)(target, index, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetFloatv(&self, pname: types::GLenum, data: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLfloat) -> ()>(self.GetFloatv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetFragDataIndex(&self, program: types::GLuint, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(self.GetFragDataIndex.f)(program, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetFragDataLocation(&self, program: types::GLuint, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(self.GetFragDataLocation.f)(program, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetFramebufferAttachmentParameteriv(&self, target: types::GLenum, attachment: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetFramebufferAttachmentParameteriv.f)(target, attachment, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetFramebufferParameteriv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetFramebufferParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetGraphicsResetStatus(&self, ) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLenum>(self.GetGraphicsResetStatus.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetInteger64i_v(&self, target: types::GLenum, index: types::GLuint, data: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLint64) -> ()>(self.GetInteger64i_v.f)(target, index, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetInteger64v(&self, pname: types::GLenum, data: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLint64) -> ()>(self.GetInteger64v.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetIntegeri_v(&self, target: types::GLenum, index: types::GLuint, data: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLint) -> ()>(self.GetIntegeri_v.f)(target, index, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetIntegerv(&self, pname: types::GLenum, data: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLint) -> ()>(self.GetIntegerv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetInternalformati64v(&self, target: types::GLenum, internalformat: types::GLenum, pname: types::GLenum, bufSize: types::GLsizei, params: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLsizei, *mut types::GLint64) -> ()>(self.GetInternalformati64v.f)(target, internalformat, pname, bufSize, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetInternalformativ(&self, target: types::GLenum, internalformat: types::GLenum, pname: types::GLenum, bufSize: types::GLsizei, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLsizei, *mut types::GLint) -> ()>(self.GetInternalformativ.f)(target, internalformat, pname, bufSize, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetMultisamplefv(&self, pname: types::GLenum, index: types::GLuint, val: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLfloat) -> ()>(self.GetMultisamplefv.f)(pname, index, val) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetNamedBufferParameteri64v(&self, buffer: types::GLuint, pname: types::GLenum, params: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint64) -> ()>(self.GetNamedBufferParameteri64v.f)(buffer, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetNamedBufferParameteriv(&self, buffer: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetNamedBufferParameteriv.f)(buffer, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetNamedBufferPointerv(&self, buffer: types::GLuint, pname: types::GLenum, params: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const *mut __gl_imports::raw::c_void) -> ()>(self.GetNamedBufferPointerv.f)(buffer, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetNamedBufferSubData(&self, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr, data: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLintptr, types::GLsizeiptr, *mut __gl_imports::raw::c_void) -> ()>(self.GetNamedBufferSubData.f)(buffer, offset, size, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetNamedFramebufferAttachmentParameteriv(&self, framebuffer: types::GLuint, attachment: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetNamedFramebufferAttachmentParameteriv.f)(framebuffer, attachment, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetNamedFramebufferParameteriv(&self, framebuffer: types::GLuint, pname: types::GLenum, param: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetNamedFramebufferParameteriv.f)(framebuffer, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetNamedRenderbufferParameteriv(&self, renderbuffer: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetNamedRenderbufferParameteriv.f)(renderbuffer, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetObjectLabel(&self, identifier: types::GLenum, name: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, label: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetObjectLabel.f)(identifier, name, bufSize, length, label) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetObjectPtrLabel(&self, ptr: *const __gl_imports::raw::c_void, bufSize: types::GLsizei, length: *mut types::GLsizei, label: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const __gl_imports::raw::c_void, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetObjectPtrLabel.f)(ptr, bufSize, length, label) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetPointerv(&self, pname: types::GLenum, params: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const *mut __gl_imports::raw::c_void) -> ()>(self.GetPointerv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramBinary(&self, program: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, binaryFormat: *mut types::GLenum, binary: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLenum, *mut __gl_imports::raw::c_void) -> ()>(self.GetProgramBinary.f)(program, bufSize, length, binaryFormat, binary) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramInfoLog(&self, program: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetProgramInfoLog.f)(program, bufSize, length, infoLog) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramInterfaceiv(&self, program: types::GLuint, programInterface: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetProgramInterfaceiv.f)(program, programInterface, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramPipelineInfoLog(&self, pipeline: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetProgramPipelineInfoLog.f)(pipeline, bufSize, length, infoLog) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramPipelineiv(&self, pipeline: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetProgramPipelineiv.f)(pipeline, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramResourceIndex(&self, program: types::GLuint, programInterface: types::GLenum, name: *const types::GLchar) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLchar) -> types::GLuint>(self.GetProgramResourceIndex.f)(program, programInterface, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramResourceLocation(&self, program: types::GLuint, programInterface: types::GLenum, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLchar) -> types::GLint>(self.GetProgramResourceLocation.f)(program, programInterface, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramResourceLocationIndex(&self, program: types::GLuint, programInterface: types::GLenum, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLchar) -> types::GLint>(self.GetProgramResourceLocationIndex.f)(program, programInterface, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramResourceName(&self, program: types::GLuint, programInterface: types::GLenum, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetProgramResourceName.f)(program, programInterface, index, bufSize, length, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramResourceiv(&self, program: types::GLuint, programInterface: types::GLenum, index: types::GLuint, propCount: types::GLsizei, props: *const types::GLenum, bufSize: types::GLsizei, length: *mut types::GLsizei, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLsizei, *const types::GLenum, types::GLsizei, *mut types::GLsizei, *mut types::GLint) -> ()>(self.GetProgramResourceiv.f)(program, programInterface, index, propCount, props, bufSize, length, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramStageiv(&self, program: types::GLuint, shadertype: types::GLenum, pname: types::GLenum, values: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetProgramStageiv.f)(program, shadertype, pname, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramiv(&self, program: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetProgramiv.f)(program, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryBufferObjecti64v(&self, id: types::GLuint, buffer: types::GLuint, pname: types::GLenum, offset: types::GLintptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, types::GLintptr) -> ()>(self.GetQueryBufferObjecti64v.f)(id, buffer, pname, offset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryBufferObjectiv(&self, id: types::GLuint, buffer: types::GLuint, pname: types::GLenum, offset: types::GLintptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, types::GLintptr) -> ()>(self.GetQueryBufferObjectiv.f)(id, buffer, pname, offset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryBufferObjectui64v(&self, id: types::GLuint, buffer: types::GLuint, pname: types::GLenum, offset: types::GLintptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, types::GLintptr) -> ()>(self.GetQueryBufferObjectui64v.f)(id, buffer, pname, offset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryBufferObjectuiv(&self, id: types::GLuint, buffer: types::GLuint, pname: types::GLenum, offset: types::GLintptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, types::GLintptr) -> ()>(self.GetQueryBufferObjectuiv.f)(id, buffer, pname, offset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryIndexediv(&self, target: types::GLenum, index: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetQueryIndexediv.f)(target, index, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryObjecti64v(&self, id: types::GLuint, pname: types::GLenum, params: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint64) -> ()>(self.GetQueryObjecti64v.f)(id, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryObjectiv(&self, id: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetQueryObjectiv.f)(id, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryObjectui64v(&self, id: types::GLuint, pname: types::GLenum, params: *mut types::GLuint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint64) -> ()>(self.GetQueryObjectui64v.f)(id, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryObjectuiv(&self, id: types::GLuint, pname: types::GLenum, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint) -> ()>(self.GetQueryObjectuiv.f)(id, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryiv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetQueryiv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetRenderbufferParameteriv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetRenderbufferParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetSamplerParameterIiv(&self, sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetSamplerParameterIiv.f)(sampler, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetSamplerParameterIuiv(&self, sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint) -> ()>(self.GetSamplerParameterIuiv.f)(sampler, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetSamplerParameterfv(&self, sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLfloat) -> ()>(self.GetSamplerParameterfv.f)(sampler, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetSamplerParameteriv(&self, sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetSamplerParameteriv.f)(sampler, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetShaderInfoLog(&self, shader: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetShaderInfoLog.f)(shader, bufSize, length, infoLog) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetShaderPrecisionFormat(&self, shadertype: types::GLenum, precisiontype: types::GLenum, range: *mut types::GLint, precision: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint, *mut types::GLint) -> ()>(self.GetShaderPrecisionFormat.f)(shadertype, precisiontype, range, precision) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetShaderSource(&self, shader: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, source: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetShaderSource.f)(shader, bufSize, length, source) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetShaderiv(&self, shader: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetShaderiv.f)(shader, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetString(&self, name: types::GLenum) -> *const types::GLubyte { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> *const types::GLubyte>(self.GetString.f)(name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetStringi(&self, name: types::GLenum, index: types::GLuint) -> *const types::GLubyte { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> *const types::GLubyte>(self.GetStringi.f)(name, index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetSubroutineIndex(&self, program: types::GLuint, shadertype: types::GLenum, name: *const types::GLchar) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLchar) -> types::GLuint>(self.GetSubroutineIndex.f)(program, shadertype, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetSubroutineUniformLocation(&self, program: types::GLuint, shadertype: types::GLenum, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLchar) -> types::GLint>(self.GetSubroutineUniformLocation.f)(program, shadertype, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetSynciv(&self, sync: types::GLsync, pname: types::GLenum, bufSize: types::GLsizei, length: *mut types::GLsizei, values: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync, types::GLenum, types::GLsizei, *mut types::GLsizei, *mut types::GLint) -> ()>(self.GetSynciv.f)(sync, pname, bufSize, length, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTexImage(&self, target: types::GLenum, level: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLenum, *mut __gl_imports::raw::c_void) -> ()>(self.GetTexImage.f)(target, level, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTexLevelParameterfv(&self, target: types::GLenum, level: types::GLint, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, *mut types::GLfloat) -> ()>(self.GetTexLevelParameterfv.f)(target, level, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTexLevelParameteriv(&self, target: types::GLenum, level: types::GLint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, *mut types::GLint) -> ()>(self.GetTexLevelParameteriv.f)(target, level, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTexParameterIiv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetTexParameterIiv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTexParameterIuiv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLuint) -> ()>(self.GetTexParameterIuiv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTexParameterfv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(self.GetTexParameterfv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTexParameteriv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetTexParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTextureImage(&self, texture: types::GLuint, level: types::GLint, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLenum, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.GetTextureImage.f)(texture, level, format, type_, bufSize, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTextureLevelParameterfv(&self, texture: types::GLuint, level: types::GLint, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, *mut types::GLfloat) -> ()>(self.GetTextureLevelParameterfv.f)(texture, level, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTextureLevelParameteriv(&self, texture: types::GLuint, level: types::GLint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, *mut types::GLint) -> ()>(self.GetTextureLevelParameteriv.f)(texture, level, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTextureParameterIiv(&self, texture: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetTextureParameterIiv.f)(texture, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTextureParameterIuiv(&self, texture: types::GLuint, pname: types::GLenum, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint) -> ()>(self.GetTextureParameterIuiv.f)(texture, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTextureParameterfv(&self, texture: types::GLuint, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLfloat) -> ()>(self.GetTextureParameterfv.f)(texture, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTextureParameteriv(&self, texture: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetTextureParameteriv.f)(texture, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTextureSubImage(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.GetTextureSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, bufSize, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTransformFeedbackVarying(&self, program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLsizei, type_: *mut types::GLenum, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLsizei, *mut types::GLenum, *mut types::GLchar) -> ()>(self.GetTransformFeedbackVarying.f)(program, index, bufSize, length, size, type_, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTransformFeedbacki64_v(&self, xfb: types::GLuint, pname: types::GLenum, index: types::GLuint, param: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, *mut types::GLint64) -> ()>(self.GetTransformFeedbacki64_v.f)(xfb, pname, index, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTransformFeedbacki_v(&self, xfb: types::GLuint, pname: types::GLenum, index: types::GLuint, param: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, *mut types::GLint) -> ()>(self.GetTransformFeedbacki_v.f)(xfb, pname, index, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTransformFeedbackiv(&self, xfb: types::GLuint, pname: types::GLenum, param: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetTransformFeedbackiv.f)(xfb, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformBlockIndex(&self, program: types::GLuint, uniformBlockName: *const types::GLchar) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLuint>(self.GetUniformBlockIndex.f)(program, uniformBlockName) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformIndices(&self, program: types::GLuint, uniformCount: types::GLsizei, uniformNames: *const *const types::GLchar, uniformIndices: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const *const types::GLchar, *mut types::GLuint) -> ()>(self.GetUniformIndices.f)(program, uniformCount, uniformNames, uniformIndices) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformLocation(&self, program: types::GLuint, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(self.GetUniformLocation.f)(program, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformSubroutineuiv(&self, shadertype: types::GLenum, location: types::GLint, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, *mut types::GLuint) -> ()>(self.GetUniformSubroutineuiv.f)(shadertype, location, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformdv(&self, program: types::GLuint, location: types::GLint, params: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLdouble) -> ()>(self.GetUniformdv.f)(program, location, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformfv(&self, program: types::GLuint, location: types::GLint, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLfloat) -> ()>(self.GetUniformfv.f)(program, location, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformiv(&self, program: types::GLuint, location: types::GLint, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLint) -> ()>(self.GetUniformiv.f)(program, location, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformuiv(&self, program: types::GLuint, location: types::GLint, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLuint) -> ()>(self.GetUniformuiv.f)(program, location, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexArrayIndexed64iv(&self, vaobj: types::GLuint, index: types::GLuint, pname: types::GLenum, param: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, *mut types::GLint64) -> ()>(self.GetVertexArrayIndexed64iv.f)(vaobj, index, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexArrayIndexediv(&self, vaobj: types::GLuint, index: types::GLuint, pname: types::GLenum, param: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetVertexArrayIndexediv.f)(vaobj, index, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexArrayiv(&self, vaobj: types::GLuint, pname: types::GLenum, param: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetVertexArrayiv.f)(vaobj, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribIiv(&self, index: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetVertexAttribIiv.f)(index, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribIuiv(&self, index: types::GLuint, pname: types::GLenum, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint) -> ()>(self.GetVertexAttribIuiv.f)(index, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribLdv(&self, index: types::GLuint, pname: types::GLenum, params: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLdouble) -> ()>(self.GetVertexAttribLdv.f)(index, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribPointerv(&self, index: types::GLuint, pname: types::GLenum, pointer: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const *mut __gl_imports::raw::c_void) -> ()>(self.GetVertexAttribPointerv.f)(index, pname, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribdv(&self, index: types::GLuint, pname: types::GLenum, params: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLdouble) -> ()>(self.GetVertexAttribdv.f)(index, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribfv(&self, index: types::GLuint, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLfloat) -> ()>(self.GetVertexAttribfv.f)(index, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribiv(&self, index: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetVertexAttribiv.f)(index, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnColorTable(&self, target: types::GLenum, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, table: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.GetnColorTable.f)(target, format, type_, bufSize, table) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnCompressedTexImage(&self, target: types::GLenum, lod: types::GLint, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.GetnCompressedTexImage.f)(target, lod, bufSize, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnConvolutionFilter(&self, target: types::GLenum, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, image: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.GetnConvolutionFilter.f)(target, format, type_, bufSize, image) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnHistogram(&self, target: types::GLenum, reset: types::GLboolean, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, values: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLboolean, types::GLenum, types::GLenum, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.GetnHistogram.f)(target, reset, format, type_, bufSize, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnMapdv(&self, target: types::GLenum, query: types::GLenum, bufSize: types::GLsizei, v: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLsizei, *mut types::GLdouble) -> ()>(self.GetnMapdv.f)(target, query, bufSize, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnMapfv(&self, target: types::GLenum, query: types::GLenum, bufSize: types::GLsizei, v: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLsizei, *mut types::GLfloat) -> ()>(self.GetnMapfv.f)(target, query, bufSize, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnMapiv(&self, target: types::GLenum, query: types::GLenum, bufSize: types::GLsizei, v: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLsizei, *mut types::GLint) -> ()>(self.GetnMapiv.f)(target, query, bufSize, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnMinmax(&self, target: types::GLenum, reset: types::GLboolean, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, values: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLboolean, types::GLenum, types::GLenum, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.GetnMinmax.f)(target, reset, format, type_, bufSize, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnPixelMapfv(&self, map: types::GLenum, bufSize: types::GLsizei, values: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *mut types::GLfloat) -> ()>(self.GetnPixelMapfv.f)(map, bufSize, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnPixelMapuiv(&self, map: types::GLenum, bufSize: types::GLsizei, values: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *mut types::GLuint) -> ()>(self.GetnPixelMapuiv.f)(map, bufSize, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnPixelMapusv(&self, map: types::GLenum, bufSize: types::GLsizei, values: *mut types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *mut types::GLushort) -> ()>(self.GetnPixelMapusv.f)(map, bufSize, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnPolygonStipple(&self, bufSize: types::GLsizei, pattern: *mut types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLubyte) -> ()>(self.GetnPolygonStipple.f)(bufSize, pattern) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnSeparableFilter(&self, target: types::GLenum, format: types::GLenum, type_: types::GLenum, rowBufSize: types::GLsizei, row: *mut __gl_imports::raw::c_void, columnBufSize: types::GLsizei, column: *mut __gl_imports::raw::c_void, span: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLsizei, *mut __gl_imports::raw::c_void, types::GLsizei, *mut __gl_imports::raw::c_void, *mut __gl_imports::raw::c_void) -> ()>(self.GetnSeparableFilter.f)(target, format, type_, rowBufSize, row, columnBufSize, column, span) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnTexImage(&self, target: types::GLenum, level: types::GLint, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLenum, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.GetnTexImage.f)(target, level, format, type_, bufSize, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnUniformdv(&self, program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut types::GLdouble) -> ()>(self.GetnUniformdv.f)(program, location, bufSize, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnUniformfv(&self, program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut types::GLfloat) -> ()>(self.GetnUniformfv.f)(program, location, bufSize, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnUniformiv(&self, program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut types::GLint) -> ()>(self.GetnUniformiv.f)(program, location, bufSize, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnUniformuiv(&self, program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut types::GLuint) -> ()>(self.GetnUniformuiv.f)(program, location, bufSize, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Hint(&self, target: types::GLenum, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(self.Hint.f)(target, mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn InvalidateBufferData(&self, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.InvalidateBufferData.f)(buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn InvalidateBufferSubData(&self, buffer: types::GLuint, offset: types::GLintptr, length: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(self.InvalidateBufferSubData.f)(buffer, offset, length) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn InvalidateFramebuffer(&self, target: types::GLenum, numAttachments: types::GLsizei, attachments: *const types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const types::GLenum) -> ()>(self.InvalidateFramebuffer.f)(target, numAttachments, attachments) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn InvalidateNamedFramebufferData(&self, framebuffer: types::GLuint, numAttachments: types::GLsizei, attachments: *const types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLenum) -> ()>(self.InvalidateNamedFramebufferData.f)(framebuffer, numAttachments, attachments) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn InvalidateNamedFramebufferSubData(&self, framebuffer: types::GLuint, numAttachments: types::GLsizei, attachments: *const types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.InvalidateNamedFramebufferSubData.f)(framebuffer, numAttachments, attachments, x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn InvalidateSubFramebuffer(&self, target: types::GLenum, numAttachments: types::GLsizei, attachments: *const types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.InvalidateSubFramebuffer.f)(target, numAttachments, attachments, x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn InvalidateTexImage(&self, texture: types::GLuint, level: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint) -> ()>(self.InvalidateTexImage.f)(texture, level) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn InvalidateTexSubImage(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei) -> ()>(self.InvalidateTexSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsBuffer(&self, buffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsBuffer.f)(buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsEnabled(&self, cap: types::GLenum) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLboolean>(self.IsEnabled.f)(cap) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsEnabledi(&self, target: types::GLenum, index: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> types::GLboolean>(self.IsEnabledi.f)(target, index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsFramebuffer(&self, framebuffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsFramebuffer.f)(framebuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsProgram(&self, program: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsProgramPipeline(&self, pipeline: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsProgramPipeline.f)(pipeline) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsQuery(&self, id: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsQuery.f)(id) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsRenderbuffer(&self, renderbuffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsRenderbuffer.f)(renderbuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsSampler(&self, sampler: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsSampler.f)(sampler) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsShader(&self, shader: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsShader.f)(shader) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsSync(&self, sync: types::GLsync) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync) -> types::GLboolean>(self.IsSync.f)(sync) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsTexture(&self, texture: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsTexture.f)(texture) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsTransformFeedback(&self, id: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsTransformFeedback.f)(id) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsVertexArray(&self, array: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsVertexArray.f)(array) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn LineWidth(&self, width: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.LineWidth.f)(width) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn LinkProgram(&self, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.LinkProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn LogicOp(&self, opcode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.LogicOp.f)(opcode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MapBuffer(&self, target: types::GLenum, access: types::GLenum) -> *mut __gl_imports::raw::c_void { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> *mut __gl_imports::raw::c_void>(self.MapBuffer.f)(target, access) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MapBufferRange(&self, target: types::GLenum, offset: types::GLintptr, length: types::GLsizeiptr, access: types::GLbitfield) -> *mut __gl_imports::raw::c_void { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLintptr, types::GLsizeiptr, types::GLbitfield) -> *mut __gl_imports::raw::c_void>(self.MapBufferRange.f)(target, offset, length, access) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MapNamedBuffer(&self, buffer: types::GLuint, access: types::GLenum) -> *mut __gl_imports::raw::c_void { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> *mut __gl_imports::raw::c_void>(self.MapNamedBuffer.f)(buffer, access) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MapNamedBufferRange(&self, buffer: types::GLuint, offset: types::GLintptr, length: types::GLsizeiptr, access: types::GLbitfield) -> *mut __gl_imports::raw::c_void { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLintptr, types::GLsizeiptr, types::GLbitfield) -> *mut __gl_imports::raw::c_void>(self.MapNamedBufferRange.f)(buffer, offset, length, access) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MemoryBarrier(&self, barriers: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(self.MemoryBarrier.f)(barriers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MemoryBarrierByRegion(&self, barriers: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(self.MemoryBarrierByRegion.f)(barriers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MinSampleShading(&self, value: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.MinSampleShading.f)(value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiDrawArrays(&self, mode: types::GLenum, first: *const types::GLint, count: *const types::GLsizei, drawcount: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLint, *const types::GLsizei, types::GLsizei) -> ()>(self.MultiDrawArrays.f)(mode, first, count, drawcount) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiDrawArraysIndirect(&self, mode: types::GLenum, indirect: *const __gl_imports::raw::c_void, drawcount: types::GLsizei, stride: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const __gl_imports::raw::c_void, types::GLsizei, types::GLsizei) -> ()>(self.MultiDrawArraysIndirect.f)(mode, indirect, drawcount, stride) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiDrawElements(&self, mode: types::GLenum, count: *const types::GLsizei, type_: types::GLenum, indices: *const *const __gl_imports::raw::c_void, drawcount: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLsizei, types::GLenum, *const *const __gl_imports::raw::c_void, types::GLsizei) -> ()>(self.MultiDrawElements.f)(mode, count, type_, indices, drawcount) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiDrawElementsBaseVertex(&self, mode: types::GLenum, count: *const types::GLsizei, type_: types::GLenum, indices: *const *const __gl_imports::raw::c_void, drawcount: types::GLsizei, basevertex: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLsizei, types::GLenum, *const *const __gl_imports::raw::c_void, types::GLsizei, *const types::GLint) -> ()>(self.MultiDrawElementsBaseVertex.f)(mode, count, type_, indices, drawcount, basevertex) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiDrawElementsIndirect(&self, mode: types::GLenum, type_: types::GLenum, indirect: *const __gl_imports::raw::c_void, drawcount: types::GLsizei, stride: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const __gl_imports::raw::c_void, types::GLsizei, types::GLsizei) -> ()>(self.MultiDrawElementsIndirect.f)(mode, type_, indirect, drawcount, stride) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiTexCoordP1ui(&self, texture: types::GLenum, type_: types::GLenum, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint) -> ()>(self.MultiTexCoordP1ui.f)(texture, type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiTexCoordP1uiv(&self, texture: types::GLenum, type_: types::GLenum, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLuint) -> ()>(self.MultiTexCoordP1uiv.f)(texture, type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiTexCoordP2ui(&self, texture: types::GLenum, type_: types::GLenum, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint) -> ()>(self.MultiTexCoordP2ui.f)(texture, type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiTexCoordP2uiv(&self, texture: types::GLenum, type_: types::GLenum, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLuint) -> ()>(self.MultiTexCoordP2uiv.f)(texture, type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiTexCoordP3ui(&self, texture: types::GLenum, type_: types::GLenum, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint) -> ()>(self.MultiTexCoordP3ui.f)(texture, type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiTexCoordP3uiv(&self, texture: types::GLenum, type_: types::GLenum, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLuint) -> ()>(self.MultiTexCoordP3uiv.f)(texture, type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiTexCoordP4ui(&self, texture: types::GLenum, type_: types::GLenum, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint) -> ()>(self.MultiTexCoordP4ui.f)(texture, type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiTexCoordP4uiv(&self, texture: types::GLenum, type_: types::GLenum, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLuint) -> ()>(self.MultiTexCoordP4uiv.f)(texture, type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedBufferData(&self, buffer: types::GLuint, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void, usage: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizeiptr, *const __gl_imports::raw::c_void, types::GLenum) -> ()>(self.NamedBufferData.f)(buffer, size, data, usage) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedBufferStorage(&self, buffer: types::GLuint, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void, flags: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizeiptr, *const __gl_imports::raw::c_void, types::GLbitfield) -> ()>(self.NamedBufferStorage.f)(buffer, size, data, flags) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedBufferSubData(&self, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLintptr, types::GLsizeiptr, *const __gl_imports::raw::c_void) -> ()>(self.NamedBufferSubData.f)(buffer, offset, size, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedFramebufferDrawBuffer(&self, framebuffer: types::GLuint, buf: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(self.NamedFramebufferDrawBuffer.f)(framebuffer, buf) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedFramebufferDrawBuffers(&self, framebuffer: types::GLuint, n: types::GLsizei, bufs: *const types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLenum) -> ()>(self.NamedFramebufferDrawBuffers.f)(framebuffer, n, bufs) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedFramebufferParameteri(&self, framebuffer: types::GLuint, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint) -> ()>(self.NamedFramebufferParameteri.f)(framebuffer, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedFramebufferReadBuffer(&self, framebuffer: types::GLuint, src: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(self.NamedFramebufferReadBuffer.f)(framebuffer, src) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedFramebufferRenderbuffer(&self, framebuffer: types::GLuint, attachment: types::GLenum, renderbuffertarget: types::GLenum, renderbuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum, types::GLuint) -> ()>(self.NamedFramebufferRenderbuffer.f)(framebuffer, attachment, renderbuffertarget, renderbuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedFramebufferTexture(&self, framebuffer: types::GLuint, attachment: types::GLenum, texture: types::GLuint, level: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLint) -> ()>(self.NamedFramebufferTexture.f)(framebuffer, attachment, texture, level) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedFramebufferTextureLayer(&self, framebuffer: types::GLuint, attachment: types::GLenum, texture: types::GLuint, level: types::GLint, layer: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLint, types::GLint) -> ()>(self.NamedFramebufferTextureLayer.f)(framebuffer, attachment, texture, level, layer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedRenderbufferStorage(&self, renderbuffer: types::GLuint, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(self.NamedRenderbufferStorage.f)(renderbuffer, internalformat, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedRenderbufferStorageMultisample(&self, renderbuffer: types::GLuint, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(self.NamedRenderbufferStorageMultisample.f)(renderbuffer, samples, internalformat, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NormalP3ui(&self, type_: types::GLenum, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.NormalP3ui.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NormalP3uiv(&self, type_: types::GLenum, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(self.NormalP3uiv.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ObjectLabel(&self, identifier: types::GLenum, name: types::GLuint, length: types::GLsizei, label: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *const types::GLchar) -> ()>(self.ObjectLabel.f)(identifier, name, length, label) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ObjectPtrLabel(&self, ptr: *const __gl_imports::raw::c_void, length: types::GLsizei, label: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const __gl_imports::raw::c_void, types::GLsizei, *const types::GLchar) -> ()>(self.ObjectPtrLabel.f)(ptr, length, label) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PatchParameterfv(&self, pname: types::GLenum, values: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(self.PatchParameterfv.f)(pname, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PatchParameteri(&self, pname: types::GLenum, value: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(self.PatchParameteri.f)(pname, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PauseTransformFeedback(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.PauseTransformFeedback.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PixelStoref(&self, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(self.PixelStoref.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PixelStorei(&self, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(self.PixelStorei.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PointParameterf(&self, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(self.PointParameterf.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PointParameterfv(&self, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(self.PointParameterfv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PointParameteri(&self, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(self.PointParameteri.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PointParameteriv(&self, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLint) -> ()>(self.PointParameteriv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PointSize(&self, size: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.PointSize.f)(size) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PolygonMode(&self, face: types::GLenum, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(self.PolygonMode.f)(face, mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PolygonOffset(&self, factor: types::GLfloat, units: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(self.PolygonOffset.f)(factor, units) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PopDebugGroup(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.PopDebugGroup.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PrimitiveRestartIndex(&self, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.PrimitiveRestartIndex.f)(index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramBinary(&self, program: types::GLuint, binaryFormat: types::GLenum, binary: *const __gl_imports::raw::c_void, length: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const __gl_imports::raw::c_void, types::GLsizei) -> ()>(self.ProgramBinary.f)(program, binaryFormat, binary, length) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramParameteri(&self, program: types::GLuint, pname: types::GLenum, value: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint) -> ()>(self.ProgramParameteri.f)(program, pname, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform1d(&self, program: types::GLuint, location: types::GLint, v0: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLdouble) -> ()>(self.ProgramUniform1d.f)(program, location, v0) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform1dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(self.ProgramUniform1dv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform1f(&self, program: types::GLuint, location: types::GLint, v0: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLfloat) -> ()>(self.ProgramUniform1f.f)(program, location, v0) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform1fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.ProgramUniform1fv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform1i(&self, program: types::GLuint, location: types::GLint, v0: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint) -> ()>(self.ProgramUniform1i.f)(program, location, v0) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform1iv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.ProgramUniform1iv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform1ui(&self, program: types::GLuint, location: types::GLint, v0: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLuint) -> ()>(self.ProgramUniform1ui.f)(program, location, v0) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform1uiv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLuint) -> ()>(self.ProgramUniform1uiv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform2d(&self, program: types::GLuint, location: types::GLint, v0: types::GLdouble, v1: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLdouble, types::GLdouble) -> ()>(self.ProgramUniform2d.f)(program, location, v0, v1) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform2dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(self.ProgramUniform2dv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform2f(&self, program: types::GLuint, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLfloat, types::GLfloat) -> ()>(self.ProgramUniform2f.f)(program, location, v0, v1) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform2fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.ProgramUniform2fv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform2i(&self, program: types::GLuint, location: types::GLint, v0: types::GLint, v1: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint) -> ()>(self.ProgramUniform2i.f)(program, location, v0, v1) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform2iv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.ProgramUniform2iv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform2ui(&self, program: types::GLuint, location: types::GLint, v0: types::GLuint, v1: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLuint, types::GLuint) -> ()>(self.ProgramUniform2ui.f)(program, location, v0, v1) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform2uiv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLuint) -> ()>(self.ProgramUniform2uiv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform3d(&self, program: types::GLuint, location: types::GLint, v0: types::GLdouble, v1: types::GLdouble, v2: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.ProgramUniform3d.f)(program, location, v0, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform3dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(self.ProgramUniform3dv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform3f(&self, program: types::GLuint, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ProgramUniform3f.f)(program, location, v0, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform3fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.ProgramUniform3fv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform3i(&self, program: types::GLuint, location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.ProgramUniform3i.f)(program, location, v0, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform3iv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.ProgramUniform3iv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform3ui(&self, program: types::GLuint, location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ProgramUniform3ui.f)(program, location, v0, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform3uiv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLuint) -> ()>(self.ProgramUniform3uiv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform4d(&self, program: types::GLuint, location: types::GLint, v0: types::GLdouble, v1: types::GLdouble, v2: types::GLdouble, v3: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.ProgramUniform4d.f)(program, location, v0, v1, v2, v3) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform4dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(self.ProgramUniform4dv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform4f(&self, program: types::GLuint, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat, v3: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ProgramUniform4f.f)(program, location, v0, v1, v2, v3) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform4fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.ProgramUniform4fv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform4i(&self, program: types::GLuint, location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint, v3: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.ProgramUniform4i.f)(program, location, v0, v1, v2, v3) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform4iv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.ProgramUniform4iv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform4ui(&self, program: types::GLuint, location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint, v3: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ProgramUniform4ui.f)(program, location, v0, v1, v2, v3) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform4uiv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLuint) -> ()>(self.ProgramUniform4uiv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix2dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.ProgramUniformMatrix2dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix2fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.ProgramUniformMatrix2fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix2x3dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.ProgramUniformMatrix2x3dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix2x3fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.ProgramUniformMatrix2x3fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix2x4dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.ProgramUniformMatrix2x4dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix2x4fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.ProgramUniformMatrix2x4fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix3dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.ProgramUniformMatrix3dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix3fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.ProgramUniformMatrix3fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix3x2dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.ProgramUniformMatrix3x2dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix3x2fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.ProgramUniformMatrix3x2fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix3x4dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.ProgramUniformMatrix3x4dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix3x4fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.ProgramUniformMatrix3x4fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix4dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.ProgramUniformMatrix4dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix4fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.ProgramUniformMatrix4fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix4x2dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.ProgramUniformMatrix4x2dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix4x2fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.ProgramUniformMatrix4x2fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix4x3dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.ProgramUniformMatrix4x3dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix4x3fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.ProgramUniformMatrix4x3fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProvokingVertex(&self, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.ProvokingVertex.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PushDebugGroup(&self, source: types::GLenum, id: types::GLuint, length: types::GLsizei, message: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *const types::GLchar) -> ()>(self.PushDebugGroup.f)(source, id, length, message) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn QueryCounter(&self, id: types::GLuint, target: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(self.QueryCounter.f)(id, target) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ReadBuffer(&self, src: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.ReadBuffer.f)(src) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ReadPixels(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *mut __gl_imports::raw::c_void) -> ()>(self.ReadPixels.f)(x, y, width, height, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ReadnPixels(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, data: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.ReadnPixels.f)(x, y, width, height, format, type_, bufSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ReleaseShaderCompiler(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.ReleaseShaderCompiler.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn RenderbufferStorage(&self, target: types::GLenum, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(self.RenderbufferStorage.f)(target, internalformat, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn RenderbufferStorageMultisample(&self, target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(self.RenderbufferStorageMultisample.f)(target, samples, internalformat, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ResumeTransformFeedback(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.ResumeTransformFeedback.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SampleCoverage(&self, value: types::GLfloat, invert: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLboolean) -> ()>(self.SampleCoverage.f)(value, invert) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SampleMaski(&self, maskNumber: types::GLuint, mask: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLbitfield) -> ()>(self.SampleMaski.f)(maskNumber, mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SamplerParameterIiv(&self, sampler: types::GLuint, pname: types::GLenum, param: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLint) -> ()>(self.SamplerParameterIiv.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SamplerParameterIuiv(&self, sampler: types::GLuint, pname: types::GLenum, param: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLuint) -> ()>(self.SamplerParameterIuiv.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SamplerParameterf(&self, sampler: types::GLuint, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLfloat) -> ()>(self.SamplerParameterf.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SamplerParameterfv(&self, sampler: types::GLuint, pname: types::GLenum, param: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLfloat) -> ()>(self.SamplerParameterfv.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SamplerParameteri(&self, sampler: types::GLuint, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint) -> ()>(self.SamplerParameteri.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SamplerParameteriv(&self, sampler: types::GLuint, pname: types::GLenum, param: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLint) -> ()>(self.SamplerParameteriv.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Scissor(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.Scissor.f)(x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ScissorArrayv(&self, first: types::GLuint, count: types::GLsizei, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLint) -> ()>(self.ScissorArrayv.f)(first, count, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ScissorIndexed(&self, index: types::GLuint, left: types::GLint, bottom: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.ScissorIndexed.f)(index, left, bottom, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ScissorIndexedv(&self, index: types::GLuint, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(self.ScissorIndexedv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SecondaryColorP3ui(&self, type_: types::GLenum, color: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.SecondaryColorP3ui.f)(type_, color) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SecondaryColorP3uiv(&self, type_: types::GLenum, color: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(self.SecondaryColorP3uiv.f)(type_, color) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ShaderBinary(&self, count: types::GLsizei, shaders: *const types::GLuint, binaryformat: types::GLenum, binary: *const __gl_imports::raw::c_void, length: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint, types::GLenum, *const __gl_imports::raw::c_void, types::GLsizei) -> ()>(self.ShaderBinary.f)(count, shaders, binaryformat, binary, length) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ShaderSource(&self, shader: types::GLuint, count: types::GLsizei, string: *const *const types::GLchar, length: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const *const types::GLchar, *const types::GLint) -> ()>(self.ShaderSource.f)(shader, count, string, length) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ShaderStorageBlockBinding(&self, program: types::GLuint, storageBlockIndex: types::GLuint, storageBlockBinding: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ShaderStorageBlockBinding.f)(program, storageBlockIndex, storageBlockBinding) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilFunc(&self, func: types::GLenum, ref_: types::GLint, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLuint) -> ()>(self.StencilFunc.f)(func, ref_, mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilFuncSeparate(&self, face: types::GLenum, func: types::GLenum, ref_: types::GLint, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint, types::GLuint) -> ()>(self.StencilFuncSeparate.f)(face, func, ref_, mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilMask(&self, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.StencilMask.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilMaskSeparate(&self, face: types::GLenum, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.StencilMaskSeparate.f)(face, mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilOp(&self, fail: types::GLenum, zfail: types::GLenum, zpass: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum) -> ()>(self.StencilOp.f)(fail, zfail, zpass) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilOpSeparate(&self, face: types::GLenum, sfail: types::GLenum, dpfail: types::GLenum, dppass: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLenum) -> ()>(self.StencilOpSeparate.f)(face, sfail, dpfail, dppass) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexBuffer(&self, target: types::GLenum, internalformat: types::GLenum, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint) -> ()>(self.TexBuffer.f)(target, internalformat, buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexBufferRange(&self, target: types::GLenum, internalformat: types::GLenum, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(self.TexBufferRange.f)(target, internalformat, buffer, offset, size) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexCoordP1ui(&self, type_: types::GLenum, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.TexCoordP1ui.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexCoordP1uiv(&self, type_: types::GLenum, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(self.TexCoordP1uiv.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexCoordP2ui(&self, type_: types::GLenum, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.TexCoordP2ui.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexCoordP2uiv(&self, type_: types::GLenum, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(self.TexCoordP2uiv.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexCoordP3ui(&self, type_: types::GLenum, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.TexCoordP3ui.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexCoordP3uiv(&self, type_: types::GLenum, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(self.TexCoordP3uiv.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexCoordP4ui(&self, type_: types::GLenum, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.TexCoordP4ui.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexCoordP4uiv(&self, type_: types::GLenum, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(self.TexCoordP4uiv.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexImage1D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLint, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.TexImage1D.f)(target, level, internalformat, width, border, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexImage2D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLint, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.TexImage2D.f)(target, level, internalformat, width, height, border, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexImage2DMultisample(&self, target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, fixedsamplelocations: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLboolean) -> ()>(self.TexImage2DMultisample.f)(target, samples, internalformat, width, height, fixedsamplelocations) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexImage3D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLint, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.TexImage3D.f)(target, level, internalformat, width, height, depth, border, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexImage3DMultisample(&self, target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, fixedsamplelocations: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLsizei, types::GLboolean) -> ()>(self.TexImage3DMultisample.f)(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexParameterIiv(&self, target: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(self.TexParameterIiv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexParameterIuiv(&self, target: types::GLenum, pname: types::GLenum, params: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLuint) -> ()>(self.TexParameterIuiv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexParameterf(&self, target: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(self.TexParameterf.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexParameterfv(&self, target: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(self.TexParameterfv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexParameteri(&self, target: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(self.TexParameteri.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexParameteriv(&self, target: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(self.TexParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexStorage1D(&self, target: types::GLenum, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei) -> ()>(self.TexStorage1D.f)(target, levels, internalformat, width) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexStorage2D(&self, target: types::GLenum, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(self.TexStorage2D.f)(target, levels, internalformat, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexStorage2DMultisample(&self, target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, fixedsamplelocations: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLboolean) -> ()>(self.TexStorage2DMultisample.f)(target, samples, internalformat, width, height, fixedsamplelocations) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexStorage3D(&self, target: types::GLenum, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLsizei) -> ()>(self.TexStorage3D.f)(target, levels, internalformat, width, height, depth) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexStorage3DMultisample(&self, target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, fixedsamplelocations: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLsizei, types::GLboolean) -> ()>(self.TexStorage3DMultisample.f)(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexSubImage1D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.TexSubImage1D.f)(target, level, xoffset, width, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexSubImage2D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.TexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexSubImage3D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.TexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureBarrier(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.TextureBarrier.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureBuffer(&self, texture: types::GLuint, internalformat: types::GLenum, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint) -> ()>(self.TextureBuffer.f)(texture, internalformat, buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureBufferRange(&self, texture: types::GLuint, internalformat: types::GLenum, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(self.TextureBufferRange.f)(texture, internalformat, buffer, offset, size) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureParameterIiv(&self, texture: types::GLuint, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLint) -> ()>(self.TextureParameterIiv.f)(texture, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureParameterIuiv(&self, texture: types::GLuint, pname: types::GLenum, params: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLuint) -> ()>(self.TextureParameterIuiv.f)(texture, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureParameterf(&self, texture: types::GLuint, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLfloat) -> ()>(self.TextureParameterf.f)(texture, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureParameterfv(&self, texture: types::GLuint, pname: types::GLenum, param: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLfloat) -> ()>(self.TextureParameterfv.f)(texture, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureParameteri(&self, texture: types::GLuint, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint) -> ()>(self.TextureParameteri.f)(texture, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureParameteriv(&self, texture: types::GLuint, pname: types::GLenum, param: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLint) -> ()>(self.TextureParameteriv.f)(texture, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureStorage1D(&self, texture: types::GLuint, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, types::GLenum, types::GLsizei) -> ()>(self.TextureStorage1D.f)(texture, levels, internalformat, width) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureStorage2D(&self, texture: types::GLuint, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(self.TextureStorage2D.f)(texture, levels, internalformat, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureStorage2DMultisample(&self, texture: types::GLuint, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, fixedsamplelocations: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLboolean) -> ()>(self.TextureStorage2DMultisample.f)(texture, samples, internalformat, width, height, fixedsamplelocations) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureStorage3D(&self, texture: types::GLuint, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLsizei) -> ()>(self.TextureStorage3D.f)(texture, levels, internalformat, width, height, depth) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureStorage3DMultisample(&self, texture: types::GLuint, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, fixedsamplelocations: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLsizei, types::GLboolean) -> ()>(self.TextureStorage3DMultisample.f)(texture, samples, internalformat, width, height, depth, fixedsamplelocations) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureSubImage1D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.TextureSubImage1D.f)(texture, level, xoffset, width, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureSubImage2D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.TextureSubImage2D.f)(texture, level, xoffset, yoffset, width, height, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureSubImage3D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.TextureSubImage3D.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureView(&self, texture: types::GLuint, target: types::GLenum, origtexture: types::GLuint, internalformat: types::GLenum, minlevel: types::GLuint, numlevels: types::GLuint, minlayer: types::GLuint, numlayers: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLenum, types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(self.TextureView.f)(texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TransformFeedbackBufferBase(&self, xfb: types::GLuint, index: types::GLuint, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(self.TransformFeedbackBufferBase.f)(xfb, index, buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TransformFeedbackBufferRange(&self, xfb: types::GLuint, index: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(self.TransformFeedbackBufferRange.f)(xfb, index, buffer, offset, size) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TransformFeedbackVaryings(&self, program: types::GLuint, count: types::GLsizei, varyings: *const *const types::GLchar, bufferMode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const *const types::GLchar, types::GLenum) -> ()>(self.TransformFeedbackVaryings.f)(program, count, varyings, bufferMode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1d(&self, location: types::GLint, x: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLdouble) -> ()>(self.Uniform1d.f)(location, x) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1dv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(self.Uniform1dv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1f(&self, location: types::GLint, v0: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat) -> ()>(self.Uniform1f.f)(location, v0) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1fv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.Uniform1fv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1i(&self, location: types::GLint, v0: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint) -> ()>(self.Uniform1i.f)(location, v0) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1iv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.Uniform1iv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1ui(&self, location: types::GLint, v0: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint) -> ()>(self.Uniform1ui.f)(location, v0) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1uiv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> ()>(self.Uniform1uiv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2d(&self, location: types::GLint, x: types::GLdouble, y: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLdouble, types::GLdouble) -> ()>(self.Uniform2d.f)(location, x, y) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2dv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(self.Uniform2dv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2f(&self, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat) -> ()>(self.Uniform2f.f)(location, v0, v1) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2fv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.Uniform2fv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2i(&self, location: types::GLint, v0: types::GLint, v1: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint) -> ()>(self.Uniform2i.f)(location, v0, v1) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2iv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.Uniform2iv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2ui(&self, location: types::GLint, v0: types::GLuint, v1: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint, types::GLuint) -> ()>(self.Uniform2ui.f)(location, v0, v1) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2uiv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> ()>(self.Uniform2uiv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3d(&self, location: types::GLint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.Uniform3d.f)(location, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3dv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(self.Uniform3dv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3f(&self, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Uniform3f.f)(location, v0, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3fv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.Uniform3fv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3i(&self, location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.Uniform3i.f)(location, v0, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3iv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.Uniform3iv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3ui(&self, location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint, types::GLuint, types::GLuint) -> ()>(self.Uniform3ui.f)(location, v0, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3uiv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> ()>(self.Uniform3uiv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4d(&self, location: types::GLint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.Uniform4d.f)(location, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4dv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(self.Uniform4dv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4f(&self, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat, v3: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Uniform4f.f)(location, v0, v1, v2, v3) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4fv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.Uniform4fv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4i(&self, location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint, v3: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.Uniform4i.f)(location, v0, v1, v2, v3) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4iv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.Uniform4iv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4ui(&self, location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint, v3: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(self.Uniform4ui.f)(location, v0, v1, v2, v3) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4uiv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> ()>(self.Uniform4uiv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformBlockBinding(&self, program: types::GLuint, uniformBlockIndex: types::GLuint, uniformBlockBinding: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(self.UniformBlockBinding.f)(program, uniformBlockIndex, uniformBlockBinding) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix2dv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.UniformMatrix2dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix2fv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.UniformMatrix2fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix2x3dv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.UniformMatrix2x3dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix2x3fv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.UniformMatrix2x3fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix2x4dv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.UniformMatrix2x4dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix2x4fv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.UniformMatrix2x4fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix3dv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.UniformMatrix3dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix3fv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.UniformMatrix3fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix3x2dv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.UniformMatrix3x2dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix3x2fv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.UniformMatrix3x2fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix3x4dv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.UniformMatrix3x4dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix3x4fv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.UniformMatrix3x4fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix4dv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.UniformMatrix4dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix4fv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.UniformMatrix4fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix4x2dv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.UniformMatrix4x2dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix4x2fv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.UniformMatrix4x2fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix4x3dv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(self.UniformMatrix4x3dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix4x3fv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.UniformMatrix4x3fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformSubroutinesuiv(&self, shadertype: types::GLenum, count: types::GLsizei, indices: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const types::GLuint) -> ()>(self.UniformSubroutinesuiv.f)(shadertype, count, indices) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UnmapBuffer(&self, target: types::GLenum) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLboolean>(self.UnmapBuffer.f)(target) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UnmapNamedBuffer(&self, buffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.UnmapNamedBuffer.f)(buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UseProgram(&self, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.UseProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UseProgramStages(&self, pipeline: types::GLuint, stages: types::GLbitfield, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLbitfield, types::GLuint) -> ()>(self.UseProgramStages.f)(pipeline, stages, program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ValidateProgram(&self, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ValidateProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ValidateProgramPipeline(&self, pipeline: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ValidateProgramPipeline.f)(pipeline) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexArrayAttribBinding(&self, vaobj: types::GLuint, attribindex: types::GLuint, bindingindex: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(self.VertexArrayAttribBinding.f)(vaobj, attribindex, bindingindex) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexArrayAttribFormat(&self, vaobj: types::GLuint, attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, normalized: types::GLboolean, relativeoffset: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLint, types::GLenum, types::GLboolean, types::GLuint) -> ()>(self.VertexArrayAttribFormat.f)(vaobj, attribindex, size, type_, normalized, relativeoffset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexArrayAttribIFormat(&self, vaobj: types::GLuint, attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, relativeoffset: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLint, types::GLenum, types::GLuint) -> ()>(self.VertexArrayAttribIFormat.f)(vaobj, attribindex, size, type_, relativeoffset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexArrayAttribLFormat(&self, vaobj: types::GLuint, attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, relativeoffset: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLint, types::GLenum, types::GLuint) -> ()>(self.VertexArrayAttribLFormat.f)(vaobj, attribindex, size, type_, relativeoffset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexArrayBindingDivisor(&self, vaobj: types::GLuint, bindingindex: types::GLuint, divisor: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(self.VertexArrayBindingDivisor.f)(vaobj, bindingindex, divisor) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexArrayElementBuffer(&self, vaobj: types::GLuint, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.VertexArrayElementBuffer.f)(vaobj, buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexArrayVertexBuffer(&self, vaobj: types::GLuint, bindingindex: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, stride: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, types::GLintptr, types::GLsizei) -> ()>(self.VertexArrayVertexBuffer.f)(vaobj, bindingindex, buffer, offset, stride) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexArrayVertexBuffers(&self, vaobj: types::GLuint, first: types::GLuint, count: types::GLsizei, buffers: *const types::GLuint, offsets: *const types::GLintptr, strides: *const types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *const types::GLuint, *const types::GLintptr, *const types::GLsizei) -> ()>(self.VertexArrayVertexBuffers.f)(vaobj, first, count, buffers, offsets, strides) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib1d(&self, index: types::GLuint, x: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble) -> ()>(self.VertexAttrib1d.f)(index, x) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib1dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(self.VertexAttrib1dv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib1f(&self, index: types::GLuint, x: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat) -> ()>(self.VertexAttrib1f.f)(index, x) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib1fv(&self, index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(self.VertexAttrib1fv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib1s(&self, index: types::GLuint, x: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLshort) -> ()>(self.VertexAttrib1s.f)(index, x) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib1sv(&self, index: types::GLuint, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(self.VertexAttrib1sv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib2d(&self, index: types::GLuint, x: types::GLdouble, y: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble) -> ()>(self.VertexAttrib2d.f)(index, x, y) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib2dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(self.VertexAttrib2dv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib2f(&self, index: types::GLuint, x: types::GLfloat, y: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat) -> ()>(self.VertexAttrib2f.f)(index, x, y) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib2fv(&self, index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(self.VertexAttrib2fv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib2s(&self, index: types::GLuint, x: types::GLshort, y: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLshort, types::GLshort) -> ()>(self.VertexAttrib2s.f)(index, x, y) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib2sv(&self, index: types::GLuint, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(self.VertexAttrib2sv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib3d(&self, index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.VertexAttrib3d.f)(index, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib3dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(self.VertexAttrib3dv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib3f(&self, index: types::GLuint, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.VertexAttrib3f.f)(index, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib3fv(&self, index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(self.VertexAttrib3fv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib3s(&self, index: types::GLuint, x: types::GLshort, y: types::GLshort, z: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLshort, types::GLshort, types::GLshort) -> ()>(self.VertexAttrib3s.f)(index, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib3sv(&self, index: types::GLuint, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(self.VertexAttrib3sv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4Nbv(&self, index: types::GLuint, v: *const types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLbyte) -> ()>(self.VertexAttrib4Nbv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4Niv(&self, index: types::GLuint, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(self.VertexAttrib4Niv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4Nsv(&self, index: types::GLuint, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(self.VertexAttrib4Nsv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4Nub(&self, index: types::GLuint, x: types::GLubyte, y: types::GLubyte, z: types::GLubyte, w: types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLubyte, types::GLubyte, types::GLubyte, types::GLubyte) -> ()>(self.VertexAttrib4Nub.f)(index, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4Nubv(&self, index: types::GLuint, v: *const types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLubyte) -> ()>(self.VertexAttrib4Nubv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4Nuiv(&self, index: types::GLuint, v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(self.VertexAttrib4Nuiv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4Nusv(&self, index: types::GLuint, v: *const types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLushort) -> ()>(self.VertexAttrib4Nusv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4bv(&self, index: types::GLuint, v: *const types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLbyte) -> ()>(self.VertexAttrib4bv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4d(&self, index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.VertexAttrib4d.f)(index, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(self.VertexAttrib4dv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4f(&self, index: types::GLuint, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat, w: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.VertexAttrib4f.f)(index, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4fv(&self, index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(self.VertexAttrib4fv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4iv(&self, index: types::GLuint, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(self.VertexAttrib4iv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4s(&self, index: types::GLuint, x: types::GLshort, y: types::GLshort, z: types::GLshort, w: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLshort, types::GLshort, types::GLshort, types::GLshort) -> ()>(self.VertexAttrib4s.f)(index, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4sv(&self, index: types::GLuint, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(self.VertexAttrib4sv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4ubv(&self, index: types::GLuint, v: *const types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLubyte) -> ()>(self.VertexAttrib4ubv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4uiv(&self, index: types::GLuint, v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(self.VertexAttrib4uiv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4usv(&self, index: types::GLuint, v: *const types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLushort) -> ()>(self.VertexAttrib4usv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribBinding(&self, attribindex: types::GLuint, bindingindex: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.VertexAttribBinding.f)(attribindex, bindingindex) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribDivisor(&self, index: types::GLuint, divisor: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.VertexAttribDivisor.f)(index, divisor) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribFormat(&self, attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, normalized: types::GLboolean, relativeoffset: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLboolean, types::GLuint) -> ()>(self.VertexAttribFormat.f)(attribindex, size, type_, normalized, relativeoffset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI1i(&self, index: types::GLuint, x: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint) -> ()>(self.VertexAttribI1i.f)(index, x) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI1iv(&self, index: types::GLuint, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(self.VertexAttribI1iv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI1ui(&self, index: types::GLuint, x: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.VertexAttribI1ui.f)(index, x) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI1uiv(&self, index: types::GLuint, v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(self.VertexAttribI1uiv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI2i(&self, index: types::GLuint, x: types::GLint, y: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint) -> ()>(self.VertexAttribI2i.f)(index, x, y) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI2iv(&self, index: types::GLuint, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(self.VertexAttribI2iv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI2ui(&self, index: types::GLuint, x: types::GLuint, y: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(self.VertexAttribI2ui.f)(index, x, y) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI2uiv(&self, index: types::GLuint, v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(self.VertexAttribI2uiv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI3i(&self, index: types::GLuint, x: types::GLint, y: types::GLint, z: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint) -> ()>(self.VertexAttribI3i.f)(index, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI3iv(&self, index: types::GLuint, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(self.VertexAttribI3iv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI3ui(&self, index: types::GLuint, x: types::GLuint, y: types::GLuint, z: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(self.VertexAttribI3ui.f)(index, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI3uiv(&self, index: types::GLuint, v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(self.VertexAttribI3uiv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI4bv(&self, index: types::GLuint, v: *const types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLbyte) -> ()>(self.VertexAttribI4bv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI4i(&self, index: types::GLuint, x: types::GLint, y: types::GLint, z: types::GLint, w: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.VertexAttribI4i.f)(index, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI4iv(&self, index: types::GLuint, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(self.VertexAttribI4iv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI4sv(&self, index: types::GLuint, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(self.VertexAttribI4sv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI4ubv(&self, index: types::GLuint, v: *const types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLubyte) -> ()>(self.VertexAttribI4ubv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI4ui(&self, index: types::GLuint, x: types::GLuint, y: types::GLuint, z: types::GLuint, w: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(self.VertexAttribI4ui.f)(index, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI4uiv(&self, index: types::GLuint, v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(self.VertexAttribI4uiv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI4usv(&self, index: types::GLuint, v: *const types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLushort) -> ()>(self.VertexAttribI4usv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribIFormat(&self, attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, relativeoffset: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLuint) -> ()>(self.VertexAttribIFormat.f)(attribindex, size, type_, relativeoffset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribIPointer(&self, index: types::GLuint, size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.VertexAttribIPointer.f)(index, size, type_, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribL1d(&self, index: types::GLuint, x: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble) -> ()>(self.VertexAttribL1d.f)(index, x) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribL1dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(self.VertexAttribL1dv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribL2d(&self, index: types::GLuint, x: types::GLdouble, y: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble) -> ()>(self.VertexAttribL2d.f)(index, x, y) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribL2dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(self.VertexAttribL2dv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribL3d(&self, index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.VertexAttribL3d.f)(index, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribL3dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(self.VertexAttribL3dv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribL4d(&self, index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.VertexAttribL4d.f)(index, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribL4dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(self.VertexAttribL4dv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribLFormat(&self, attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, relativeoffset: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLuint) -> ()>(self.VertexAttribLFormat.f)(attribindex, size, type_, relativeoffset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribLPointer(&self, index: types::GLuint, size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.VertexAttribLPointer.f)(index, size, type_, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribP1ui(&self, index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, types::GLuint) -> ()>(self.VertexAttribP1ui.f)(index, type_, normalized, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribP1uiv(&self, index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, *const types::GLuint) -> ()>(self.VertexAttribP1uiv.f)(index, type_, normalized, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribP2ui(&self, index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, types::GLuint) -> ()>(self.VertexAttribP2ui.f)(index, type_, normalized, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribP2uiv(&self, index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, *const types::GLuint) -> ()>(self.VertexAttribP2uiv.f)(index, type_, normalized, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribP3ui(&self, index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, types::GLuint) -> ()>(self.VertexAttribP3ui.f)(index, type_, normalized, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribP3uiv(&self, index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, *const types::GLuint) -> ()>(self.VertexAttribP3uiv.f)(index, type_, normalized, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribP4ui(&self, index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, types::GLuint) -> ()>(self.VertexAttribP4ui.f)(index, type_, normalized, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribP4uiv(&self, index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, *const types::GLuint) -> ()>(self.VertexAttribP4uiv.f)(index, type_, normalized, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribPointer(&self, index: types::GLuint, size: types::GLint, type_: types::GLenum, normalized: types::GLboolean, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLboolean, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.VertexAttribPointer.f)(index, size, type_, normalized, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexBindingDivisor(&self, bindingindex: types::GLuint, divisor: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.VertexBindingDivisor.f)(bindingindex, divisor) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexP2ui(&self, type_: types::GLenum, value: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.VertexP2ui.f)(type_, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexP2uiv(&self, type_: types::GLenum, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(self.VertexP2uiv.f)(type_, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexP3ui(&self, type_: types::GLenum, value: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.VertexP3ui.f)(type_, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexP3uiv(&self, type_: types::GLenum, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(self.VertexP3uiv.f)(type_, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexP4ui(&self, type_: types::GLenum, value: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.VertexP4ui.f)(type_, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexP4uiv(&self, type_: types::GLenum, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(self.VertexP4uiv.f)(type_, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Viewport(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.Viewport.f)(x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ViewportArrayv(&self, first: types::GLuint, count: types::GLsizei, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLfloat) -> ()>(self.ViewportArrayv.f)(first, count, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ViewportIndexedf(&self, index: types::GLuint, x: types::GLfloat, y: types::GLfloat, w: types::GLfloat, h: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ViewportIndexedf.f)(index, x, y, w, h) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ViewportIndexedfv(&self, index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(self.ViewportIndexedfv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn WaitSync(&self, sync: types::GLsync, flags: types::GLbitfield, timeout: types::GLuint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync, types::GLbitfield, types::GLuint64) -> ()>(self.WaitSync.f)(sync, flags, timeout) }
}

        unsafe impl __gl_imports::Send for Gl {}
