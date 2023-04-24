use glfw_window::GlfwWindow;
//use graphics::{Context, Graphics, Transformed};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

const BG_COLOR: [f32; 4] = [0.1; 4];

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlfwWindow = WindowSettings::new("piston-example-user_input", [640, 480])
        .exit_on_esc(true)
        .resizable(false)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let gl = &mut GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        // activates on press or release
        if let Some(args) = e.button_args() {
            println!("Scancode {:?}", args.scancode);
        }
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                graphics::clear(BG_COLOR, g);
                graphics::rectangle(
                    [0.0, 0.0, 1.0, 1.0],
                    [25.0, 25.0, 50.0, 50.0],
                    c.transform,
                    g,
                );
            });
        }
        if let Some(_args) = e.update_args() {
		}
    }
}
