use crate::iterations::{julia_pixels, mandelbrot_pixels};
use crate::window_size::*;
use mandelbruhst_cli::palette::ColorPalette;
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use std::time::Instant;

const MAX_ITERATION_DEFAULT: u32 = 50;
const MAX_ITERATION_JUMP: u32 = 4;
const MAX_ITERATION_LOWER_BOUND: u32 = 10;
const KEY_TRANSLATION_DISTANCE: f64 = 0.03;

const MANDELBROT_MAX_ITERATION: u32 = 100;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Params {
    pub c_re: f64,
    pub c_im: f64,
    pub max_iterations: u32,
}

impl Params {
    pub fn new() -> Self {
        Params {
            c_re: 0.0,
            c_im: 0.0,
            max_iterations: MAX_ITERATION_DEFAULT,
        }
    }

    pub fn update(&mut self, window: &Window) {
        // Translation
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Discard) {
            let pos_x = mouse_x as f64 / WIDTH_F - 0.5;
            let pos_y = -mouse_y as f64 / HEIGHT_F + 0.5;
            self.c_re = pos_x * X_RANGE;
            self.c_im = pos_y * Y_RANGE;
        }
        if window.is_key_down(Key::W) {
            self.c_im += KEY_TRANSLATION_DISTANCE;
        }
        if window.is_key_down(Key::S) {
            self.c_im -= KEY_TRANSLATION_DISTANCE;
        }
        if window.is_key_down(Key::D) {
            self.c_re += KEY_TRANSLATION_DISTANCE;
        }
        if window.is_key_down(Key::A) {
            self.c_re -= KEY_TRANSLATION_DISTANCE;
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

pub struct State {
    params: Params,
    pixels: Vec<u32>,
    ref_pixels: Vec<u32>,
    palette: ColorPalette,
}

impl State {
    pub fn new(palette: ColorPalette) -> Self {
        State {
            params: Params::new(),
            pixels: vec![],
            ref_pixels: mandelbrot_pixels(MANDELBROT_MAX_ITERATION, &palette),
            palette,
        }
    }

    fn update(&mut self, window: &Window) {
        // Only update pixels if the parameters have been changed
        let old_params = self.params.clone();
        self.params.update(window);
        if self.params != old_params {
            let start_time = Instant::now();
            self.pixels = julia_pixels(&self.params, &self.palette);
            let elapsed_time = format!("{:.2?}", start_time.elapsed());
            println!(
                "{:>9} for: Max iters = {:3}, c = {:6.3} + {:6.3}i",
                elapsed_time, self.params.max_iterations, self.params.c_re, self.params.c_im,
            );
        }
    }

    pub fn run(&mut self) {
        let mut window =
            Window::new("Julia Explorer", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

        while window.is_open() && !window.is_key_down(Key::Escape) {
            let pixels = if window.is_key_down(Key::Space) {
                &self.ref_pixels
            } else {
                self.update(&window);
                &self.pixels
            };
            window.update_with_buffer(pixels, WIDTH, HEIGHT).unwrap();
        }
    }
}
