pub mod josh_palette;
pub mod mandelbrot;

pub const WIDTH: usize = 1200;
pub const HEIGHT: usize = 800;
pub const WIDTH_F: f64 = WIDTH as f64;
pub const HEIGHT_F: f64 = HEIGHT as f64;
pub const X_RANGE: f64 = 4.5;
pub const Y_RANGE: f64 = X_RANGE * HEIGHT_F / WIDTH_F;
