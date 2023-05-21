pub mod iterations;
pub mod simulation;
pub mod window_size {
    pub const INITIAL_WIDTH: usize = 1000;
    pub const INITIAL_HEIGHT: usize = 800;
    const MIN_X_RANGE: f64 = 4.2;
    const MIN_Y_RANGE: f64 = 3.6;
    pub fn x_y_ranges(width: usize, height: usize) -> (f64, f64) {
        let x_range = MIN_Y_RANGE / height as f64 * width as f64;
        if x_range < MIN_X_RANGE {
            let y_range = MIN_X_RANGE / width as f64 * height as f64;
            (MIN_X_RANGE, y_range)
        } else {
            (x_range, MIN_Y_RANGE)
        }
    }
}
