use minifb::{Key, Window};
/**
 * This is a bouncy box. No more, no less.
 * Bouncing through time in this ephimeral hologram of existence.
 * If someone finds this code in the future, this bouncy box
 * will keep bouncing decades after you are long gone and
 * forgotten. It's just a box. A bouncy box. And yet its
 * legacy is timeless. You are not. Somehow you think it's
 * OK to keep wasting your time reading this description. Curious
 * isn't it? You could be changing the world, but you are reading
 * a delusional description about a bouncy box demo.
 * How far are you willing to cotinue wasting your time on earth?
 * Probably longer than I could write. I should probably stop
 * procastrinating and get this demo done once and for all.
 * I was not that bored to keep writting anyway. What are
 * you doing with your life?
 */

pub struct BouncyBox {
    pub window_width: usize,
    pub window_height: usize,
    pub buffer: Vec<u32>,
    buffer_n: usize,
    pos_x: u32,
    pos_y: u32,
    step_x: i32,
    step_y: i32,
    cube_size: u32,
    area_size: usize,
}

impl BouncyBox {
    pub fn new(window_width: usize, window_height: usize) -> BouncyBox {
        let buffer_len: usize = (window_width * window_height) * 4 * 2;
        let buffer: Vec<u32> = vec![0; buffer_len];
        let pos_x: u32 = 0;
        let pos_y: u32 = 0;
        let step_x: i32 = 1;
        let step_y: i32 = 2;
        let cube_size = 50;
        BouncyBox {
            window_width,
            window_height,
            buffer,
            buffer_n: 0,
            pos_x,
            pos_y,
            step_x,
            step_y,
            cube_size,
            area_size: window_width * window_height, //store the real size of the screen
        }
    }

    fn flip_buffer_in_use(&mut self) {
        if self.buffer_n == 0 {
            self.buffer_n = 1;
        } else {
            self.buffer_n = 0;
        }
    }

    /**
     * return a slice in the buffer that has just been updated
     */
    pub fn get_buffer_to_print(&mut self) -> &[u32] {
        let start_offset = self.buffer_n * self.area_size as usize;
        &self.buffer[start_offset..start_offset + self.area_size as usize]
    }

    pub fn game_step(&mut self, window: &Window) {
        self.flip_buffer_in_use();
        let offset = self.buffer_n * self.area_size as usize;
        self.buffer
            .iter_mut()
            .skip(offset)
            .take(self.area_size as usize)
            .for_each(|value| *value = 0);
        for i in 0..self.cube_size {
            for j in 0..self.cube_size {
                let pixel = i + self.pos_x + (j + self.pos_y) * self.window_width as u32;
                #[cfg(feature = "web")]
                {
                    self.buffer[offset + pixel as usize] = 0xFF42F5AD; //ABGR
                }
                #[cfg(not(feature = "web"))]
                {
                    self.buffer[offset + pixel as usize] = 0xFFADF542; //ARGB
                }
            }
        }
        self.pos_x = (self.pos_x as i32 + self.step_x) as u32;
        self.pos_y = (self.pos_y as i32 + self.step_y) as u32;
        if window.is_key_down(Key::Up) {
            self.step_y = -2;
        } else if window.is_key_down(Key::Down) {
            self.step_y = 2;
        }
        if window.is_key_down(Key::Left) {
            self.step_x = -1;
        } else if window.is_key_down(Key::Right) {
            self.step_x = 1;
        }
        if self.pos_x == 0 || self.pos_x + self.cube_size >= self.window_width as u32 {
            self.step_x *= -1
        }
        if self.pos_y == 0 || self.pos_y + self.cube_size >= self.window_height as u32 {
            self.step_y *= -1
        }
    }
}
