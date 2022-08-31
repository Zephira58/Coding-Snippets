#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui; //Imports the rendering engine
use eframe::egui::Visuals; //Imports dark mode

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui", //app name
        options, //just leave this at options
        Box::new(|_cc| Box::new(MyApp::default())), //leave this as default
    );
}

struct MyApp {
    //Enter global values to be used with your app here
    name: String,
    age: i32,
    checkbox: bool,
    test: String,
}

impl Default for MyApp { //defaults for your global values
    fn default() -> Self {
        Self {
            //enter global default values here
            name: "".to_string(),
            age: 0,
            checkbox: false,
            test: "".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().visuals = Visuals::dark(); // Makes the buttons dark
            ctx.set_visuals(egui::Visuals::dark()); // Make the ui dark

            egui::warn_if_debug_build(ui); // Warns if debug build
            ctx.request_repaint(); // Keep updating frame
            ui.separator();
            
            ui.horizontal(|ui| { // Makes whatevers nested in here side by side
                ui.label("Enter your name: "); // Essentily prints a message to the screen
                ui.text_edit_singleline(&mut self.name); // Makes a text edit box
            });

            ui.horizontal(|ui| {
            ui.label("Enter your age");
            ui.add(egui::DragValue::new(&mut self.age)); // Adds a clickable/editable number to the screen
            });

            let checkbox = ui.checkbox(&mut self.checkbox, "Click a checkbox"); // Renders a checkbox on the screen

            if checkbox.clicked() { // Makes the checkbox clickable and changes the value of the checkbox when clicked
                ui.toggle_value(&mut self.checkbox, "text"); // Toggles the value of the checkbox
            };
            if checkbox.secondary_clicked() {
                self.checkbox = false; // The "self" prefix is uses to access the global value
            };
            ui.separator(); // Adds a separator 

                if ui.button("Subtract").clicked() { // Makes a button that subtracts 1 from the age value
                    self.age -= 1;
                };
                if ui.button("Add").clicked() { // Makes a button that adds 1 to the age value
                    self.age += 1;
                };

                let response = ui.button("Open popup"); // Makes a button that opens a popup
                let popup_id = ui.make_persistent_id("my_unique_id");

                if response.clicked() {
                    ui.memory().toggle_popup(popup_id);
                }
                
                egui::popup::popup_below_widget(ui, popup_id, &response, |ui| {
                    //The contents of the popup go here
                    ui.set_min_width(200.0); // if you want to control the size
                    ui.label("This is an example of a popup window");
                    ui.text_edit_multiline(&mut self.test);

                    //How to make a tooltip
                    if ui.ui_contains_pointer() {
                        egui::show_tooltip(ui.ctx(), egui::Id::new("my_tooltip"), |ui| {
                            //The contents of the tooltip go here
                            ui.label("Demo tooltip");
                        });
                    }
                });



                egui::ScrollArea::vertical().show(ui, |ui| { //Adds a scrollbar to anything nested in here
                    ui.label(&format!("Hello {}! You are {} years old.", self.name, self.age)); // Prints a message to the screen
                    if self.checkbox {
                        ui.separator();
                        for i in 0..100 { // Prints a message to the screen 100 times
                            ui.label(&format!("Theres a scrollbar on the right"));
                        }
                    }
                });
        });
    }
}
