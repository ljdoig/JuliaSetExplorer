use crate::iterations::{julia_pixels, mandelbrot_pixels};
use crate::window_size::*;
use mandelbruhst_cli::palette::{ColorPalette, ConfigRGB};
use minifb::{Key, KeyRepeat, MouseMode, Window};

const MAX_ITERATION_DEFAULT: u32 = 70;
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

    pub fn update(&mut self, window: &Window) {
        // Translation
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Pass)
        {
            let (mouse_x, mouse_y) = (mouse_x + 270.0, mouse_y + 150.0);
            let (width, height) = window.get_size();
            let pos_x = mouse_x as f64 / width as f64 - 0.5;
            let pos_y = -mouse_y as f64 / height as f64 + 0.5;
            let (x_range, y_range) = x_y_ranges(width, height);
            self.c_re = pos_x * x_range;
            self.c_im = pos_y * y_range;
        }

        // Changing max iterations
        if window.is_key_down(Key::Left) {
            self.max_iterations += MAX_ITERATION_JUMP;
        }
        if window.is_key_down(Key::Right) {
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
    pub fn new() -> Self {
        let palette = ColorPalette::new(vec![
            ConfigRGB {
                value: 0.0,
                red: 0,
                green: 18,
                blue: 25,
            },
            ConfigRGB {
                value: 0.1,
                red: 20,
                green: 33,
                blue: 61,
            },
            ConfigRGB {
                value: 0.25,
                red: 252,
                green: 163,
                blue: 17,
            },
            ConfigRGB {
                value: 0.5,
                red: 229,
                green: 229,
                blue: 229,
            },
            ConfigRGB {
                value: 1.0,
                red: 255,
                green: 255,
                blue: 255,
            },
        ])
        .unwrap();
        State {
            params: JuliaParams::new(),
            pixels: vec![],
            ref_pixels: vec![],
            palette,
        }
    }

    pub fn update(
        &mut self,
        window: &Window,
        width: usize,
        height: usize,
    ) -> &[u32] {
        if window.is_key_down(Key::Space) {
            // Only update ref_pixels if the window has resized
            if self.ref_pixels.len() != width * height {
                self.ref_pixels = mandelbrot_pixels(
                    MANDELBROT_MAX_ITERATION,
                    &self.palette,
                    width,
                    height,
                )
            }
            &self.ref_pixels
        } else {
            // Only update pixels if the parameters have been changed or the window has resized
            let old_params = self.params.clone();
            self.params.update(window);
            if self.params != old_params || self.pixels.len() != width * height
            {
                self.pixels =
                    julia_pixels(&self.params, &self.palette, width, height);
            }
            &self.pixels
        }
    }
}
