use std::ops::Add;
use std::ops::AddAssign;

use glfw_window::GlfwWindow;
use graphics::Context;
//use graphics::{Context, Graphics, Transformed};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

const BG_COLOR: [f32; 4] = [0.1; 4];
const SNAKE_COLOR: [f32; 4] = [0.3, 0.3, 0.9, 1.0];
const CELL_SIZE: f64 = 20.0;
const PLAYFIELD_WIDTH: f64 = 640.0 / CELL_SIZE;
const PLAYFIELD_HEIGHT: f64 = 480.0 / CELL_SIZE;
const UPDATE_INTERVAL: f64 = 1.0 / 4.0;

struct GameState {
    spos: [f64; 2],
    sdir: [f64; 2],
    tail: Vec<[f64; 2]>,
    tail_length: isize,
    fpos: [f64; 2],
    interval_time: f64,
}

impl GameState {
    fn new() -> Self {
        GameState {
            spos: [0.0; 2],
            sdir: [0.0; 2],
            tail: vec![],
            tail_length: 0,
            fpos: [0.0; 2],
            interval_time: 0.0,
        }
    }

    fn randomize_food(&mut self) {}

    fn update(&mut self, dt: f64) {
        self.interval_time += dt;
        if self.interval_time < UPDATE_INTERVAL {
            return;
        }
        self.interval_time -= UPDATE_INTERVAL;

        self.spos[0] += self.sdir[0];
        self.spos[1] += self.sdir[1];
        // branchless wrapping
        self.spos[0] += PLAYFIELD_WIDTH
            * ((self.spos[0] < 0.0) as u8 as f64
                + -((self.spos[0] > PLAYFIELD_WIDTH - 1.0) as u8 as f64));
        self.spos[1] += PLAYFIELD_HEIGHT
            * ((self.spos[1] < 0.0) as u8 as f64
                + -((self.spos[1] > PLAYFIELD_HEIGHT - 1.0) as u8 as f64));
    }

    fn button_event(&mut self, args: ButtonArgs) {
        if args.state == ButtonState::Release {
            return;
        }
        match args.button {
            Button::Keyboard(Key::D) => self.sdir = [1.0, 0.0],
            Button::Keyboard(Key::A) => self.sdir = [-1.0, 0.0],
            Button::Keyboard(Key::S) => self.sdir = [0.0, 1.0],
            Button::Keyboard(Key::W) => self.sdir = [0.0, -1.0],
            _ => {}
        }
    }

    fn render(&self, args: RenderArgs, gl: &mut GlGraphics) {
        use graphics::{clear, rectangle};
        gl.draw(args.viewport(), |c, g| {
            clear(BG_COLOR, g);
            rectangle(
                SNAKE_COLOR,
                [
                    self.spos[0] * CELL_SIZE,
                    self.spos[1] * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                ],
                c.transform,
                g,
            );
        })
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlfwWindow = WindowSettings::new("Snake", [640, 480])
        .exit_on_esc(true)
        .resizable(false)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let gl = &mut GlGraphics::new(opengl);

    let mut gamestate = GameState::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        // activates on press or release
        if let Some(args) = e.button_args() {
            gamestate.button_event(args);
        }
        if let Some(args) = e.render_args() {
            gamestate.render(args, gl);
        }
        if let Some(args) = e.update_args() {
            gamestate.update(args.dt);
        }
    }
}
