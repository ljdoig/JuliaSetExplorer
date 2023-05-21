use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use std::time::Instant;

use explorer::josh_palette::ColorPalette;
use explorer::mandelbrot::{julia_pixels, mandelbrot_pixels};
use explorer::*;

const MAX_ITERATION_DEFAULT: u32 = 50;
const MAX_ITERATION_JUMP: u32 = 4;
const MAX_ITERATION_LOWER_BOUND: u32 = 10;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
struct Params {
    c_re: f64,
    c_im: f64,
    max_iterations: u32,
}

impl Params {
    fn new() -> Self {
        Params {
            c_re: 0.0,
            c_im: 0.0,
            max_iterations: MAX_ITERATION_DEFAULT,
        }
    }

    fn update(&mut self, window: &Window) {
        // Translation
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Discard) {
            let pos_x = mouse_x as f64 / WIDTH_F - 0.5;
            let pos_y = -mouse_y as f64 / HEIGHT_F + 0.5;
            self.c_re = pos_x * X_RANGE;
            self.c_im = pos_y * Y_RANGE;
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
        if window.is_key_pressed(Key::Enter, KeyRepeat::No) {
            self.max_iterations = MAX_ITERATION_DEFAULT;
        }
    }
}

struct State {
    params: Params,
    pixels: Vec<u32>,
    ref_pixels: Vec<u32>,
    palette: ColorPalette,
}

impl State {
    fn new() -> Self {
        let params = Params::new();
        let palette = ColorPalette::default();
        State {
            params,
            pixels: vec![],
            ref_pixels: mandelbrot_pixels(150, &palette),
            palette,
        }
    }

    fn update(&mut self, window: &Window) {
        let old_params = self.params.clone();

        self.params.update(window);

        // Only update pixels if the parameters have been changed
        if self.params != old_params {
            let start_time = Instant::now();
            self.pixels = julia_pixels(
                self.params.c_re,
                self.params.c_im,
                self.params.max_iterations,
                &self.palette,
            );
            let elapsed_time = format!("{:.2?}", start_time.elapsed());
            println!(
                "{:>9} for: Max iters = {:3}, c = {:6.3} + {:6.3}i",
                elapsed_time, self.params.max_iterations, self.params.c_re, self.params.c_im,
            );
        }
    }
}

fn main() {
    let mut state = State::new();

    let mut window = Window::new(
        "Julia Explorer",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    // window.set_cursor_style(minifb::CursorStyle::Crosshair);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let pixels = if window.is_key_down(Key::Space) {
            &state.ref_pixels
        } else {
            state.update(&window);
            &state.pixels
        };
        window.update_with_buffer(pixels, WIDTH, HEIGHT).unwrap();
    }
}
