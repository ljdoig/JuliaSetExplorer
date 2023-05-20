pub fn get_iterations(
    mut z_re: f64,
    mut z_im: f64,
    c_re: f64,
    c_im: f64,
    max_iterations: u32,
) -> f64 {
    let mut iterations = 0;
    while iterations < max_iterations && z_re * z_re + z_im * z_im <= 4.0 {
        let z_re_ = z_re;
        z_re = (z_re + z_im) * (z_re - z_im) + c_re;
        z_im = 2.0 * z_re_ * z_im + c_im;
        iterations += 1;
    }
    iterations as f64
}
