/// Universal Emulator trait that defines the interface for all plug-and-play cores.
pub trait Emulator {
    /// Loads a ROM from a byte slice. Returns `Ok(())` on success, or an error message.
    fn load_rom(&mut self, rom: &[u8]) -> Result<(), String>;

    /// Steps the emulation by one frame or system cycles.
    fn step(&mut self);

    /// Returns a reference to the active screen buffer formatted as RGBA bytes.
    fn get_screen_buffer(&self) -> &[u8];

    /// Returns the resolution (width, height) of the emulator display.
    fn get_screen_size(&self) -> (usize, usize);

    /// Sends input controller state mask (e.g. 8-bit button state) to the emulator.
    fn handle_input(&mut self, buttons: u8);

    /// Resets the emulation core state.
    fn reset(&mut self);
}

/// Metadata describing a specific emulator core system.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoreInfo {
    pub name: String,
    pub system: String,
    pub file_extensions: Vec<String>,
}
