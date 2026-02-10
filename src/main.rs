mod app;
mod config;
mod gamepad;
mod menu;
mod mouse;
mod setupapp;
mod smoothing;
mod tray;

use app::StickApp;
use config::Config;
use iced::window;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load_from_file(None)?;
    println!("Configuration loaded.");
    
    iced::application(StickApp::title, StickApp::update, StickApp::view)
        .subscription(StickApp::subscription)
        .window(window::Settings {
            exit_on_close_request: false,
            ..window::Settings::default()
        })
        .run_with(move || StickApp::new(config).unwrap())?;
    
    Ok(())
}
