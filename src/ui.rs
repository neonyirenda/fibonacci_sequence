use eframe::egui::{self, Color32, Vec2};
use crate::visualization::{SpiralDrawer, utils};

/// Maximum supported Fibonacci number for optimal display
pub const MAX_FIBONACCI_N: u32 = 25;

/// UI component for input controls
pub struct InputControls;

impl InputControls {
    /// Render the input section of the UI
    pub fn render(
        ui: &mut egui::Ui,
        input_text: &mut String,
        on_calculate: &mut bool,
    ) {
        ui.horizontal(|ui| {
            ui.label("Enter a number (0-25):");
            ui.add_space(10.0);

            let response = ui.add(
                egui::TextEdit::singleline(input_text)
                    .desired_width(100.0)
                    .font(egui::TextStyle::Heading),
            );

            ui.add_space(10.0);

            let calculate_button = ui.add_sized([150.0, 30.0], egui::Button::new("🌀 Generate Spiral"));

            // Handle button click or Enter key
            if calculate_button.clicked()
                || (response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
            {
                *on_calculate = true;
            }
        });
    }
}

/// UI component for displaying results
pub struct ResultDisplay;

impl ResultDisplay {
    /// Render the result text
    pub fn render_result_text(ui: &mut egui::Ui, result_text: &str) {
        if !result_text.is_empty() {
            ui.label(
                egui::RichText::new(result_text)
                    .size(16.0)
                    .color(Color32::DARK_GREEN),
            );
            ui.add_space(10.0);
        }
    }

    /// Render the Fibonacci sequence display
    pub fn render_sequence(ui: &mut egui::Ui, fibonacci_sequence: &[u64]) {
        ui.group(|ui| {
            ui.label("Sequence:");
            ui.add_space(5.0);

            let sequence_text = utils::format_sequence(fibonacci_sequence);

            ui.label(
                egui::RichText::new(&sequence_text)
                    .font(egui::FontId::monospace(11.0)),
            );
        });
    }
}

/// UI component for the Fibonacci spiral visualization
pub struct SpiralVisualization {
    drawer: SpiralDrawer,
}

impl Default for SpiralVisualization {
    fn default() -> Self {
        Self {
            drawer: SpiralDrawer::default(),
        }
    }
}

impl SpiralVisualization {
    /// Create a new spiral visualization with custom drawer
    pub fn new(drawer: SpiralDrawer) -> Self {
        Self { drawer }
    }

    /// Render the spiral visualization
    pub fn render(
        &self,
        ui: &mut egui::Ui,
        fibonacci_sequence: &[u64],
        current_n: u32,
    ) {
        if fibonacci_sequence.is_empty() || current_n == 0 {
            return;
        }

        ui.group(|ui| {
            ui.label("Fibonacci Spiral:");
            ui.add_space(10.0);

            // Create a custom painting area
            let (rect, _response) = ui.allocate_exact_size(Vec2::new(600.0, 400.0), egui::Sense::hover());

            if ui.is_rect_visible(rect) {
                self.drawer.draw_spiral(ui, rect, fibonacci_sequence, current_n);
            }
        });
    }
}

/// UI component for displaying tips and instructions
pub struct InstructionsPanel;

impl InstructionsPanel {
    /// Render the instructions section
    pub fn render(ui: &mut egui::Ui) {
        ui.add_space(20.0);

        ui.separator();
        ui.small("💡 Tips:");
        ui.small("• Enter a number to see the Fibonacci spiral visualization");
        ui.small(&format!("• Numbers 0-{} are supported for optimal display", MAX_FIBONACCI_N));
        ui.small("• Each rectangle's size corresponds to its Fibonacci number");
        ui.small("• The spiral follows the golden ratio pattern");
    }

    /// Render additional mathematical information
    pub fn render_math_info(ui: &mut egui::Ui, fibonacci_sequence: &[u64], current_n: u32) {
        if fibonacci_sequence.len() >= 2 && current_n > 1 {
            ui.add_space(10.0);
            ui.group(|ui| {
                ui.label("Mathematical Properties:");
                ui.add_space(5.0);

                // Golden ratio approximation
                let last_fib = fibonacci_sequence[current_n as usize];
                let second_last_fib = fibonacci_sequence[(current_n - 1) as usize];
                let golden_ratio = utils::golden_ratio_approximation(last_fib, second_last_fib);

                ui.small(&format!(
                    "Golden ratio approximation: F({}) / F({}) ≈ {:.6}",
                    current_n,
                    current_n - 1,
                    golden_ratio
                ));

                ui.small(&format!(
                    "Actual golden ratio (φ): {:.6}",
                    (1.0 + 5.0_f64.sqrt()) / 2.0
                ));

                // Sum property
                let sum: u64 = fibonacci_sequence.iter().sum();
                ui.small(&format!("Sum of all terms: {}", sum));
            });
        }
    }
}

/// Validation utilities for user input
pub mod validation {
    use super::MAX_FIBONACCI_N;

    /// Validate and parse user input
    pub fn validate_input(input: &str) -> Result<u32, String> {
        match input.trim().parse::<u32>() {
            Ok(n) if n <= MAX_FIBONACCI_N => Ok(n),
            Ok(n) => Err(format!(
                "Number {} is too large! Please enter 0-{}",
                n, MAX_FIBONACCI_N
            )),
            Err(_) => Err("Please enter a valid number".to_string()),
        }
    }

    /// Check if input is valid without parsing
    pub fn is_valid_input(input: &str) -> bool {
        validate_input(input).is_ok()
    }

    /// Get a helpful error message for invalid input
    pub fn get_error_message(input: &str) -> String {
        validate_input(input).unwrap_err()
    }
}

/// Theme and styling utilities
pub mod theme {
    use eframe::egui::Color32;

    /// Application color scheme
    pub struct ColorScheme {
        pub primary: Color32,
        pub secondary: Color32,
        pub success: Color32,
        pub error: Color32,
        pub background: Color32,
    }

    impl Default for ColorScheme {
        fn default() -> Self {
            Self {
                primary: Color32::from_rgb(70, 130, 180),    // Steel blue
                secondary: Color32::from_rgb(255, 215, 0),   // Gold
                success: Color32::DARK_GREEN,
                error: Color32::DARK_RED,
                background: Color32::from_rgb(248, 248, 255), // Ghost white
            }
        }
    }

    /// Get the golden ratio color palette
    pub fn golden_palette() -> [Color32; 5] {
        [
            Color32::from_rgb(255, 215, 0),   // Gold
            Color32::from_rgb(255, 223, 0),   // Golden yellow
            Color32::from_rgb(255, 140, 0),   // Dark orange
            Color32::from_rgb(255, 165, 0),   // Orange
            Color32::from_rgb(255, 193, 37),  // Goldenrod
        ]
    }
}
