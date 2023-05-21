use image::Rgb;
use mandelbruhst_cli::palette::ColorPalette;
use rayon::prelude::*;
use std::marker::Sync;

use crate::*;

fn iterations(mut z_re: f64, mut z_im: f64, c_re: f64, c_im: f64, max_iterations: u32) -> f64 {
    let mut iterations = 0;
    while iterations < max_iterations && z_re * z_re + z_im * z_im <= 4.0 {
        let z_re_ = z_re;
        z_re = (z_re + z_im) * (z_re - z_im) + c_re;
        z_im = 2.0 * z_re_ * z_im + c_im;
        iterations += 1;
    }
    iterations as f64
}

fn rev_appended(mut a: Vec<u32>) -> Vec<u32> {
    let mut b = a.clone();
    b.reverse();
    a.append(&mut b);
    a
}

fn get_pixels(num_pixels: usize, get_pixel: impl Fn(f64, f64) -> u32 + Sync) -> Vec<u32> {
    let precomp1 = X_RANGE / (WIDTH_F - 1.0);
    let precomp2 = -X_RANGE / 2.0;
    let precomp3 = -Y_RANGE / (HEIGHT_F - 1.0);
    let precomp4 = Y_RANGE / 2.0;
    (0..num_pixels)
        .into_par_iter()
        .map(|i| {
            let col = i % WIDTH;
            let row = i / WIDTH;
            let x = col as f64 * precomp1 + precomp2;
            let y = row as f64 * precomp3 + precomp4;
            get_pixel(x, y)
        })
        .collect()
}

fn iterations_to_u32(iterations: f64, max_iterations: u32, palette: &ColorPalette) -> u32 {
    let Rgb([r, g, b]) = palette.value(iterations / max_iterations as f64);
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

pub fn julia_pixels(c_re: f64, c_im: f64, max_iterations: u32, palette: &ColorPalette) -> Vec<u32> {
    let get_pixel = |x, y| {
        let iterations = iterations(x, y, c_re, c_im, max_iterations);
        iterations_to_u32(iterations, max_iterations, palette)
    };
    let first_half = get_pixels(HEIGHT * WIDTH / 2, get_pixel);
    rev_appended(first_half)
}

pub fn mandelbrot_pixels(max_iterations: u32, palette: &ColorPalette) -> Vec<u32> {
    get_pixels(HEIGHT * WIDTH, |x, y| {
        let iterations = iterations(0.0, 0.0, x, y, max_iterations);
        iterations_to_u32(iterations, max_iterations, palette)
    })
}
