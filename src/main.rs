use image::Rgb;
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use rayon::prelude::*;
use std::time::{Duration, Instant};

mod josh_palette;
use josh_palette::ColorPalette;

const WIDTH: usize = 900;
const HEIGHT: usize = 600;
const WIDTH_F: f64 = WIDTH as f64;
const HEIGHT_F: f64 = HEIGHT as f64;

const MAX_ITERATION_JUMP: u32 = 25;
const MAX_ITERATION_LOWER_BOUND: u32 = 25;
const KEYS_TRANSLATION: f64 = 0.004;

const CLICK_DELAY_MILLIS: u64 = 150;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
struct Params {
    centre_x: f64,
    centre_y: f64,
    max_iterations: u32,
    last_clicked: Instant,
}

impl Params {
    fn new() -> Self {
        Params {
            centre_x: 0.0,
            centre_y: 0.0,
            max_iterations: 75,
            last_clicked: Instant::now(),
        }
    }

    fn get_iterations(&self, mut z_re: f64, mut z_im: f64) -> f64 {
        let c_re = self.centre_x;
        let c_im = self.centre_y;
        let mut iterations = 0;
        while iterations < self.max_iterations && z_re * z_re + z_im * z_im <= 4.0 {
            let z_re_ = z_re;
            z_re = (z_re + z_im) * (z_re - z_im) + c_re;
            z_im = 2.0 * z_re_ * z_im + c_im;
            iterations += 1;
        }
        if iterations < self.max_iterations {
            iterations as f64 - ((z_re * z_re + z_im * z_im).log2() / 2.0).log2()
        } else {
            iterations as f64
        }
    }

    fn region_width(&self) -> f64 {
        4.0
    }

    fn region_height(&self) -> f64 {
        self.region_width() * HEIGHT_F / WIDTH_F
    }

    fn get_pixels(&self, palette: &ColorPalette) -> Vec<u32> {
        let start_time = Instant::now();
        let precomp1 = self.region_width() / WIDTH_F;
        let precomp2 = -self.region_width() / 2.0;
        let precomp3 = -self.region_height() / HEIGHT_F;
        let precomp4 = self.region_height() / 2.0;
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
        println!(
            "{:?} for: Max iters = {}, Centre = ({},{})",
            start_time.elapsed(),
            self.max_iterations,
            self.centre_x,
            self.centre_y,
        );
        output
    }

    fn update(&mut self, window: &Window) {
        // Translation
        let region_width = self.region_width();
        let region_height = self.region_height();
        if self.last_clicked.elapsed() > Duration::from_millis(CLICK_DELAY_MILLIS) {
            if window.get_mouse_down(MouseButton::Left) {
                if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Discard) {
                    let pos_x = mouse_x as f64 / WIDTH_F - 0.5;
                    let pos_y = -mouse_y as f64 / HEIGHT_F + 0.5;
                    self.centre_x = pos_x * region_width;
                    self.centre_y = pos_y * region_height;
                }
                self.last_clicked = Instant::now();
            }
        }
        if window.is_key_pressed(Key::Up, KeyRepeat::Yes) {
            self.centre_y += KEYS_TRANSLATION * region_height; // Pan up
        }
        if window.is_key_pressed(Key::Down, KeyRepeat::Yes) {
            self.centre_y -= KEYS_TRANSLATION * region_height; // Pan down
        }
        if window.is_key_pressed(Key::Right, KeyRepeat::Yes) {
            self.centre_x += KEYS_TRANSLATION * region_width; // Pan right
        }
        if window.is_key_pressed(Key::Left, KeyRepeat::Yes) {
            self.centre_x -= KEYS_TRANSLATION * region_width; // Pan left
        }

        // Changing max iterations
        if self.last_clicked.elapsed() > Duration::from_millis(CLICK_DELAY_MILLIS) {
            if window.get_mouse_down(MouseButton::Right) {
                self.max_iterations += MAX_ITERATION_JUMP;
                self.last_clicked = Instant::now();
            }
        }
        if window.is_key_pressed(Key::D, KeyRepeat::No) {
            self.max_iterations += MAX_ITERATION_JUMP;
        }
        if window.is_key_pressed(Key::A, KeyRepeat::No) {
            if self.max_iterations <= MAX_ITERATION_JUMP + MAX_ITERATION_LOWER_BOUND {
                self.max_iterations = MAX_ITERATION_LOWER_BOUND;
            } else {
                self.max_iterations -= MAX_ITERATION_JUMP;
            }
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
        let palette = ColorPalette::new(vec![
            (0., Rgb([0, 18, 25])),
            (0.1, Rgb([20, 33, 61])),
            (0.25, Rgb([252, 163, 17])),
            (0.6, Rgb([229, 229, 229])),
            (0.8, Rgb([255, 255, 255])),
            (1., Rgb([0, 0, 0])),
        ])
        .unwrap();
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
