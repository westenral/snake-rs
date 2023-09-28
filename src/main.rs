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
const CELL_SIZE: f64 = 80.0;
const CELL_EDGE_BUFFER: f64 = 8.0;
const SCREEN_WIDTH: f64 = 1280.0;
const SCREEN_HEIGHT: f64 = 800.0;
const PLAYFIELD_WIDTH: f64 = SCREEN_WIDTH / CELL_SIZE;
const PLAYFIELD_HEIGHT: f64 = SCREEN_HEIGHT / CELL_SIZE;
const UPDATE_INTERVAL: f64 = 1.0 / 4.0;

enum RenderMode {
    Fixed,
    Follow,
}

struct GameState {
    is_paused: bool,
    is_game_running: bool,
    interval_time: f64,
    render_mode: RenderMode,

    spos: [f64; 2],
    pspos: [f64; 2],
    sdir: [f64; 2],
    next_sdir: [f64; 2],

    tail: VecDeque<[f64; 2]>,
    tail_length: isize,

    fpos: [f64; 2],
}

impl GameState {
    fn new() -> Self {
        GameState {
            is_paused: false,
            is_game_running: true,
            spos: [PLAYFIELD_WIDTH / 2.0, PLAYFIELD_HEIGHT / 2.0],
            pspos: [PLAYFIELD_WIDTH / 2.0, PLAYFIELD_HEIGHT / 2.0],
            sdir: [0.0; 2],
            next_sdir: [0.0; 2],
            tail: Vec::new().into(),
            tail_length: 0,
            fpos: [PLAYFIELD_WIDTH / 2.0 - 4.0, PLAYFIELD_HEIGHT / 2.0 - 4.0],
            interval_time: 0.0,
            render_mode: RenderMode::Fixed,
        }
    }

    fn randomize_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_loc = Self::get_new_food_loc(&mut rng);
        let mut tail = self.tail.clone();
        tail.push_back(self.spos);
        while tail.contains(&new_loc) {
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

    fn limit_direction(&mut self) {
        let dir_diff = [
            self.sdir[0] + self.next_sdir[0],
            self.sdir[1] + self.next_sdir[1],
        ];
        if dir_diff == [0.0, 0.0] {
            self.next_sdir = self.sdir;
        }
        self.sdir = self.next_sdir;
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

    fn update_snake_position(&mut self) {
        self.pspos = self.spos;
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

    fn food_collisision(&mut self) {
        if self.spos == self.fpos {
            self.randomize_food();
            self.tail_length += 1;
            self.tail.push_front(self.pspos);
        }
    }

    fn tail_collision(&mut self) {
        self.is_game_running = !self.tail.contains(&self.spos)
    }

    fn update(&mut self, dt: f64) {
        if !self.is_game_running || self.is_paused {
            return;
        }
        if !self.check_if_past_timestep(dt) {
            return;
        }

        self.update_tail();
        self.limit_direction();
        self.update_snake_position();
        self.food_collisision();
        self.tail_collision();
    }

    fn button_event(&mut self, args: ButtonArgs) {
        if args.state == ButtonState::Release {
            return;
        }
        if args.button == Button::Keyboard(Key::Escape) {
            self.is_paused ^= true
        };
        if self.is_paused {
            return;
        }
        match args.button {
            Button::Keyboard(Key::D) => self.next_sdir = [1.0, 0.0],
            Button::Keyboard(Key::A) => self.next_sdir = [-1.0, 0.0],
            Button::Keyboard(Key::S) => self.next_sdir = [0.0, 1.0],
            Button::Keyboard(Key::W) => self.next_sdir = [0.0, -1.0],
            Button::Keyboard(Key::Space) => {
                if !self.is_game_running {
                    *self = GameState::new();
                }
            }
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

            for segment in &tail_vec {
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

            'connections: for section in tail_vec.windows(2) {
                if (section[0][0] - section[1][0]).abs() > 1.0
                    || (section[0][1] - section[1][1]).abs() > 1.0
                {
                    continue 'connections;
                }
                graphics::rectangle_from_to(
                    SNAKE_COLOR,
                    [
                        section[0][0] * CELL_SIZE + CELL_EDGE_BUFFER,
                        section[0][1] * CELL_SIZE + CELL_EDGE_BUFFER,
                    ],
                    [
                        (section[1][0] + 1.0) * CELL_SIZE - CELL_EDGE_BUFFER,
                        (section[1][1] + 1.0) * CELL_SIZE - CELL_EDGE_BUFFER,
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
    let mut window: GlfwWindow = WindowSettings::new("Snake", [SCREEN_WIDTH, SCREEN_HEIGHT])
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
