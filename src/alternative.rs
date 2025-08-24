use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Box, Button, Entry, Label, Orientation, Frame};

const APP_ID: &str = "org.fibonacci.alternative";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_alternative_ui);
    app.run()
}

fn build_alternative_ui(app: &Application) {
    println!("Building alternative UI...");
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Fibonacci Calculator - Alternative")
        .default_width(500)
        .default_height(400)
        .build();

    // Create main container with frame
    let frame = Frame::new(Some("Fibonacci Calculator"));
    let main_box = Box::new(Orientation::Vertical, 20);
    main_box.set_margin_top(30);
    main_box.set_margin_bottom(30);
    main_box.set_margin_start(30);
    main_box.set_margin_end(30);

    // Create input section with frame
    let input_frame = Frame::new(Some("Input"));
    let input_box = Box::new(Orientation::Vertical, 10);
    input_box.set_margin_top(15);
    input_box.set_margin_bottom(15);
    input_box.set_margin_start(15);
    input_box.set_margin_end(15);

    let instruction = Label::new(Some("Enter a number between 0 and 40:"));
    
    let entry = Entry::new();
    entry.set_text("10"); // Pre-fill with a default value
    entry.set_width_chars(25);
    entry.set_alignment(0.5); // Center the text
    
    // Add debugging
    entry.connect_changed(|entry| {
        println!("Entry changed: '{}'", entry.text());
    });
    
    let button = Button::with_label("🧮 Calculate Fibonacci Number");
    button.set_size_request(200, 50);
    
    input_box.append(&instruction);
    input_box.append(&entry);
    input_box.append(&button);
    input_frame.set_child(Some(&input_box));

    // Create result section with frame
    let result_frame = Frame::new(Some("Result"));
    let result_box = Box::new(Orientation::Vertical, 10);
    result_box.set_margin_top(15);
    result_box.set_margin_bottom(15);
    result_box.set_margin_start(15);
    result_box.set_margin_end(15);

    let result_label = Label::new(Some("Click the button to calculate"));
    result_label.set_selectable(true);
    result_label.set_wrap(true);
    result_label.set_justify(gtk4::Justification::Center);
    
    result_box.append(&result_label);
    result_frame.set_child(Some(&result_box));

    // Add sections to main box
    main_box.append(&input_frame);
    main_box.append(&result_frame);
    frame.set_child(Some(&main_box));

    // Set up event handlers
    let entry_clone = entry.clone();
    let result_clone = result_label.clone();
    
    button.connect_clicked(move |_| {
        println!("Button clicked!");
        let input_text = entry_clone.text();
        println!("Processing input: '{}'", input_text);
        
        match input_text.parse::<u32>() {
            Ok(n) if n <= 40 => {
                let result = fib(n);
                let text = format!("F({}) = {}\n\nThe {}th Fibonacci number is {}", n, result, n, result);
                result_clone.set_text(&text);
                println!("Result: F({}) = {}", n, result);
            }
            Ok(n) => {
                result_clone.set_text(&format!("Number {} is too large!\nPlease enter a number between 0 and 40", n));
            }
            Err(_) => {
                result_clone.set_text("Invalid input!\nPlease enter a valid number");
            }
        }
    });

    // Enter key handler
    entry.connect_activate(move |entry| {
        println!("Enter key pressed!");
        let input_text = entry.text();
        println!("Processing input from Enter: '{}'", input_text);
        
        match input_text.parse::<u32>() {
            Ok(n) if n <= 40 => {
                let result = fib(n);
                let text = format!("F({}) = {}\n\nThe {}th Fibonacci number is {}", n, result, n, result);
                result_label.set_text(&text);
                println!("Result: F({}) = {}", n, result);
            }
            Ok(n) => {
                result_label.set_text(&format!("Number {} is too large!\nPlease enter a number between 0 and 40", n));
            }
            Err(_) => {
                result_label.set_text("Invalid input!\nPlease enter a valid number");
            }
        }
    });

    window.set_child(Some(&frame));
    window.present();
    
    // Focus the entry field
    entry.grab_focus();
    
    println!("Alternative UI setup complete!");
}

fn fib(n: u32) -> u64 {
    if n < 2 {
        n as u64
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
