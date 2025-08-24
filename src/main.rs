mod app;
mod fibonacci;
mod ui;
mod visualization;

use app::FibonacciApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Fibonacci Spiral Generator"),
        ..Default::default()
    };

    eframe::run_native(
        "Fibonacci Spiral Generator",
        options,
        Box::new(|_cc| Ok(Box::new(FibonacciApp::default()))),
    )
}

