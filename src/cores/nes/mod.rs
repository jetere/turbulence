use crate::core::Emulator;

pub struct NesEmulator {
    screen_buffer: Vec<u8>,
    width: usize,
    height: usize,
    frame_count: u32,
}

impl NesEmulator {
    pub fn new() -> Self {
        let width = 256;
        let height = 240;
        let screen_buffer = vec![0; width * height * 4]; // RGBA
        Self {
            screen_buffer,
            width,
            height,
            frame_count: 0,
        }
    }
}

impl Emulator for NesEmulator {
    fn load_rom(&mut self, _rom: &[u8]) -> Result<(), String> {
        // NES Rom loading stub
        Ok(())
    }

    fn step(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);

        // Render a moving pattern on a red background to demonstrate active execution
        let shift = (self.frame_count % self.width as u32) as usize;
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) * 4;
                
                // Red background
                self.screen_buffer[idx] = 220; // R
                
                // Dynamic green/blue scanning lines
                let is_line = (x + y + shift) % 40 < 4;
                if is_line {
                    self.screen_buffer[idx + 1] = 180; // G
                    self.screen_buffer[idx + 2] = 50;  // B
                } else {
                    self.screen_buffer[idx + 1] = 60;
                    self.screen_buffer[idx + 2] = 20;
                }
                
                self.screen_buffer[idx + 3] = 255; // Alpha
            }
        }
    }

    fn get_screen_buffer(&self) -> &[u8] {
        &self.screen_buffer
    }

    fn get_screen_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn handle_input(&mut self, _buttons: u8) {
        // Stub: Handle input (A, B, Select, Start, Up, Down, Left, Right)
    }

    fn reset(&mut self) {
        self.frame_count = 0;
        // Clear buffer
        for i in 0..self.screen_buffer.len() {
            self.screen_buffer[i] = 0;
        }
    }
}
