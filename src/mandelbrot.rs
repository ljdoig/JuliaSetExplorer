use rayon::prelude::*;

use crate::josh_palette::ColorPalette;
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

fn rev_append(mut a: Vec<u32>) -> Vec<u32> {
    let mut b = a.clone();
    b.reverse();
    a.append(&mut b);
    a
}

fn get_coords(num_pixels: usize) -> Vec<(f64, f64)> {
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
            (x, y)
        })
        .collect()
}

pub fn julia_pixels(c_re: f64, c_im: f64, max_iterations: u32, palette: &ColorPalette) -> Vec<u32> {
    let first_half: Vec<u32> = get_coords(HEIGHT * WIDTH / 2)
        .into_par_iter()
        .map(|(x, y)| iterations(x, y, c_re, c_im, max_iterations))
        .map(|iterations| palette.value(iterations / max_iterations as f64))
        .collect();
    rev_append(first_half)
}

pub fn mandelbrot_pixels(max_iterations: u32, palette: &ColorPalette) -> Vec<u32> {
    get_coords(HEIGHT * WIDTH)
        .into_par_iter()
        .map(|(x, y)| iterations(0.0, 0.0, x, y, max_iterations))
        .map(|iterations| palette.value(iterations / max_iterations as f64))
        .collect()
}
