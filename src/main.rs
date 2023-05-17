use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use rayon::prelude::*;
use std::time::{Duration, Instant};

const WIDTH: usize = 900;
const HEIGHT: usize = 600;
const WIDTH_F: f64 = WIDTH as f64;
const HEIGHT_F: f64 = HEIGHT as f64;

const MAX_ITERATION_JUMP: u32 = 250;
const MAX_ITERATION_LOWER_BOUND: u32 = 100;
const ZOOM_FACTOR: f64 = 1.1;

const CLICK_DELAY_MILLIS: u64 = 150;

const COLOURS: [(u8, u8, u8); 4] = [
    (10, 147, 150),  // Dark blue
    (0, 18, 25),     // Blackish
    (174, 32, 18),   // Orange
    (233, 216, 166), // Light yellow
];
const NUM_COLOURS: usize = COLOURS.len();

fn linear_interpolation((r1, g1, b1): (u8, u8, u8), (r2, g2, b2): (u8, u8, u8), t: f64) -> u32 {
    let red = interpolate_component(r1, r2, t);
    let green = interpolate_component(g1, g2, t);
    let blue = interpolate_component(b1, b2, t);
    red << 16 | green << 8 | blue
}

fn interpolate_component(c1: u8, c2: u8, t: f64) -> u32 {
    let interpolated = (c1 as f64) * (1.0 - t) + (c2 as f64) * t;
    interpolated.round() as u32
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
struct Params {
    zoom: f64,
    centre_x: f64,
    centre_y: f64,
    max_iterations: u32,
    scroll: f32,
    last_clicked: Instant,
}

impl Params {
    fn new() -> Self {
        Params {
            zoom: 1.0,
            centre_x: -1.0,
            centre_y: 0.0,
            max_iterations: 100,
            scroll: 0.0,
            last_clicked: Instant::now(),
        }
    }

    fn get_iterations(&self, c_re: f64, c_im: f64) -> f64 {
        let mut z_re = 0.0;
        let mut z_im = 0.0;
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

    fn colour_iterations(&self, mut iterations: f64) -> u32 {
        iterations *= 0.03;
        let iter_whole = iterations.floor();
        let iter_frac = iterations - iter_whole;
        let iter_whole = iter_whole as usize;

        let colour = COLOURS[iter_whole % NUM_COLOURS];
        let next_colour = COLOURS[(iter_whole + 1) % NUM_COLOURS];

        linear_interpolation(colour, next_colour, iter_frac)
    }

    fn region_width(&self) -> f64 {
        4.0 / self.zoom
    }

    fn region_height(&self) -> f64 {
        self.region_width() * HEIGHT_F / WIDTH_F
    }

    fn get_pixels(&self) -> Vec<u32> {
        let start_time = Instant::now();
        let precomp1 = self.region_width() / WIDTH_F;
        let precomp2 = -self.region_width() / 2.0 + self.centre_x;
        let precomp3 = -self.region_height() / HEIGHT_F;
        let precomp4 = self.region_height() / 2.0 + self.centre_y;
        let output = (0..HEIGHT * WIDTH)
            .into_par_iter()
            .map(|i| {
                let col = i % WIDTH;
                let row = i / WIDTH;
                let x = col as f64 * precomp1 + precomp2;
                let y = row as f64 * precomp3 + precomp4;
                self.get_iterations(x, y)
            })
            .map(|iterations| self.colour_iterations(iterations))
            .collect();
        println!(
            "{:?} for: Max iters = {}, Zoom = {}, Centre = ({},{})",
            start_time.elapsed(),
            self.max_iterations,
            self.zoom,
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
                    let change_x = mouse_x as f64 / WIDTH_F - 0.5;
                    let change_y = -mouse_y as f64 / HEIGHT_F + 0.5;
                    self.centre_x += change_x * region_width;
                    self.centre_y += change_y * region_height;
                }
                self.last_clicked = Instant::now();
            }
        }
        if window.is_key_pressed(Key::Up, KeyRepeat::Yes) {
            self.centre_y += 0.1 * region_height; // Pan up
        }
        if window.is_key_pressed(Key::Down, KeyRepeat::Yes) {
            self.centre_y -= 0.1 * region_height; // Pan down
        }
        if window.is_key_pressed(Key::Right, KeyRepeat::Yes) {
            self.centre_x += 0.1 * region_width; // Pan right
        }
        if window.is_key_pressed(Key::Left, KeyRepeat::Yes) {
            self.centre_x -= 0.1 * region_width; // Pan left
        }

        // Zooming
        if window.is_key_pressed(Key::W, KeyRepeat::Yes) {
            self.zoom *= 2.0;
        }
        if window.is_key_pressed(Key::S, KeyRepeat::Yes) {
            self.zoom *= 0.5;
        }
        if let Some((_, scroll)) = window.get_scroll_wheel() {
            self.zoom *= ZOOM_FACTOR.powf(scroll as f64);
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
}

impl State {
    fn new() -> Self {
        let params = Params::new();
        let pixels = params.get_pixels();
        State { params, pixels }
    }

    fn update(&mut self, window: &Window) {
        let old_params = self.params.clone();

        self.params.update(window);

        // Only update pixels if the parameters have been changed
        if self.params != old_params {
            self.pixels = self.params.get_pixels();
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Mandelbrot Explorer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

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
