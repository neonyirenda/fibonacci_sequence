use eframe::egui;
use crate::fibonacci;
use crate::ui::{InputControls, ResultDisplay, SpiralVisualization, InstructionsPanel, validation};

/// Main application state
#[derive(Default)]
pub struct FibonacciApp {
    /// User input text
    input_text: String,
    /// Result message to display
    result_text: String,
    /// Generated Fibonacci sequence
    fibonacci_sequence: Vec<u64>,
    /// Current n value
    current_n: u32,
    /// UI components
    spiral_visualization: SpiralVisualization,
}

impl FibonacciApp {
    /// Create a new Fibonacci application
    pub fn new() -> Self {
        Self {
            input_text: String::new(),
            result_text: String::new(),
            fibonacci_sequence: Vec::new(),
            current_n: 0,
            spiral_visualization: SpiralVisualization::default(),
        }
    }

    /// Calculate Fibonacci sequence based on user input
    fn calculate_fibonacci(&mut self) {
        match validation::validate_input(&self.input_text) {
            Ok(n) => {
                let result = fibonacci::fib(n);
                self.result_text = format!("F({}) = {}", n, result);
                self.current_n = n;

                // Generate the sequence up to n using the more efficient iterative method
                self.fibonacci_sequence = fibonacci::generate_sequence_iterative(n);

                println!("Calculated F({}) = {}", n, result);
            }
            Err(error_msg) => {
                self.result_text = error_msg;
                self.fibonacci_sequence.clear();
                self.current_n = 0;
            }
        }
    }

    /// Reset the application state
    pub fn reset(&mut self) {
        self.input_text.clear();
        self.result_text.clear();
        self.fibonacci_sequence.clear();
        self.current_n = 0;
    }

    /// Get the current Fibonacci sequence
    pub fn get_sequence(&self) -> &[u64] {
        &self.fibonacci_sequence
    }

    /// Get the current n value
    pub fn get_current_n(&self) -> u32 {
        self.current_n
    }

    /// Check if there are results to display
    pub fn has_results(&self) -> bool {
        !self.fibonacci_sequence.is_empty() && self.current_n > 0
    }
}

impl eframe::App for FibonacciApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Main heading
            ui.heading("🧮 Fibonacci Spiral Generator");
            ui.add_space(20.0);

            // Input section
            let mut should_calculate = false;
            InputControls::render(ui, &mut self.input_text, &mut should_calculate);

            if should_calculate {
                self.calculate_fibonacci();
            }

            ui.add_space(10.0);

            // Result section
            ResultDisplay::render_result_text(ui, &self.result_text);

            // Fibonacci Spiral Visualization
            if self.has_results() {
                self.spiral_visualization.render(
                    ui,
                    &self.fibonacci_sequence,
                    self.current_n,
                );

                ui.add_space(10.0);

                // Show sequence numbers
                ResultDisplay::render_sequence(ui, &self.fibonacci_sequence);

                // Mathematical information
                InstructionsPanel::render_math_info(ui, &self.fibonacci_sequence, self.current_n);
            }

            // Instructions and tips
            InstructionsPanel::render(ui);

            // Add a reset button at the bottom
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("🔄 Reset").clicked() {
                    self.reset();
                }

                ui.add_space(20.0);

                // Show some statistics if we have results
                if self.has_results() {
                    ui.label(format!(
                        "📊 Displaying {} terms | Largest: F({}) = {}",
                        self.fibonacci_sequence.len(),
                        self.current_n,
                        self.fibonacci_sequence.last().unwrap_or(&0)
                    ));
                }
            });
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Storage functionality can be implemented later if needed
        // For now, we'll skip persistent storage
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
}

/// Application configuration and setup
pub mod config {
    use eframe::egui;

    /// Get the default native options for the application
    pub fn get_native_options() -> eframe::NativeOptions {
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([800.0, 700.0])
                .with_min_inner_size([600.0, 500.0])
                .with_title("Fibonacci Spiral Generator")
                .with_icon(load_icon()),
            ..Default::default()
        }
    }

    /// Load application icon (placeholder for now)
    fn load_icon() -> egui::IconData {
        // For now, return a default icon
        // In a real application, you would load an actual icon file
        egui::IconData {
            rgba: vec![255; 32 * 32 * 4], // 32x32 white square
            width: 32,
            height: 32,
        }
    }

    /// Application metadata
    pub struct AppInfo {
        pub name: &'static str,
        pub version: &'static str,
        pub author: &'static str,
        pub description: &'static str,
    }

    impl Default for AppInfo {
        fn default() -> Self {
            Self {
                name: "Fibonacci Spiral Generator",
                version: env!("CARGO_PKG_VERSION"),
                author: "Fibonacci Enthusiast",
                description: "A beautiful visualization of the Fibonacci sequence and golden spiral",
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = FibonacciApp::new();
        assert_eq!(app.get_current_n(), 0);
        assert!(app.get_sequence().is_empty());
        assert!(!app.has_results());
    }

    #[test]
    fn test_app_reset() {
        let mut app = FibonacciApp::new();
        app.input_text = "10".to_string();
        app.calculate_fibonacci();
        
        assert!(app.has_results());
        
        app.reset();
        assert!(!app.has_results());
        assert!(app.input_text.is_empty());
    }
}
