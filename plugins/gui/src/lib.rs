//!
//! Implementation of GUI plugin. Gives an ability to visualize atoms, 
//! molecules and all the stuff inside the world.
//! TODO: Zoom by mouse scroll button
//! TODO: Move into 4 directions
//!
pub mod global;

extern crate piston_window;
extern crate image as im;
extern crate vecmath;

use global::ATOM_COLORS;
use piston_window::TextureContext;
use piston_window::PistonWindow;
use piston_window::WindowSettings;
use piston_window::TextureSettings;
use piston_window::Texture;
use piston_window::OpenGL;
use piston_window::RenderEvent;
use piston_window::Window;
use piston_window::Transformed;
use piston_window::image;
use piston_window::clear;
use share::io::IO;
use share::io::events::EVENT_SET_DOT;
use share::io::Param;
use share::atom::get_type;
use global::Gui;
///
/// Plugin's internal Gui data
///
static mut GUI: Option<Gui> = None;
///
/// Plugin API. initializes plugin. Creates piston windows, canvas, context an
/// all needed stuff for drawing in 2D
///
#[no_mangle] fn init(io: &mut IO) {
    create_gui(io);
    add_listeners(io);
}
///
/// Plugin API. Do main work by haddling GUI and user events, drawing atoms
///
#[no_mangle] pub fn idle(_io: &IO) {
    let gui_ref = unsafe { &mut GUI }.as_mut().unwrap();
    let win = &mut gui_ref.win;
    if let Some(e) = win.next() {
        if e.render_args().is_some() {
            gui_ref.texture.update(&mut gui_ref.texture_ctx, &gui_ref.canvas).unwrap();
            win.draw_2d(&e, |c, g, device| {
                clear([1.0; 4], g);
                c.zoom(gui_ref.zoom);
                // Update texture before rendering.
                gui_ref.texture_ctx.encoder.flush(device);
                image(&gui_ref.texture, c.transform, g);
            });
        }
    }
}
///
/// Plugin API. Destroys plugin.
///
#[no_mangle] pub fn remove(_io: &IO) {
    let gui_ref = unsafe { &mut GUI }.as_mut().unwrap();
    let win = &mut gui_ref.win;
    win.set_should_close(true);
}

fn create_gui(io: &mut IO) {
    let width = io.cfg.WIDTH() as u32;
    let height = io.cfg.HEIGHT() as u32;
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
    let texture = Texture::from_image(&mut texture_ctx, &canvas, &TextureSettings::new()).unwrap();

    unsafe {
        GUI = Some(Gui {
            width,
            height,
            win,
            canvas,
            texture_ctx,
            texture,
            zoom: 1.0
        })
    }
}
///
/// Adds core listeners to react on
///
fn add_listeners(io: &mut IO) {
    //
    // If a dot was added into the world we have to draw it on a canvas
    //
    io.on(EVENT_SET_DOT, |params| {
        let gui_ref = unsafe { &mut GUI }.as_mut().unwrap();
        if let Param::SetDot(offs, atom) = params.param {
            // TODO: x, y should be calculated according to the size and
            // TODO: offset of the canvas, because canvas may show only
            // TOSO: a part of big world (zoom, scroll)
            let width = gui_ref.width;
            let x = offs as u32 % width;
            let y = offs as u32 / width;
            gui_ref.canvas.put_pixel(x, y, im::Rgba(ATOM_COLORS[get_type(atom) as usize]));
        }
    });
}