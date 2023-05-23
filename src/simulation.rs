use crate::iterations::{julia_pixels, mandelbrot_pixels};
use crate::window_size::*;
use mandelbruhst_cli::palette::ColorPalette;
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use std::time::Instant;

const MAX_ITERATION_DEFAULT: u32 = 60;
const MAX_ITERATION_JUMP: u32 = 3;
const MAX_ITERATION_LOWER_BOUND: u32 = 12;
const MANDELBROT_MAX_ITERATION: u32 = 100;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct JuliaParams {
    pub c_re: f64,
    pub c_im: f64,
    pub max_iterations: u32,
}

impl JuliaParams {
    pub fn new() -> Self {
        JuliaParams {
            c_re: 0.0,
            c_im: 0.0,
            max_iterations: MAX_ITERATION_DEFAULT,
        }
    }

    pub fn update(&mut self, window: &Window, width: usize, height: usize) {
        // Translation
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Pass) {
            let pos_x = mouse_x as f64 / width as f64 - 0.5;
            let pos_y = -mouse_y as f64 / height as f64 + 0.5;
            let (x_range, y_range) = x_y_ranges(width, height);
            self.c_re = pos_x * x_range;
            self.c_im = pos_y * y_range;
        }

        // Changing max iterations
        if window.get_mouse_down(MouseButton::Left) || window.is_key_down(Key::Left) {
            self.max_iterations += MAX_ITERATION_JUMP;
        }
        if window.get_mouse_down(MouseButton::Right) || window.is_key_down(Key::Right) {
            self.max_iterations -= MAX_ITERATION_JUMP;
            if self.max_iterations < MAX_ITERATION_LOWER_BOUND {
                self.max_iterations = MAX_ITERATION_LOWER_BOUND;
            }
        }

        // Reset
        if window.is_key_pressed(Key::Enter, KeyRepeat::No) {
            self.max_iterations = MAX_ITERATION_DEFAULT;
        }
    }
}

pub struct State {
    params: JuliaParams,
    pixels: Vec<u32>,
    ref_pixels: Vec<u32>,
    palette: ColorPalette,
}

impl State {
    pub fn new(palette: ColorPalette) -> Self {
        State {
            params: JuliaParams::new(),
            pixels: vec![],
            ref_pixels: vec![],
            palette,
        }
    }

    fn update(&mut self, window: &Window, width: usize, height: usize) -> &[u32] {
        if window.is_key_down(Key::Space) {
            // Only update ref_pixels if the window has resized
            if self.ref_pixels.len() != width * height {
                self.ref_pixels =
                    mandelbrot_pixels(MANDELBROT_MAX_ITERATION, &self.palette, width, height)
            }
            &self.ref_pixels
        } else {
            // Only update pixels if the parameters have been changed
            let old_params = self.params.clone();
            self.params.update(window, width, height);
            if self.params != old_params || self.pixels.len() != width * height {
                let start_time = Instant::now();
                self.pixels = julia_pixels(&self.params, &self.palette, width, height);
                let elapsed_time = format!("{:.2?}", start_time.elapsed());
                println!(
                    "{:>9} for: Max iters = {:3}, width = {:4}, height {:4}, c = {:6.3} + {:6.3}i",
                    elapsed_time,
                    self.params.max_iterations,
                    width,
                    height,
                    self.params.c_re,
                    self.params.c_im,
                );
            }
            &self.pixels
        }
    }

    pub fn run(&mut self) {
        let mut window = Window::new(
            "Julia Explorer",
            INITIAL_WIDTH,
            INITIAL_HEIGHT,
            WindowOptions {
                resize: true,
                ..WindowOptions::default()
            },
        )
        .unwrap();
        while window.is_open() && !window.is_key_down(Key::Escape) {
            // Ensure width and height are consistent throughout loop
            let (width, height) = window.get_size();
            let pixels = self.update(&window, width, height);
            window.update_with_buffer(pixels, width, height).unwrap();
        }
    }
}
