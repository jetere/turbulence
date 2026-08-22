/// Struct representing the GBA's System memory Bus.
pub struct Bus {
    /// BIOS ROM (16 KB)
    pub bios: Vec<u8>,
    /// Board Work RAM (256 KB External EWRAM)
    pub ewram: Vec<u8>,
    /// Chip Work RAM (32 KB Internal IWRAM)
    pub iwram: Vec<u8>,
    /// Video RAM (96 KB VRAM)
    pub vram: Vec<u8>,
    /// GamePak Cartridge ROM (Max 32 MB)
    pub rom: Vec<u8>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            bios: vec![0; 16 * 1024],
            ewram: vec![0; 256 * 1024],
            iwram: vec![0; 32 * 1024],
            vram: vec![0; 96 * 1024],
            rom: Vec::new(),
        }
    }

    /// Loads a GBA ROM into the cartridge ROM memory bank.
    pub fn load_rom(&mut self, rom: &[u8]) {
        self.rom = rom.to_vec();
    }

    /// Read 8-bit value from GBA memory map.
    pub fn read8(&self, addr: u32) -> u8 {
        match addr {
            // BIOS region
            0x00000000..=0x00003FFF => self.bios[addr as usize],
            // External WRAM (EWRAM)
            0x02000000..=0x0203FFFF => self.ewram[(addr - 0x02000000) as usize],
            // Internal WRAM (IWRAM)
            0x03000000..=0x03007FFF => self.iwram[(addr - 0x03000000) as usize],
            // VRAM region
            0x06000000..=0x06017FFF => self.vram[(addr - 0x06000000) as usize],
            // ROM region
            0x08000000..=0x09FFFFFF => {
                let offset = (addr - 0x08000000) as usize;
                if offset < self.rom.len() {
                    self.rom[offset]
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    /// Write 8-bit value to GBA memory map.
    pub fn write8(&mut self, addr: u32, val: u8) {
        match addr {
            // External WRAM (EWRAM)
            0x02000000..=0x0203FFFF => self.ewram[(addr - 0x02000000) as usize] = val,
            // Internal WRAM (IWRAM)
            0x03000000..=0x03007FFF => self.iwram[(addr - 0x03000000) as usize] = val,
            // VRAM region
            0x06000000..=0x06017FFF => self.vram[(addr - 0x06000000) as usize] = val,
            _ => {}
        }
    }
}
