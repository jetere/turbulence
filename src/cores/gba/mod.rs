pub mod bus;
pub mod cpu;

use crate::core::Emulator;
use bus::Bus;
use cpu::Cpu;

pub struct GbaEmulator {
    cpu: Cpu,
    bus: Bus,
    screen_buffer: Vec<u8>,
    width: usize,
    height: usize,
    frame_count: u32,
}

impl GbaEmulator {
    pub fn new() -> Self {
        let width = 240;
        let height = 160;
        let screen_buffer = vec![0; width * height * 4]; // RGBA
        Self {
            cpu: Cpu::new(),
            bus: Bus::new(),
            screen_buffer,
            width,
            height,
            frame_count: 0,
        }
    }
}

impl Emulator for GbaEmulator {
    fn load_rom(&mut self, rom: &[u8]) -> Result<(), String> {
        self.bus.load_rom(rom);
        self.cpu.reset();
        Ok(())
    }

    fn step(&mut self) {
        self.cpu.step(&mut self.bus);
        self.frame_count = self.frame_count.wrapping_add(1);

        // Render a moving blue/indigo grid to demonstrate active execution
        let offset = (self.frame_count / 2) as usize;
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) * 4;

                // Blue/indigo background
                self.screen_buffer[idx] = 40;   // R
                self.screen_buffer[idx + 1] = 60;  // G
                self.screen_buffer[idx + 2] = 220; // B
                
                // Scrolling grid check
                let is_grid_x = (x + offset) % 30 < 2;
                let is_grid_y = (y + offset) % 30 < 2;
                if is_grid_x || is_grid_y {
                    self.screen_buffer[idx] = 180;
                    self.screen_buffer[idx + 1] = 200;
                    self.screen_buffer[idx + 2] = 255;
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
        // Stub: Handle GBA button input
    }

    fn reset(&mut self) {
        self.cpu.reset();
        self.frame_count = 0;
        for i in 0..self.screen_buffer.len() {
            self.screen_buffer[i] = 0;
        }
    }
}
