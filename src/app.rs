use crate::core::Emulator;
use crate::cores::{create_emulator, SystemType};
use eframe::egui;

pub struct TurbulenceApp {
    selected_system: SystemType,
    emulator: Option<Box<dyn Emulator>>,
    rom_path: String,
    status_message: String,
    is_running: bool,
    texture: Option<egui::TextureHandle>,
}

impl TurbulenceApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            selected_system: SystemType::NES,
            emulator: None,
            rom_path: String::new(),
            status_message: "Ready".to_string(),
            is_running: false,
            texture: None,
        }
    }
}

impl eframe::App for TurbulenceApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Turbulence Emulator Hub");
                ui.separator();
                
                ui.label("System:");
                ui.selectable_value(&mut self.selected_system, SystemType::NES, "NES");
                ui.selectable_value(&mut self.selected_system, SystemType::GBA, "GBA");
            });
        });

        egui::SidePanel::left("left_panel").width_range(200.0..=300.0).show(ctx, |ui| {
            ui.heading("Controls");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("ROM Path:");
                ui.text_edit_singleline(&mut self.rom_path);
            });

            if ui.button("Load Mock ROM").clicked() {
                let mut emu = create_emulator(self.selected_system);
                let mock_rom = vec![0; 512]; // dummy rom content
                match emu.load_rom(&mock_rom) {
                    Ok(()) => {
                        self.emulator = Some(emu);
                        self.status_message = format!(
                            "Loaded mock ROM for {:?}",
                            self.selected_system
                        );
                        self.is_running = true;
                    }
                    Err(e) => {
                        self.status_message = format!("Error loading ROM: {}", e);
                    }
                }
            }

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button(if self.is_running { "Pause" } else { "Play" }).clicked() {
                    if self.emulator.is_some() {
                        self.is_running = !self.is_running;
                        self.status_message = if self.is_running {
                            "Running".to_string()
                        } else {
                            "Paused".to_string()
                        };
                    }
                }

                if ui.button("Reset").clicked() {
                    if let Some(ref mut emu) = self.emulator {
                        emu.reset();
                        self.status_message = "Emulator Reset".to_string();
                    }
                }
            });

            ui.separator();
            ui.label(format!("Status: {}", self.status_message));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.is_running {
                if let Some(ref mut emu) = self.emulator {
                    emu.step();

                    let (w, h) = emu.get_screen_size();
                    let pixels = emu.get_screen_buffer();

                    let color_image = egui::ColorImage::from_rgba_unmultiplied(
                        [w, h],
                        pixels,
                    );

                    self.texture = Some(ctx.load_texture(
                        "emulator-screen",
                        color_image,
                        egui::TextureOptions::default(),
                    ));

                    // Keep updating the frame
                    ctx.request_repaint();
                }
            }

            if let Some(ref texture) = self.texture {
                // Scale texture up so it is easily visible
                let size = egui::vec2(texture.size()[0] as f32 * 2.0, texture.size()[1] as f32 * 2.0);
                ui.image((texture.id(), size));
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("No ROM Loaded. Select a system and load a mock ROM to start emulation.");
                });
            }
        });
    }
}
