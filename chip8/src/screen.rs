/// The width of the CHIP-8 screen
pub const SCREEN_WIDTH_PIXELS: usize = 64;
/// The height of the CHIP-8 screen
pub const SCREEN_HEIGHT_PIXELS: usize = 32;

/// Represents the state of a CHIP-8 screen.
pub struct Chip8Screen {
    screen: [bool; SCREEN_WIDTH_PIXELS * SCREEN_HEIGHT_PIXELS],
}

impl Default for Chip8Screen {
    fn default() -> Self {
        Self {
            screen: [false; SCREEN_WIDTH_PIXELS * SCREEN_HEIGHT_PIXELS],
        }
    }
}

impl Chip8Screen {
    #[must_use]
    pub fn new() -> Chip8Screen {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.screen = [false; SCREEN_WIDTH_PIXELS * SCREEN_HEIGHT_PIXELS];
    }

    /// Draw a sprite to the screen, where `sprite` is an array of pixels for an
    /// 8xN sprite, where N is `sprite.len()`. Each bit is one pixel where `1`
    /// represents on and `0` represents off. Pixels are drawn left-to-right
    /// (most to least significant bit), top-to-bottom.
    ///
    /// See [CHIP‐8 Technical Reference](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#graphics)
    /// by Matthew Mikolay for more info.
    ///
    /// The length of `sprite` must be less than 8.
    ///
    /// `x` and `y` coordinates will be wrapped modulo the size of the screen in
    /// their respective directions.
    #[must_use]
    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        if sprite.is_empty() {
            // no pixels to draw, can't be any collisions
            return false;
        }

        // Whether an on pixel (value true) has been turned off (set to false)
        let mut collision = false;

        // Wrap coordinate
        let x = x % (SCREEN_WIDTH_PIXELS as u8);
        let y = y % (SCREEN_HEIGHT_PIXELS as u8);

        let sprite_width = 8;
        let sprite_height = sprite.len();
        // The dimensions of the actual area to draw, stopping at the border of
        // the screen.
        let area_width = if x as usize + sprite_width > SCREEN_WIDTH_PIXELS {
            (sprite_width - ((x as usize + sprite_width) % SCREEN_WIDTH_PIXELS)) as u8
        } else {
            sprite_width as u8
        };
        let area_height = if y as usize + sprite_height > SCREEN_HEIGHT_PIXELS {
            (sprite_width - ((y as usize + sprite_height) % SCREEN_HEIGHT_PIXELS)) as u8
        } else {
            sprite_height as u8
        };

        for ix in 0..area_width {
            for iy in 0..area_height {
                let pixel = self.get_pixel(x + ix, y + iy);
                let sprite_pixel = (sprite[iy as usize] & (0b1000_0000 >> ix)) != 0;
                if pixel && sprite_pixel {
                    collision = true;
                }
                self.set_pixel(x + ix, y + iy, pixel ^ sprite_pixel);
            }
        }

        collision
    }

    #[must_use]
    pub fn get_pixel(&self, x: u8, y: u8) -> bool {
        self.screen[calc_index(x, y)]
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, value: bool) {
        self.screen[calc_index(x, y)] = value
    }
}

#[must_use]
fn calc_index(x: u8, y: u8) -> usize {
    let x = x as usize;
    let y = y as usize;
    if x >= SCREEN_WIDTH_PIXELS || y >= SCREEN_HEIGHT_PIXELS {
        panic!("pixel coordinate is outside screen boundary")
    }
    y * SCREEN_WIDTH_PIXELS + x
}

#[cfg(test)]
mod test {
    use super::{calc_index, Chip8Screen};

    #[test]
    fn test_calc_index_bounds_checks_doesnt_panic() {
        _ = calc_index(0, 0);
        _ = calc_index(5, 30);
        _ = calc_index(63, 3);
        _ = calc_index(63, 31);
    }

    #[test]
    #[should_panic(expected = "pixel coordinate is outside screen boundary")]
    fn test_calc_index_bounds_checks_panics_x() {
        _ = calc_index(64, 3);
    }

    #[test]
    #[should_panic(expected = "pixel coordinate is outside screen boundary")]
    fn test_calc_index_bounds_checks_panics_y() {
        _ = calc_index(3, 32);
    }

    #[test]
    fn test_calc_index() {
        assert_eq!(calc_index(0, 0), 0);
        assert_eq!(calc_index(1, 0), 1);
        assert_eq!(calc_index(63, 0), 63);

        assert_eq!(calc_index(0, 1), 64);
        assert_eq!(calc_index(1, 1), 65);
        assert_eq!(calc_index(63, 1), 127);

        assert_eq!(calc_index(0, 5), 5 * 64);

        assert_eq!(calc_index(63, 31), (64 * 32) - 1);
    }

    #[test]
    fn test_draw_sprite_empty() {
        let expected_collision = false;

        let sprite: [u8; 0] = [];
        let mut screen = Chip8Screen::new();
        let collision = screen.draw_sprite(0, 0, &sprite);

        assert_eq!(collision, expected_collision);
    }

    #[test]
    fn test_draw_sprite_simple() {
        let expected_collision = false;
        let mut expected_screen = [false; 64 * 32];
        let offset = 64 + 1; // (1, 1); y * width + x
        expected_screen[offset] = true;
        expected_screen[offset + 1] = true;
        expected_screen[offset + 2] = false;
        expected_screen[offset + 3] = false;
        expected_screen[offset + 4] = true;
        expected_screen[offset + 5] = true;
        expected_screen[offset + 6] = false;
        expected_screen[offset + 7] = false;
        let offset = offset + 64; // move down one row
        expected_screen[offset] = false;
        expected_screen[offset + 1] = false;
        expected_screen[offset + 2] = true;
        expected_screen[offset + 3] = true;
        expected_screen[offset + 4] = false;
        expected_screen[offset + 5] = false;
        expected_screen[offset + 6] = true;
        expected_screen[offset + 7] = true;

        let sprite = [0b1100_1100, 0b0011_0011];
        let mut screen = Chip8Screen::new();
        let collision = screen.draw_sprite(1, 1, &sprite);

        assert_eq!(screen.screen, expected_screen);
        assert_eq!(collision, expected_collision);
    }

    #[test]
    fn test_draw_sprite_overlap() {
        let expected_collision1 = false;
        let expected_collision2 = true;
        // XX.   ...   XX.
        // XX. + .XX = X.X
        // ...   .XX   .XX
        let mut expected_screen = [false; 64 * 32];
        expected_screen[calc_index(0, 0)] = true;
        expected_screen[calc_index(1, 0)] = true;
        expected_screen[calc_index(0, 1)] = true;
        expected_screen[calc_index(2, 1)] = true;
        expected_screen[calc_index(1, 2)] = true;
        expected_screen[calc_index(2, 2)] = true;

        let sprite1 = [0b1100_0000, 0b1100_0000];
        let sprite2 = [0b0000_0000, 0b0110_0000, 0b0110_0000];
        let mut screen = Chip8Screen::new();
        let collision1 = screen.draw_sprite(0, 0, &sprite1);
        let collision2 = screen.draw_sprite(0, 0, &sprite2);

        assert_eq!(screen.screen, expected_screen);
        assert_eq!(collision1, expected_collision1);
        assert_eq!(collision2, expected_collision2);
    }

    #[test]
    fn test_draw_sprite_not_overlap() {
        let expected_collision1 = false;
        let expected_collision2 = false;
        // XXX   ...   XXX
        // X.X + .X. = XXX
        // XXX   ...   XXX
        let mut expected_screen = [false; 64 * 32];
        expected_screen[calc_index(0, 0)] = true;
        expected_screen[calc_index(1, 0)] = true;
        expected_screen[calc_index(2, 0)] = true;
        expected_screen[calc_index(0, 1)] = true;
        expected_screen[calc_index(1, 1)] = true;
        expected_screen[calc_index(2, 1)] = true;
        expected_screen[calc_index(0, 2)] = true;
        expected_screen[calc_index(1, 2)] = true;
        expected_screen[calc_index(2, 2)] = true;

        let sprite1 = [0b1110_0000, 0b1010_0000, 0b1110_0000];
        let sprite2 = [0b0000_0000, 0b0100_0000];
        let mut screen = Chip8Screen::new();
        let collision1 = screen.draw_sprite(0, 0, &sprite1);
        let collision2 = screen.draw_sprite(0, 0, &sprite2);

        assert_eq!(screen.screen, expected_screen);
        assert_eq!(collision1, expected_collision1);
        assert_eq!(collision2, expected_collision2);
    }

    #[test]
    fn test_draw_sprite_screen_edge() {
        let expected_collision = false;
        let mut expected_screen = [false; 64 * 32];
        expected_screen[calc_index(62, 30)] = true;
        expected_screen[calc_index(63, 30)] = true;
        expected_screen[calc_index(62, 31)] = true;
        expected_screen[calc_index(63, 31)] = true;

        let sprite = [
            0b1111_1111,
            0b1111_1111,
            0b1111_1111,
            0b1111_1111,
            0b1111_1111,
            0b1111_1111,
            0b1111_1111,
            0b1111_1111,
        ];
        let mut screen = Chip8Screen::new();
        let collision = screen.draw_sprite(62, 30, &sprite);

        assert_eq!(screen.screen, expected_screen);
        assert_eq!(collision, expected_collision);
    }
}
