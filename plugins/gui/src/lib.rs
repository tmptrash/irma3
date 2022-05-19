//!
//! Implementation of GUI plugin. Gives an ability to visualize atoms, 
//! molecules and all the stuff inside the world.
//!
pub mod defs;

extern crate piston_window;
extern crate image as im;
extern crate vecmath;

use defs::{ATOM_COLORS, BLACK_COLOR_F32, BLACK_COLOR_U8};
use piston_window::{Event, TextureContext, PistonWindow, WindowSettings, Texture, Filter, Key};
use piston_window::{OpenGL, RenderEvent, Window, Transformed, TextureSettings, MouseCursorEvent};
use piston_window::{MouseScrollEvent, Button, PressEvent};
use piston_window::{image, clear};
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
#[no_mangle] pub fn idle(core: &Core) {
    let gui_ref = unsafe { &mut GUI }.as_mut().unwrap();
    let win = &mut gui_ref.win;
    if let Some(e) = win.next() {
        if let Some(pos) = e.mouse_cursor_args() {                             // save current mouse x,y
            gui_ref.mouse_pos = pos;
        }
        if e.render_args().is_some() {                                         // Rerender the world
            render(&e, gui_ref);
        }
        if let Some(pos) = e.mouse_scroll_args() {                             // User uses mouse scroll
            on_zoom(gui_ref, core, pos[1]);
            //gui_ref.zoom += pos[1];
        }
        if let Some(b) = e.press_args() {
            if b == Button::Keyboard(Key::Up) {
                gui_ref.shift_y += 10.0;
            } else if b == Button::Keyboard(Key::Down) {
                gui_ref.shift_y -= 10.0;
            } else if b == Button::Keyboard(Key::Left) {
                gui_ref.shift_x += 10.0;
            } else if b == Button::Keyboard(Key::Right) {
                gui_ref.shift_x -= 10.0;
            }
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
        .exit_on_esc(false)
        .graphics_api(OpenGL::V3_2)
        .fullscreen(false)
        .build()
        .unwrap();
    let mut texture_ctx = TextureContext {
        factory: win.factory.clone(),
        encoder: win.factory.create_command_buffer().into()
    };
    let mut settings = TextureSettings::new();
    settings.set_mag(Filter::Nearest); // turn off antialiasing
    let texture = Texture::from_image(&mut texture_ctx, &canvas, &settings).unwrap();

    unsafe {
        GUI = Some(Gui {
            width,
            height,
            win,
            canvas,
            texture_ctx,
            texture,
            zoom: 1.0,
            shift_x: 0.0,
            shift_y: 0.0,
            mouse_pos: [0.0, 0.0]
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
        // Update texture before rendering.
        gui.texture_ctx.encoder.flush(device);
        image(&gui.texture, c.transform.zoom(gui.zoom).trans(gui.shift_x, gui.shift_y), g);
    });
}
///
/// Mouse scroll handler
///
fn on_zoom(gui: &mut Gui, core: &Core, scroll: f64) {
    let z = gui.zoom;
    let m_x = gui.mouse_pos[0] - gui.shift_x * z;                              // Mouse x,y in a world
    let m_y = gui.mouse_pos[1] - gui.shift_y * z;
    let width = core.cfg.WIDTH() as f64 * z;
    let height = core.cfg.HEIGHT() as f64 * z;
    //if m_x < 0.0 || m_x >= width || m_y < 0.0 || m_y >= height { return }
    let ratio_x = m_x / width;
    let ratio_y = m_y / height;

    if core.cfg.zoom != 0 {                                                    // Calculate new zoom coefficient
        let old_zoom = gui.zoom;
        gui.zoom += scroll / core.cfg.zoom.abs() as f64;
        if gui.zoom <= 0.0 { gui.zoom = old_zoom }
    }

    let z = gui.zoom;                                                          // Updated zoom and shift
    gui.shift_x = -(core.cfg.WIDTH()  as f64 * z * ratio_x - gui.mouse_pos[0]) / z;
    gui.shift_y = -(core.cfg.HEIGHT() as f64 * z * ratio_y - gui.mouse_pos[1]) / z;
}