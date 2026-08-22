mod app;
mod core;
mod cores;

use app::TurbulenceApp;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([720.0, 520.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Turbulence - Multi-System Emulator Hub",
        native_options,
        Box::new(|cc| {
            // Instantiates application state
            Ok(Box::new(TurbulenceApp::new(cc)))
        }),
    )
}
