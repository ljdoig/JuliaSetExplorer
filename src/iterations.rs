use crate::simulation::JuliaParams;
use crate::window_size::*;
use image::Rgb;
use mandelbruhst_cli::palette::ColorPalette;
use rayon::prelude::*;
use std::marker::Sync;

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

fn get_pixels(
    num_pixels: usize,
    width: usize,
    height: usize,
    vertical_offset: bool,
    get_pixel_from_coords: impl Fn(f64, f64) -> u32 + Sync,
) -> Vec<u32> {
    let (x_range, y_range) = x_y_ranges(width, height);
    let x_per_pixel = x_range / (width as f64 - 1.0);
    let min_x_range = -x_range / 2.0;
    let y_per_pixel = -y_range / (height as f64 - 1.0);
    let min_y_range = y_range / 2.0 + if vertical_offset { 0.001 } else { 0.0 };
    (0..num_pixels)
        .into_par_iter()
        .map(|i| {
            let col = i % width;
            let row = i / width;
            let x = col as f64 * x_per_pixel + min_x_range;
            let y = row as f64 * y_per_pixel + min_y_range;
            get_pixel_from_coords(x, y)
        })
        .collect()
}

fn iterations_to_u32(iterations: f64, max_iterations: u32, palette: &ColorPalette) -> u32 {
    let Rgb([r, g, b]) = palette.value(iterations / max_iterations as f64);
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

pub fn julia_pixels(
    params: &JuliaParams,
    palette: &ColorPalette,
    width: usize,
    height: usize,
) -> Vec<u32> {
    let get_pixel_from_coords = |x, y| {
        let iterations = iterations(x, y, params.c_re, params.c_im, params.max_iterations);
        iterations_to_u32(iterations, params.max_iterations, palette)
    };
    let mut first_half = get_pixels(
        (width * height + 1) / 2,
        width,
        height,
        false,
        get_pixel_from_coords,
    );
    // Save calculating the second half - just duplicate
    let mut second_half = first_half.clone();
    // If there is a centre pixel we shoudn't duplicate it
    if width % 2 == 1 && height % 2 == 1 {
        second_half.pop();
    }
    second_half.reverse();
    first_half.append(&mut second_half);
    first_half
}

pub fn mandelbrot_pixels(
    max_iterations: u32,
    palette: &ColorPalette,
    width: usize,
    height: usize,
) -> Vec<u32> {
    // Avoid a filled white line along the negative x-axis
    let vertical_offset = height % 2 == 1;
    get_pixels(width * height, width, height, vertical_offset, |x, y| {
        let iterations = iterations(0.0, 0.0, x, y, max_iterations);
        iterations_to_u32(iterations, max_iterations, palette)
    })
}
