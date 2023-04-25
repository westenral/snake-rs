use std::collections::VecDeque;

use glfw_window::GlfwWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use rand::prelude::*;

const BG_COLOR: [f32; 4] = [0.1; 4];
const SNAKE_COLOR: [f32; 4] = [0.3, 0.3, 0.9, 1.0];
const FOOD_COLOR: [f32; 4] = [0.7, 0.4, 0.1, 1.0];
const CELL_SIZE: f64 = 20.0;
const CELL_EDGE_BUFFER: f64 = 2.0;
const PLAYFIELD_WIDTH: f64 = 640.0 / CELL_SIZE;
const PLAYFIELD_HEIGHT: f64 = 480.0 / CELL_SIZE;
const UPDATE_INTERVAL: f64 = 1.0 / 4.0;

struct GameState {
    spos: [f64; 2],
    sdir: [f64; 2],
    tail: VecDeque<[f64; 2]>,
    tail_length: isize,
    fpos: [f64; 2],
    interval_time: f64,
}

impl GameState {
    fn new() -> Self {
        GameState {
            spos: [16.0, 12.0],
            sdir: [0.0; 2],
            tail: Vec::new().into(),
            tail_length: 0,
            fpos: [8.0; 2],
            interval_time: 0.0,
        }
    }

    fn randomize_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_loc = Self::get_new_food_loc(&mut rng);
        while self.tail.contains(&new_loc) {
            new_loc = Self::get_new_food_loc(&mut rng);
        }
        self.fpos = new_loc;
    }

    fn get_new_food_loc(rng: &mut ThreadRng) -> [f64; 2] {
        [
            rng.gen_range(0..PLAYFIELD_WIDTH as usize) as f64,
            rng.gen_range(0..PLAYFIELD_HEIGHT as usize) as f64,
        ]
    }

    fn check_if_past_timestep(&mut self, dt: f64) -> bool {
        self.interval_time += dt;
        if self.interval_time < UPDATE_INTERVAL {
            return false;
        }
        self.interval_time -= UPDATE_INTERVAL;
        return true;
    }

    fn update_snake_position(&mut self) {
        self.spos[0] += self.sdir[0];
        self.spos[1] += self.sdir[1];
        // branchless screen wrapping
        self.spos[0] += PLAYFIELD_WIDTH
            * ((self.spos[0] < 0.0) as u8 as f64
                + -((self.spos[0] > PLAYFIELD_WIDTH - 1.0) as u8 as f64));
        self.spos[1] += PLAYFIELD_HEIGHT
            * ((self.spos[1] < 0.0) as u8 as f64
                + -((self.spos[1] > PLAYFIELD_HEIGHT - 1.0) as u8 as f64));
    }

    fn check_food_collisision(&mut self) {
        if self.spos == self.fpos {
            self.randomize_food();
            self.tail_length += 1;
            self.tail.push_back(self.spos);
        }
    }

    // Call before updating snake position
    fn update_tail(&mut self) {
        if self.tail_length == 0 {
            return;
        }
        self.tail.push_back(self.spos);
        self.tail.rotate_right(1);
        self.tail.pop_back();
    }

    fn update(&mut self, dt: f64) {
        if !self.check_if_past_timestep(dt) {
            return;
        }

        self.update_tail();
        self.update_snake_position();
        self.check_food_collisision();
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
            let mut tail_vec_deque = self.tail.clone();
            tail_vec_deque.push_front(self.spos);
            let tail_vec: Vec<[f64; 2]> = tail_vec_deque.into();

            for segment in tail_vec {
                rectangle(
                    SNAKE_COLOR,
                    [
                        segment[0] * CELL_SIZE + CELL_EDGE_BUFFER,
                        segment[1] * CELL_SIZE + CELL_EDGE_BUFFER,
                        CELL_SIZE - 2.0 * CELL_EDGE_BUFFER,
                        CELL_SIZE - 2.0 * CELL_EDGE_BUFFER,
                    ],
                    c.transform,
                    g,
                );
            }
            rectangle(
                FOOD_COLOR,
                [
                    self.fpos[0] * CELL_SIZE + CELL_EDGE_BUFFER,
                    self.fpos[1] * CELL_SIZE + CELL_EDGE_BUFFER,
                    CELL_SIZE - 2.0 * CELL_EDGE_BUFFER,
                    CELL_SIZE - 2.0 * CELL_EDGE_BUFFER,
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
        .vsync(true)
        .build()
        .unwrap();

    let gl = &mut GlGraphics::new(opengl);

    let mut gamestate = GameState::new();

    let mut events = Events::new(EventSettings::new().ups(60));
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
