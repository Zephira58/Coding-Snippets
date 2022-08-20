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
}

impl Default for MyApp { //defaults for your global values
    fn default() -> Self {
        Self {
            //enter global default values here
            name: "".to_string(),
            age: 0,
            checkbox: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().visuals = Visuals::dark(); // Makes the buttons dark
            ctx.set_visuals(egui::Visuals::dark()); // Make the ui dark

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
                self.checkbox = true; // The "self" prefix is uses to access the global value
            };
            if checkbox.secondary_clicked() {
                self.checkbox = false;
            };
            ui.separator(); // Adds a separator 

                if ui.button("Subtract").clicked() { // Makes a button that subtracts 1 from the age value
                    self.age -= 1;
                };
                if ui.button("Add").clicked() { // Makes a button that adds 1 to the age value
                    self.age += 1;
                };

                egui::ScrollArea::vertical().show(ui, |ui| { //Adds a scrollbar to anything nested in here
                    ui.label(&format!("Hello {}! You are {} years old.", self.name, self.age)); // Prints a message to the screen
                    if self.checkbox {
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");
                        ui.label("Theres a scrollbar on the right");

                    }
                });
        });
    }
}
