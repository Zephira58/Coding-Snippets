#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
use app::*;

pub const APP_NAME: &str = "Webhook Sender";
fn main() {
    print!("{esc}c", esc = 27 as char);
    println!("\nApplication made by Xanthus58");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
    println!("License: {}", env!("CARGO_PKG_LICENSE"));
    println!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
    println!("Notice: This application is not affiliated with Discord in any way.\nThe application will say message sent even if the webhook URL is invalid.");
    println!("\n-Logs-");
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        APP_NAME,                                   //app name
        options,                                    //just leave this at options
        Box::new(|_cc| Box::new(MyApp::default())), //leave this as default
    );
}
