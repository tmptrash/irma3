//!
//! Module of definitions for GUI plugin
//! 
use piston_window::{PistonWindow, TextureContext, G2dTexture};
use image as im;
///
/// Black color for no atom or empty cell
///
pub const BLACK_COLOR_U8: im::Rgba<u8> = im::Rgba([0, 0, 0, 0]);
///
/// Map of atoms types to their colors
///
pub const ATOM_COLORS: [im::Rgba<u8>; 8] = [
    BLACK_COLOR_U8,                            // no atom | empty cell | black pixel
    im::Rgba([250, 0,   0,   255]),            // mov atom
    im::Rgba([0,   250, 0,   255]),            // fix atom
    im::Rgba([0,   0,   250, 255]),            // spl atom
    im::Rgba([0,   150, 250, 255]),            // if atom
    im::Rgba([150, 0,   150, 255]),            // job atom
    BLACK_COLOR_U8,                            // no atom | empty cell | black pixel
    BLACK_COLOR_U8                             // no atom | empty cell | black pixel
];
///
/// Color for clear screen operation
///
pub const BLACK_COLOR_F32: [f32; 4] = [0.0; 4];
///
/// Internal GUI plugin data
///
pub struct Gui {
    pub width: u32,                            // world width
    pub height: u32,                           // world height
    pub win: PistonWindow,                     // window object
    pub canvas: im::RgbaImage,                 // image buffer to draw the pixels
    pub texture_ctx: TextureContext<           // GL texture context
        gfx_device_gl::Factory,
        gfx_device_gl::Resources,
        gfx_device_gl::CommandBuffer>,
    pub texture: G2dTexture,                   // GL texture to draw on
    pub zoom: f64                              // zoom coefficient
}