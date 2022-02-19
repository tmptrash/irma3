//!
//! Implementation of GUI plugin. Gives an ability to visualize atoms, 
//! molecules and all the stuff inside the world.
//!
pub mod defs;

extern crate piston_window;
extern crate image as im;
extern crate vecmath;

use defs::{ATOM_COLORS, BLACK_COLOR_F32, BLACK_COLOR_U8};
use piston_window::{Event, TextureContext, PistonWindow, WindowSettings, Texture};
use piston_window::{OpenGL, RenderEvent, Window, Transformed, TextureSettings};
use piston_window::{image, clear};
use share::io::IO;
use share::io::events::{EVENT_SET_DOT, EVENT_MOVE_DOT};
use share::io::Param;
use share::atom::get_type;
use share::Core;
use defs::Gui;
///
/// Plugin's internal Gui data
///
static mut GUI: Option<Gui> = None;
///
/// Plugin API. initializes plugin. Creates piston windows, canvas, context an
/// all needed stuff for drawing in 2D
///
#[no_mangle] fn init(core: &mut Core) {
    create_gui(core);
    add_listeners(core);
}
///
/// Plugin API. Do main work by haddling GUI and user events, drawing atoms
///
#[no_mangle] pub fn idle(_core: &IO) {
    let gui_ref = unsafe { &mut GUI }.as_mut().unwrap();
    let win = &mut gui_ref.win;
    if let Some(e) = win.next() {
        if e.render_args().is_some() {
            render(&e, gui_ref);
        }
    }
}
///
/// Plugin API. Destroys plugin.
///
#[no_mangle] pub fn remove(_core: &Core) {
    let gui_ref = unsafe { &mut GUI }.as_mut().unwrap();
    let win = &mut gui_ref.win;
    win.set_should_close(true);
}

fn create_gui(core: &mut Core) {
    let width = core.cfg.WIDTH() as u32;
    let height = core.cfg.HEIGHT() as u32;
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
fn add_listeners(core: &mut Core) {
    //
    // If a dot was added into the world we have to draw it on a canvas
    //
    core.io.on(EVENT_SET_DOT, |p| {
        let gui_ref = unsafe { &mut GUI }.as_mut().unwrap();
        if let Param::SetDot(offs, atom) = p {
            let w = gui_ref.width;
            gui_ref.canvas.put_pixel(*offs as u32 % w, *offs as u32 / w, ATOM_COLORS[get_type(*atom) as usize]);
        }
    });
    //
    // If a dot was moved in the world we have to update it on a canvas
    //
    core.io.on(EVENT_MOVE_DOT, |p| {
        let gui_ref = unsafe { &mut GUI }.as_mut().unwrap();
        if let Param::MoveDot(offs0, offs1, atom) = p {
            let w = gui_ref.width;
            gui_ref.canvas.put_pixel(*offs1 as u32 % w, *offs1 as u32 / w, ATOM_COLORS[get_type(*atom) as usize]);
            gui_ref.canvas.put_pixel(*offs0 as u32 % w, *offs0 as u32 / w, BLACK_COLOR_U8);
        }
    });
}
///
/// Rendering one frame
///
fn render(e: &Event, gui: &mut Gui) {
    gui.texture.update(&mut gui.texture_ctx, &gui.canvas).unwrap();
    gui.win.draw_2d(e, |c, g, device| {
        clear(BLACK_COLOR_F32, g);
        c.zoom(gui.zoom);
        // Update texture before rendering.
        gui.texture_ctx.encoder.flush(device);
        image(&gui.texture, c.transform, g);
    });
}