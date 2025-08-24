# Fibonacci Sequence Generator

A cross-platform graphical application written in Rust using egui that generates and visualizes Fibonacci sequences.

## Features

- **Cross-Platform GUI**: Modern, responsive interface built with egui (works on Windows, macOS, Linux)
- **Input Validation**: Accepts numbers from 0 to 40 (to prevent excessive computation)
- **Dual Display**: Shows both the specific Fibonacci number and the complete sequence
- **Visual Representation**: ASCII bar chart visualization of the sequence
- **Interactive Elements**: Expandable sequence view, scrollable results
- **Keyboard Support**: Press Enter in the input field to calculate
- **Modern UI**: Clean design with proper visual feedback

## Installation

### Prerequisites

- Rust (latest stable version)
- No additional system dependencies required! (egui handles everything)

### Building

1. Clone or download this repository
2. Navigate to the project directory
3. Build and run:

```bash
cargo run
```

## Usage

1. Launch the application using `cargo run`
2. Enter a number between 0 and 40 in the input field
3. Click "Generate Fibonacci" or press Enter
4. View the results:
   - **Single Result**: Shows F(n) = result
   - **Complete Sequence**: Displays all Fibonacci numbers from F(0) to F(n)
   - **Visual Chart**: ASCII bar representation of the sequence values

## Examples

- Input: `10` → Shows F(10) = 55 and the complete sequence F(0) through F(10)
- Input: `20` → Shows F(20) = 6765 with visual bars scaled appropriately

## Technical Details

- **Language**: Rust
- **GUI Framework**: egui (immediate mode GUI)
- **Cross-Platform**: Native performance on Windows, macOS, and Linux
- **Algorithm**: Iterative Fibonacci calculation for optimal performance
- **Return Type**: u64 to handle larger Fibonacci numbers
- **Dependencies**: Minimal - just eframe and egui
- **Architecture**: Modular design with separate concerns

## Code Structure

The application is organized into several modules:

- **`main.rs`**: Application entry point and configuration
- **`app.rs`**: Main application state and logic
- **`fibonacci.rs`**: Fibonacci calculation algorithms and utilities
- **`ui.rs`**: User interface components and styling
- **`visualization.rs`**: Spiral drawing and visualization logic

### Module Overview

```
src/
├── main.rs           # Entry point
├── app.rs            # Application state & eframe::App implementation
├── fibonacci.rs      # Mathematical calculations
├── ui.rs             # UI components and validation
└── visualization.rs  # Spiral drawing and graphics
```

## Features & Benefits

### Modular Architecture
- **Separation of Concerns**: Each module has a single responsibility
- **Maintainability**: Easy to modify individual components
- **Testability**: Each module can be tested independently
- **Extensibility**: Simple to add new features or algorithms

### Performance Optimizations
- **Iterative Algorithm**: More efficient than recursive approach
- **Optimized Rendering**: Smooth graphics with egui
- **Memory Efficient**: Minimal memory footprint

### User Experience
- **Responsive UI**: Real-time feedback and smooth interactions
- **Visual Learning**: Beautiful spiral helps understand the golden ratio
- **Mathematical Insights**: Shows relationships between Fibonacci numbers

## Limitations

- Maximum input is 25 for optimal visual display
- Spiral visualization works best for n ≤ 12 due to screen space
- Uses u64 for calculations (supports up to F(93))

## License

This project is open source and available under the MIT License.
