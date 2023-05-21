pub mod iterations;
pub mod simulation;
pub mod window_size {
    pub const WIDTH: usize = 1000;
    pub const HEIGHT: usize = 840;
    pub const WIDTH_F: f64 = WIDTH as f64;
    pub const HEIGHT_F: f64 = HEIGHT as f64;
    pub const Y_RANGE: f64 = 3.8;
    pub const X_RANGE: f64 = Y_RANGE / HEIGHT_F * WIDTH_F;
}
