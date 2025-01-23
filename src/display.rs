#[derive(Debug)]
pub struct C8Display {
    display_array: [u8; 64 * 32],
}

impl C8Display {
    pub fn new() -> Self {
        C8Display {
            display_array: [0; 64 * 32],
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, value: u8) {
        self.display_array[y * 64 + x] = value;
    }

    pub fn clear_screen(&mut self) {
        self.display_array = [0; 64 * 32];
    }
    pub fn get_display(&self) -> &[u8; 64 * 32] {
        &self.display_array
    }
}

#[cfg(test)]
mod test {
    use crate::C8Display;
    #[test]
    fn c8display_pixelwrite() {
        let mut display = C8Display::new();
        display.write_pixel(5, 7, 1);

        let value = display.display_array[453];
        assert_eq!(value, 1);
    }

    #[test]
    fn c8display_clearscreen() {
        let mut display = C8Display::new();
        display.display_array = [1; 64 * 32];

        display.clear_screen();
        for i in display.display_array {
            assert_eq!(i, 0);
        }
    }
}
