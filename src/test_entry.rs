use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Box, Button, Entry, Label, Orientation};

const APP_ID: &str = "org.fibonacci.test";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_test_ui);
    app.run()
}

fn build_test_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Entry Test")
        .default_width(400)
        .default_height(200)
        .build();

    let main_box = Box::new(Orientation::Vertical, 10);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);

    let label = Label::new(Some("Type something in the entry field:"));
    let entry = Entry::new();
    entry.set_placeholder_text(Some("Type here..."));
    
    let result_label = Label::new(Some("You typed: "));
    
    let button = Button::with_label("Show Text");
    
    let result_label_clone = result_label.clone();
    let entry_clone = entry.clone();
    button.connect_clicked(move |_| {
        let text = entry_clone.text();
        result_label_clone.set_text(&format!("You typed: {}", text));
    });

    main_box.append(&label);
    main_box.append(&entry);
    main_box.append(&button);
    main_box.append(&result_label);

    window.set_child(Some(&main_box));
    window.present();
    
    // Try to focus the entry
    entry.grab_focus();
}
