use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rayon::prelude::*;
use std::time::Instant;

mod josh_palette;
use josh_palette::ColorPalette;

const WIDTH: usize = 1200;
const HEIGHT: usize = 800;
const WIDTH_F: f64 = WIDTH as f64;
const HEIGHT_F: f64 = HEIGHT as f64;
const X_RANGE: f64 = 4.5;
const Y_RANGE: f64 = X_RANGE * HEIGHT_F / WIDTH_F;

const MAX_ITERATION_DEFAULT: u32 = 75;
const MAX_ITERATION_JUMP: u32 = 15;
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

    fn get_iterations(&self, mut z_re: f64, mut z_im: f64) -> f64 {
        let c_re = self.c_x;
        let c_im = self.c_y;
        let mut iterations = 0;
        while iterations < self.max_iterations && z_re * z_re + z_im * z_im <= 4.0 {
            let z_re_ = z_re;
            z_re = (z_re + z_im) * (z_re - z_im) + c_re;
            z_im = 2.0 * z_re_ * z_im + c_im;
            iterations += 1;
        }
        iterations as f64
    }

    fn get_pixels(&self, palette: &ColorPalette) -> Vec<u32> {
        let start_time = Instant::now();
        let precomp1 = X_RANGE / WIDTH_F;
        let precomp2 = -X_RANGE / 2.0;
        let precomp3 = -Y_RANGE / HEIGHT_F;
        let precomp4 = Y_RANGE / 2.0;
        let output = (0..HEIGHT * WIDTH)
            .into_par_iter()
            .map(|i| {
                let col = i % WIDTH;
                let row = i / WIDTH;
                let x = col as f64 * precomp1 + precomp2;
                let y = row as f64 * precomp3 + precomp4;
                self.get_iterations(x, y)
            })
            .map(|iterations| palette.value(iterations / self.max_iterations as f64))
            .collect();
        let elapsed_time = format!("{:.2?}", start_time.elapsed());
        println!(
            "{:>9} for: Max iters = {:3}, c = {:.3} + {:.3}i",
            elapsed_time, self.max_iterations, self.c_x, self.c_y,
        );
        output
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
        if window.get_mouse_down(MouseButton::Left) {
            if self.max_iterations == MAX_ITERATION_LOWER_BOUND {
                self.max_iterations = MAX_ITERATION_JUMP;
            } else {
                self.max_iterations += MAX_ITERATION_JUMP;
            }
        }
        if window.get_mouse_down(MouseButton::Right) {
            if self.max_iterations <= MAX_ITERATION_JUMP {
                self.max_iterations = MAX_ITERATION_LOWER_BOUND;
            } else {
                self.max_iterations -= MAX_ITERATION_JUMP;
            }
        }

        // Reset
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
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
        let palette = ColorPalette::default();
        let params = Params::new();
        let pixels = params.get_pixels(&palette);
        State {
            params,
            pixels,
            palette,
        }
    }

    fn update(&mut self, window: &Window) {
        let old_params = self.params.clone();

        self.params.update(window);

        // Only update pixels if the parameters have been changed
        if self.params != old_params {
            self.pixels = self.params.get_pixels(&self.palette);
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
