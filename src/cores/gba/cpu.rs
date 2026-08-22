use super::bus::Bus;

/// Struct representing the GBA's ARM7TDMI CPU.
pub struct Cpu {
    /// General purpose registers R0-R12, Stack Pointer R13 (SP), Link Register R14 (LR), Program Counter R15 (PC)
    pub registers: [u32; 16],
    /// Current Program Status Register (CPSR)
    pub cpsr: u32,
    /// Saved Program Status Register (SPSR)
    pub spsr: u32,
    /// CPU Cycle Counter
    pub cycles: u64,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            cpsr: 0x1F, // System mode default
            spsr: 0,
            cycles: 0,
        }
    }

    /// Resets the CPU state.
    pub fn reset(&mut self) {
        // GBA ROM start execution address
        self.registers[15] = 0x08000000;
        self.cpsr = 0x1F;
        self.spsr = 0;
        self.cycles = 0;
    }

    /// Steps the CPU instruction execution by a single cycles/instruction block.
    pub fn step(&mut self, _bus: &mut Bus) {
        // Mock fetch, decode, execute instruction loop
        let _pc = self.registers[15];
        
        // Advance program counter (ARM mode instructions are 4 bytes)
        self.registers[15] = self.registers[15].wrapping_add(4);
        self.cycles += 4;
    }
}
