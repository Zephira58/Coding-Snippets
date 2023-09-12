use std::thread;
use std::time::Duration;

use eframe::egui::{self, Color32, Visuals, Window};
use egui_notify::{Anchor, Toast, Toasts};

mod api_handler; //Imports the API handler
use api_handler::*;

use crate::APP_NAME;

pub struct MyApp {
    //Enter global values to be used with your app here
    message: String,
    toasts: Toasts,
    closable: bool,
    duration: f32,
    webhook: String,
    username: String,
    avatar_url: String,
    embed: bool,
    embed_title: String,
    embed_footer: String,
    embed_footer_icon: String,
    embed_image: String,
    embed_thumbnail: String,
    embed_field_title: String,
    embed_field_value: String,
}

impl Default for MyApp {
    //defaults for your global values
    fn default() -> Self {
        Self {
            //enter global default values here
            message: "".to_string(),
            toasts: Toasts::default().with_anchor(Anchor::TopRight),
            closable: true,
            duration: 3.5,
            webhook: "".to_string(),
            username: "Xans Webhook Sender".to_string(),
            avatar_url: "https://cdn.discordapp.com/avatars/292971545956188160/eab559efa07f0f3dd13d21ac5f26c4ce.png?size=1024".to_string(),
            embed: false,
            embed_title: "".to_string(),
            embed_footer: "".to_string(),
            embed_footer_icon: "".to_string(),
            embed_image: "".to_string(),
            embed_thumbnail: "".to_string(),
            embed_field_title: "".to_string(),
            embed_field_value: "".to_string(),
        }
    }
}

// The env! macro gets the variable at compile time.
const CURRENT_BUILD: &str = env!("CARGO_PKG_VERSION");
const REPO_URL: &str = env!("CARGO_PKG_REPOSITORY");

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui|{
            Window::new(APP_NAME).show(ctx, |ui| {
                ui.style_mut().visuals = Visuals::dark(); // Makes the buttons dark
                ctx.set_visuals(egui::Visuals::dark()); // Make the ui dark
                egui::warn_if_debug_build(ui);

                let cb = |t: &mut Toast| {
                    //Callback for the toast
                    t.set_closable(self.closable)
                        .set_duration(Some(Duration::from_millis((1000. * self.duration) as u64)));
                };

                ui.horizontal(|ui| {
                    ui.label("Current build:");
                    ui.hyperlink_to(CURRENT_BUILD, REPO_URL);
                });

                ui.add_space(10.0);

                self.message = self.message.replace("\n", "");

                //UI elements for application introduction
                ui.label("Hello and welcome to webhook sender!");
                ui.label(
                "You can randomly generate an insult, affirmation or write your own message in the boxes below!\n");
                ui.colored_label(Color32::from_rgb(150, 0, 0), "Notice: This application is not affiliated with Discord in any way.\nThe application will say message sent even if the webhook URL is invalid.");

                ui.separator();

                //Base UI elements for the app
                ui.label("*Required");
                ui.horizontal(|ui| {
                    ui.label("*Enter Webhook URL:");
                    ui.text_edit_singleline(&mut self.webhook);
                });
                ui.horizontal(|ui| {
                    ui.label("Enter Username: ");
                    ui.add_space(25.0);
                    ui.text_edit_singleline(&mut self.username);
                });
                ui.horizontal(|ui| {
                    ui.label("Enter Avatar URL: ");
                    ui.add_space(20.0);
                    ui.text_edit_singleline(&mut self.avatar_url);
                });
                ui.horizontal(|ui| {
                    ui.label("*Enter a message:");
                    ui.add_space(21.0);
                    ui.text_edit_multiline(&mut self.message);
                });

                //UI elements for embeds (optional)
                if self.embed {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Enter Embed Title: ");
                        ui.add_space(58.0);
                        ui.text_edit_singleline(&mut self.embed_title);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Enter Embed Footer: ");
                        ui.add_space(46.0);
                        ui.text_edit_singleline(&mut self.embed_footer);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Enter Embed Footer Icon: ");
                        ui.add_space(20.0);
                        ui.text_edit_singleline(&mut self.embed_footer_icon);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Enter Embed Image URL: ");
                        ui.add_space(24.0);
                        ui.text_edit_singleline(&mut self.embed_image);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Enter Embed Thumbnail URL: ");
                        ui.text_edit_singleline(&mut self.embed_thumbnail);
                    });
                    ui.horizontal(|ui| {
                        ui.label("*Enter Embed Field Title: ");
                        ui.add_space(23.0);
                        ui.text_edit_singleline(&mut self.embed_field_title);
                    });
                    ui.horizontal(|ui| {
                        ui.label("*Enter Embed Field Value: ");
                        ui.add_space(17.0);
                        ui.text_edit_singleline(&mut self.embed_field_value);
                    });
                }

                ui.separator();

                ui.label("-Your message-");
                ui.label(&self.message);

                //UI for the buttons and error handling
                ui.horizontal(|ui| {
                    if ui.button("Generate an insult").clicked() {
                        self.message = get_insult();
                        cb(self.toasts.success("Generation Successful!")); //Sends a success toast
                    }

                    if ui.button("Generate an affirmation").clicked() {
                        self.message = get_affirmation();
                        cb(self.toasts.success("Generation Successful!"));
                    }

                    if ui.button("Send message").clicked() {
                        let mut error = false;
                        if self.webhook.is_empty() {
                            println!("\nERROR: Webhook URL not found!");
                            cb(self.toasts.error("Please enter a wehbook url"));
                            error = true;
                        }
                        if self.message.is_empty() {
                            println!("\nERROR: Message not found!");
                            cb(self.toasts.error("Please enter a message"));
                            error = true;
                        }

                        if self.embed {
                            if self.embed_field_title.is_empty() || self.embed_field_value.is_empty() {
                                cb(self.toasts.error("Embed field title and value are required!"));
                                println!("ERROR: Embed field title and value are required!");
                                error = true;
                            }
                            if !error{
                                send_embed(&self.message, &self.webhook, &self.username, &self.avatar_url, &self.embed_title, &self.embed_footer, &self.embed_footer_icon, &self.embed_image, &self.embed_thumbnail, &self.embed_field_title, &self.embed_field_value).expect("Error sending embed");
                                cb(self.toasts.success("Embed Sent!"));
                                self.message = "".to_string();
                                println!("Embed sent!");
                                return;
                            }
                        }

                        if !error{
                            send_message(&self.message, &self.webhook, &self.username, &self.avatar_url);
                            cb(self.toasts.success("Message Sent!"));
                            self.message = "".to_string();
                        }
                    }

                    //Various UI elements and checks for application updates
                    let update = ui.button("Update");
                    if update.hovered() {
                        egui::show_tooltip(ui.ctx(), egui::Id::new("my_tooltip"), |ui| {
                            ui.label("Download any updates if available");
                        });
                    }
                    if update.clicked() {
                        cb(self.toasts.info("Updating... See console for logs"));
                        println!("\nChecking for updates...");
                        thread::spawn(move || {
                            download_update().expect("Failed to download update");
                        });
                    }

                    //UI elements for the embed toggle
                    if ui.checkbox(&mut self.embed, "Embed?").clicked() {
                        println!("\nEmbed: {}", &self.embed);
                        cb(self.toasts.success("Embed toggled!"));
                    }
                });
                self.toasts.show(ctx); // Requests to render toasts
            });
        });
    }
}
