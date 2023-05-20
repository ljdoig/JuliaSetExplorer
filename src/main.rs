use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rayon::prelude::*;
use std::time::Instant;

use explorer::josh_palette::ColorPalette;
use explorer::mandelbrot::get_iterations;
use explorer::*;

const MAX_ITERATION_DEFAULT: u32 = 50;
const MAX_ITERATION_JUMP: u32 = 4;
const MAX_ITERATION_LOWER_BOUND: u32 = 10;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
struct Params {
    c_x: f64,
    c_y: f64,
    max_iterations: u32,
}

impl Params {
    fn new() -> Self {
        Params {
            c_x: 0.0,
            c_y: 0.0,
            max_iterations: MAX_ITERATION_DEFAULT,
        }
    }

    fn get_pixels(&self, palette: &ColorPalette, mandelbrot: bool) -> Vec<u32> {
        let precomp1 = X_RANGE / WIDTH_F;
        let precomp2 = -X_RANGE / 2.0;
        let precomp3 = -Y_RANGE / HEIGHT_F;
        let precomp4 = Y_RANGE / 2.0;
        (0..HEIGHT * WIDTH)
            .into_par_iter()
            .map(|i| {
                let col = i % WIDTH;
                let row = i / WIDTH;
                let x = col as f64 * precomp1 + precomp2;
                let y = row as f64 * precomp3 + precomp4;
                if mandelbrot {
                    get_iterations(0.0, 0.0, x, y, 200)
                } else {
                    get_iterations(x, y, self.c_x, self.c_y, self.max_iterations)
                }
            })
            .map(|iterations| palette.value(iterations / self.max_iterations as f64))
            .collect()
    }

    fn update(&mut self, window: &Window) {
        // Translation
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Discard) {
            let pos_x = mouse_x as f64 / WIDTH_F - 0.5;
            let pos_y = -mouse_y as f64 / HEIGHT_F + 0.5;
            self.c_x = pos_x * X_RANGE;
            self.c_y = pos_y * Y_RANGE;
        }

        // Changing max iterations
        if window.get_mouse_down(MouseButton::Left) || window.is_key_down(Key::Left) {
            if self.max_iterations == MAX_ITERATION_LOWER_BOUND {
                self.max_iterations = MAX_ITERATION_JUMP;
            } else {
                self.max_iterations += MAX_ITERATION_JUMP;
            }
        }
        if window.get_mouse_down(MouseButton::Right) || window.is_key_down(Key::Right) {
            if self.max_iterations <= MAX_ITERATION_JUMP {
                self.max_iterations = MAX_ITERATION_LOWER_BOUND;
            } else {
                self.max_iterations -= MAX_ITERATION_JUMP;
            }
        }

        // Reset
        if window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No) {
            self.max_iterations = MAX_ITERATION_DEFAULT;
        }
    }
}

struct State {
    params: Params,
    pixels: Vec<u32>,
    palette: ColorPalette,
}

impl State {
    fn new() -> Self {
        State {
            params: Params::new(),
            pixels: vec![],
            palette: ColorPalette::default(),
        }
    }

    fn update(&mut self, window: &Window) {
        let old_params = self.params.clone();

        self.params.update(window);

        // Only update pixels if the parameters have been changed
        if self.params != old_params || window.is_key_down(Key::Space) {
            let start_time = Instant::now();
            self.pixels = self
                .params
                .get_pixels(&self.palette, window.is_key_down(Key::Space));
            let elapsed_time = format!("{:.2?}", start_time.elapsed());
            println!(
                "{:>9} for: Max iters = {:3}, c = {:6.3} + {:6.3}i",
                elapsed_time, self.params.max_iterations, self.params.c_x, self.params.c_y,
            );
        }
    }
}

fn main() {
    let mut window =
        Window::new("Julia Explorer", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let mut state = State::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update the state in case of input
        state.update(&window);

        // Update the window based on the current state
        window
            .update_with_buffer(&state.pixels, WIDTH, HEIGHT)
            .unwrap();
    }
}
