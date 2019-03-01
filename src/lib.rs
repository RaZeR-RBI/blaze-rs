#![deny(missing_docs)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! This crate wraps the [blaze library](https://github.com/razer-rbi/blaze)
//! which is geared towards efficient and cross-platform 2D sprite drawing using OpenGL.
//! Supported features:
//! * Dynamic batched sprite drawing
//! * Static batched sprite drawing
//! * Immediate drawing
//! * Texture loading
//! * Render targets
//! * Custom shaders
//! * Screenshot saving

#[macro_use]
extern crate bitflags;
extern crate bytes;
#[macro_use]
extern crate enum_primitive;

/// Defines blending-related functionality.
pub mod blend;
/// Dynamic batched drawing. Designed for moving sprites.
pub mod dynamic;
/// Immediate-mode drawing.
pub mod immediate;
/// Render target support.
pub mod rendertarget;
/// Custom shader support.
pub mod shader;
/// Static batched drawing. Designed for static geometry.
pub mod r#static;
/// Texture loading and saving.
pub mod texture;

#[cfg_attr(tarpaulin, skip)]
mod internal;

use crate::internal::wrap_result;

/// A rectangle which has it's top-left corner position, width and height expressed in floats.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    /// X position of top left corner
    pub x: f32,
    /// Y position of top left corner
    pub y: f32,
    /// Width
    pub w: f32,
    /// Height
    pub h: f32,
}

/// Underlying sprite quad data structure used by VAOs (vertex array objects).
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SpriteQuad {
    /// Four vertices defining the quad, defined in the following order:
    /// 0 - top-left, 1 - bottom-left, 2 - top-right, 3 - bottom-right
    /// (think of N flipped horizontally, like И)
    pub vertices: [Vertex; 4usize],
}

/// Two-dimensional float vector.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
}
/// Four-dimensional float vector.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
    /// Z coordinate
    pub z: f32,
    /// W coordinate
    pub w: f32,
}

/// Underlying vertex data structure used by VAOs (vertex array objects).
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    /// X position on the screen
    pub x: f32,
    /// Y position on the screen
    pub y: f32,
    /// Texture coordinate (U)
    pub u: f32,
    /// Texture coordinate (V)
    pub v: f32,
    /// Color (R)
    pub r: f32,
    /// Color (G)
    pub g: f32,
    /// Color (B)
    pub b: f32,
    /// Color (alpha)
    pub a: f32,
}

use internal::*;
use std::ffi::*;
use std::os::raw::*;
use std::string::*;

/// Represents a RGBA color.
#[derive(Debug, Copy, Clone)]
pub struct Color {
    /// Red component, defined in range from 0.0 to 1.0
    pub r: f32,
    /// Green component, defined in range from 0.0 to 1.0
    pub g: f32,
    /// Blue component, defined in range from 0.0 to 1.0
    pub b: f32,
    /// Alpha component, defined in range from 0.0 to 1.0
    pub a: f32,
}

#[cfg_attr(tarpaulin, skip)]
impl From<Vector4> for Color {
    fn from(vector: Vector4) -> Self {
        Color { r: vector.x, g: vector.y, b: vector.z, a: vector.w }
    }
}

#[cfg_attr(tarpaulin, skip)]
impl From<Color> for Vector4 {
    fn from(color: Color) -> Self {
        Vector4 { x: color.r, y: color.g, z: color.b, w: color.a }
    }
}

enum_from_primitive! {
    /// Defines supported flip modes that can be used when the sprite is drawn.
    #[cfg_attr(tarpaulin, skip)]
    #[derive(Debug, Copy, Clone)]
    pub enum SpriteFlip
    {
        /// Draws the sprite texture as it is
        None = BLZ_SpriteFlip_NONE as isize,
        /// Flips the texture horizontally
        FlipH = BLZ_SpriteFlip_FLIP_H as isize,
        /// Flips the texture vertically
        FlipV = BLZ_SpriteFlip_FLIP_V as isize,
        /// Flips the texture both horizontally and vertically
        Both = BLZ_SpriteFlip_BOTH as isize
    }
}

/// Defines a type for OpenGL procedure loader.
pub type GLProcLoader = unsafe extern "C" fn(name: *const c_char) -> *mut c_void;
/// Alias for `Result<(), String>`.
pub type CallResult = Result<(), String>;

/// Returns last API error information string. The same string is returned from
/// API calls which return `Result<..., String>`.
/// Note: error string may be empty even if an API call failed, the output
/// `Result<..., String>` will return an `Err("Unknown error")` in that case.
pub fn get_last_error() -> Option<String> {
    unsafe {
        let ptr = BLZ_GetLastError().as_ref();
        ptr.map(|val| CStr::from_ptr(val).to_str().unwrap().to_owned())
    }
}

/// Loads OpenGL functions used by this library. Requires an active window with
/// an OpenGL context (version 3.0 core and above).
/// # Example (using SDL2)
/// ```
///    use blaze_rs::load;
///    use sdl2::video::GLProfile;
///
///    let context = sdl2::init().unwrap();
///    let video_sys = context.video().unwrap();
///    let gl_attr = video_sys.gl_attr();
///    gl_attr.set_context_profile(GLProfile::Core);
///    gl_attr.set_context_version(3, 0);
///    let window = video_sys.window("Test", 800, 600)
///        .opengl()
///        .build()
///        .unwrap();
///    let _ctx = window.gl_create_context().unwrap();
///
///    match load(sdl2::sys::SDL_GL_GetProcAddress) {
///        Ok(_) => {}
///        Err(e) => panic!(e),
///    }
/// ```
pub fn load(loader: GLProcLoader) -> CallResult {
    unsafe {
        match BLZ_Load(Some(loader)) {
            x if x > 0 => Ok(()),
            _ => panic!(get_last_error().unwrap_or("Unknown error".to_string())),
        }
    }
}

/// Sets viewport dimensions. All sprite position and size calculations will be
/// based on them. In most cases, you should pass the window size in pixels here,
/// or a scaled value for pixel-art based games, for example.
pub fn set_viewport(width: u32, height: u32) -> CallResult {
    unsafe { wrap_result(BLZ_SetViewport(width as i32, height as i32)) }
}

/// Sets the color which is used for clearing the framebuffer.
pub fn set_clear_color(color: Color) {
    unsafe {
        BLZ_SetClearColor(color.into());
    }
}

/// Clears the current framebuffer.
pub fn clear() {
    unsafe {
        BLZ_Clear();
    }
}
