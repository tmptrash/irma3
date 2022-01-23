//!
//! Implementation of GUI plugin. Gives an ability to visualize the atoms, 
//! molecules and all the stuff inside the world.
//! TODO: Zoom by mouse scroll button
//! TODO: Move into 4 directions
//!
extern crate piston_window;
extern crate image as im;
extern crate vecmath;

use piston_window::TextureContext;
use piston_window::PistonWindow;
use piston_window::G2dTexture;
use piston_window::WindowSettings;
use piston_window::TextureSettings;
use piston_window::Texture;
use piston_window::OpenGL;
use gfx_device_gl::*;
use im::Pixel;
use vecmath::*;
use share::io::IO;
use share::cfg::Config;
///
/// Internal GUI plugin data
///
struct Gui {
    width: u32,                            // world width
    height: u32,                           // world height
    win: PistonWindow,                     // window object
    canvas: im::RgbaImage,                 // image buffer to draw the pixels
    texture_ctx: TextureContext<           // GL texture context
        gfx_device_gl::Factory,
        gfx_device_gl::Resources,
        gfx_device_gl::CommandBuffer>,
    texture: G2dTexture,                   // GL texture to draw on
    zoom: f32                              // zoom coefficient
}
///
/// Plugin's internal Gui data
///
static mut GUI: Option<Gui> = None;
///
/// Plugin API. initializes plugin
///
#[no_mangle] fn init(_io: &IO, cfg: &mut Config) {
    let width = cfg.WIDTH() as u32;
    let height = cfg.HEIGHT() as u32;
    let canvas = im::ImageBuffer::new(width, height);
    let mut win: PistonWindow = WindowSettings::new("irma4", (width, height))
        .exit_on_esc(true)
        .graphics_api(OpenGL::V3_2)
        .fullscreen(true)
        .build()
        .unwrap();
    let mut texture_ctx = TextureContext {
        factory: win.factory.clone(),
        encoder: win.factory.create_command_buffer().into()
    };
    let mut texture = Texture::from_image(&mut texture_ctx, &canvas, &TextureSettings::new()).unwrap();

    unsafe {
        GUI = Some(Gui {
            width,
            height,
            win,
            canvas,
            texture_ctx,
            texture,
            zoom: 1.0
        });
    }
}
///
/// Plugin API. Do main work by haddling GUI and user events, drawing atoms
///
#[no_mangle] pub fn idle(_io: &IO) {
    let gui_ref = unsafe { &mut GUI }.as_mut().unwrap();
    let win_ref = &mut gui_ref.win;
    let event = win_ref.next();
    if !event.is_some() { return }
}
///
/// Plugin API. Destroys plugin.
///
#[no_mangle] pub fn remove(_io: &IO) {
    // TODO: remove the window!!!
}