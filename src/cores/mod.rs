pub mod gba;
pub mod nes;

use crate::core::Emulator;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SystemType {
    NES,
    GBA,
}

/// Creates a new emulator instance based on the system type.
pub fn create_emulator(system: SystemType) -> Box<dyn Emulator> {
    match system {
        SystemType::NES => Box::new(nes::NesEmulator::new()),
        SystemType::GBA => Box::new(gba::GbaEmulator::new()),
    }
}
