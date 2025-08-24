use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};

/// Colors for the Fibonacci spiral rectangles (golden/yellow theme)
pub const FIBONACCI_COLORS: [Color32; 8] = [
    Color32::from_rgb(255, 255, 200), // Light yellow
    Color32::from_rgb(255, 245, 180), // Cream
    Color32::from_rgb(255, 235, 160), // Light gold
    Color32::from_rgb(255, 225, 140), // Gold
    Color32::from_rgb(255, 215, 120), // Darker gold
    Color32::from_rgb(255, 205, 100), // Orange-gold
    Color32::from_rgb(255, 195, 80),  // Deep gold
    Color32::from_rgb(255, 185, 60),  // Amber
];

/// Represents a rectangle in the Fibonacci spiral
#[derive(Debug, Clone)]
pub struct FibonacciRectangle {
    pub rect: Rect,
    pub value: u64,
    pub index: usize,
}

/// Spiral drawer for Fibonacci visualization
pub struct SpiralDrawer {
    pub grid_size: f32,
    pub grid_color: Color32,
}

impl Default for SpiralDrawer {
    fn default() -> Self {
        Self {
            grid_size: 10.0,
            grid_color: Color32::from_rgba_unmultiplied(200, 200, 200, 100),
        }
    }
}

impl SpiralDrawer {
    /// Create a new spiral drawer with custom grid settings
    pub fn new(grid_size: f32, grid_color: Color32) -> Self {
        Self {
            grid_size,
            grid_color,
        }
    }

    /// Draw the complete Fibonacci spiral
    pub fn draw_spiral(
        &self,
        ui: &mut egui::Ui,
        rect: Rect,
        fibonacci_sequence: &[u64],
        current_n: u32,
    ) {
        let painter = ui.painter();

        if fibonacci_sequence.len() < 3 {
            return;
        }

        // Draw grid background
        self.draw_grid(rect, &painter);

        // Calculate and draw rectangles
        let rectangles = self.calculate_spiral_rectangles(rect, fibonacci_sequence);
        self.draw_rectangles(&painter, &rectangles);

        // Draw title
        self.draw_title(&painter, rect, current_n);
    }

    /// Calculate the positions and sizes of all rectangles in the spiral
    fn calculate_spiral_rectangles(&self, rect: Rect, fibonacci_sequence: &[u64]) -> Vec<FibonacciRectangle> {
        let mut rectangles = Vec::new();

        if fibonacci_sequence.len() < 3 {
            return rectangles;
        }

        // Calculate scale factor to ensure smaller numbers are visible
        // Use a more conservative scaling to prevent overlapping
        let max_fib = fibonacci_sequence.iter().max().unwrap_or(&1);
        let available_size = rect.width().min(rect.height()) * 0.5;

        // Base unit size - ensure even F(1)=1 is readable but not too large
        let min_unit_size: f32 = 20.0;
        let max_unit_size = available_size / (*max_fib as f32).sqrt();
        let unit = min_unit_size.max(max_unit_size.min(35.0));

        // Build all rectangles first to calculate total bounds
        let mut temp_rectangles = Vec::new();

        // Start with the first two 1x1 squares at origin (0,0)
        // Use the same square root scaling for consistency but more conservative
        let first_size = (1.0_f32).sqrt() * unit * 1.2;
        let rect1 = Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::splat(first_size));
        temp_rectangles.push((rect1, 1, 0));

        let rect2 = Rect::from_min_size(Pos2::new(first_size, 0.0), Vec2::splat(first_size));
        temp_rectangles.push((rect2, 1, 1));

        // Build the spiral outward from origin
        let mut current_width = first_size * 2.0;
        let mut current_height = first_size;
        let mut base_x = 0.0;
        let mut base_y = 0.0;

        for i in 2..fibonacci_sequence.len().min(12) {
            let fib_val = fibonacci_sequence[i];
            // Use square root scaling to make smaller numbers more visible
            // while still maintaining proportional relationships
            let size = (fib_val as f32).sqrt() * unit * 1.2;
            let direction_idx = (i - 2) % 4;

            let (new_rect, new_base_x, new_base_y, new_width, new_height) = match direction_idx {
                0 => {
                    // down
                    let r = Rect::from_min_size(
                        Pos2::new(base_x, base_y + current_height),
                        Vec2::new(current_width, size),
                    );
                    (r, base_x, base_y, current_width, current_height + size)
                }
                1 => {
                    // left
                    let r = Rect::from_min_size(
                        Pos2::new(base_x - size, base_y),
                        Vec2::new(size, current_height),
                    );
                    (r, base_x - size, base_y, current_width + size, current_height)
                }
                2 => {
                    // up
                    let r = Rect::from_min_size(
                        Pos2::new(base_x, base_y - size),
                        Vec2::new(current_width, size),
                    );
                    (r, base_x, base_y - size, current_width, current_height + size)
                }
                3 => {
                    // right
                    let r = Rect::from_min_size(
                        Pos2::new(base_x + current_width, base_y),
                        Vec2::new(size, current_height),
                    );
                    (r, base_x, base_y, current_width + size, current_height)
                }
                _ => continue,
            };

            temp_rectangles.push((new_rect, fib_val, i));
            base_x = new_base_x;
            base_y = new_base_y;
            current_width = new_width;
            current_height = new_height;
        }

        // Calculate the bounding box of all rectangles
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for (temp_rect, _, _) in &temp_rectangles {
            min_x = min_x.min(temp_rect.min.x);
            min_y = min_y.min(temp_rect.min.y);
            max_x = max_x.max(temp_rect.max.x);
            max_y = max_y.max(temp_rect.max.y);
        }

        let spiral_width = max_x - min_x;
        let spiral_height = max_y - min_y;

        // Calculate offset to center the spiral in the available area
        let center_x = rect.center().x;
        let center_y = rect.center().y;
        let offset_x = center_x - (min_x + spiral_width / 2.0);
        let offset_y = center_y - (min_y + spiral_height / 2.0);

        // Apply the centering offset to all rectangles
        for (temp_rect, fib_val, index) in temp_rectangles {
            let centered_rect = Rect::from_min_size(
                Pos2::new(temp_rect.min.x + offset_x, temp_rect.min.y + offset_y),
                temp_rect.size(),
            );

            rectangles.push(FibonacciRectangle {
                rect: centered_rect,
                value: fib_val,
                index,
            });
        }

        rectangles
    }

    /// Draw all rectangles with their numbers
    fn draw_rectangles(&self, painter: &egui::Painter, rectangles: &[FibonacciRectangle]) {
        for (i, fib_rect) in rectangles.iter().enumerate() {
            let color = FIBONACCI_COLORS[i % FIBONACCI_COLORS.len()];

            // Draw rectangle
            painter.rect_filled(fib_rect.rect, 2.0, color);
            painter.rect_stroke(fib_rect.rect, 2.0, Stroke::new(2.0, Color32::BLACK));

            // Calculate appropriate text size based on rectangle dimensions
            let min_dimension = fib_rect.rect.width().min(fib_rect.rect.height());
            let text_size = if min_dimension > 80.0 {
                20.0
            } else if min_dimension > 60.0 {
                16.0
            } else if min_dimension > 40.0 {
                14.0
            } else if min_dimension > 25.0 {
                12.0
            } else {
                10.0
            };

            // Use a bold font for better readability
            let font_id = egui::FontId {
                size: text_size,
                family: egui::FontFamily::Proportional,
            };

            let text = fib_rect.value.to_string();

            // Check if text will fit in the rectangle
            let text_galley = painter.layout_no_wrap(
                text.clone(),
                font_id.clone(),
                Color32::BLACK,
            );

            // Only draw text if it fits comfortably in the rectangle
            let text_fits = text_galley.size().x < (fib_rect.rect.width() - 4.0) &&
                           text_galley.size().y < (fib_rect.rect.height() - 4.0);

            if text_fits {
                let text_pos = fib_rect.rect.center();

                // Draw text with a subtle shadow/outline for better contrast
                // Draw shadow (slightly offset)
                painter.text(
                    text_pos + Vec2::new(1.0, 1.0),
                    egui::Align2::CENTER_CENTER,
                    &text,
                    font_id.clone(),
                    Color32::from_rgba_unmultiplied(0, 0, 0, 100), // Semi-transparent black shadow
                );

                // Draw main text
                painter.text(
                    text_pos,
                    egui::Align2::CENTER_CENTER,
                    &text,
                    font_id,
                    Color32::BLACK,
                );
            }
        }
    }

    /// Draw the background grid
    fn draw_grid(&self, rect: Rect, painter: &egui::Painter) {
        // Vertical lines
        let mut x = rect.min.x;
        while x <= rect.max.x {
            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                Stroke::new(0.5, self.grid_color),
            );
            x += self.grid_size;
        }

        // Horizontal lines
        let mut y = rect.min.y;
        while y <= rect.max.y {
            painter.line_segment(
                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                Stroke::new(0.5, self.grid_color),
            );
            y += self.grid_size;
        }
    }

    /// Draw the title
    fn draw_title(&self, painter: &egui::Painter, rect: Rect, current_n: u32) {
        let title_text = format!("Fibonacci Spiral (n = {})", current_n);
        let title_pos = Pos2::new(rect.min.x + 15.0, rect.min.y + 15.0);

        // Draw title background for better readability
        let text_size = 18.0;
        let font_id = egui::FontId::proportional(text_size);

        // Estimate text bounds for background
        let text_galley = painter.layout_no_wrap(
            title_text.clone(),
            font_id.clone(),
            Color32::BLACK,
        );

        let background_rect = Rect::from_min_size(
            title_pos - Vec2::new(5.0, 2.0),
            text_galley.size() + Vec2::new(10.0, 4.0),
        );

        // Draw semi-transparent background
        painter.rect_filled(
            background_rect,
            4.0,
            Color32::from_rgba_unmultiplied(255, 255, 255, 200),
        );
        painter.rect_stroke(
            background_rect,
            4.0,
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, 100)),
        );

        // Draw title text
        painter.text(
            title_pos,
            egui::Align2::LEFT_TOP,
            title_text,
            font_id,
            Color32::BLACK,
        );
    }
}

/// Utility functions for visualization
pub mod utils {
    /// Format a Fibonacci sequence for display
    pub fn format_sequence(sequence: &[u64]) -> String {
        sequence
            .iter()
            .enumerate()
            .map(|(i, &val)| format!("F({}) = {}", i, val))
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Calculate the golden ratio approximation from two consecutive Fibonacci numbers
    pub fn golden_ratio_approximation(fib_n: u64, fib_n_minus_1: u64) -> f64 {
        if fib_n_minus_1 == 0 {
            return 0.0;
        }
        fib_n as f64 / fib_n_minus_1 as f64
    }

    /// Get a description of the Fibonacci spiral
    pub fn get_spiral_description(n: u32) -> String {
        format!(
            "The Fibonacci spiral is created by drawing quarter-circle arcs \
            connecting the opposite corners of squares in the Fibonacci tiling. \
            For n = {}, the spiral contains {} rectangles, with the largest \
            rectangle having a Fibonacci number of F({}) = {}.",
            n,
            n + 1,
            n,
            crate::fibonacci::fib(n)
        )
    }
}
