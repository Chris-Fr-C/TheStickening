mod app;
mod config;
mod gamepad;
mod menu;
mod mouse;
mod setupapp;
mod smoothing;

use app::StickApp;
use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load_from_file(None)?;
    println!("Configuration loaded.");

    iced::application(StickApp::title, StickApp::update, StickApp::view)
        .subscription(StickApp::subscription)
        .run_with(move || StickApp::new(config).unwrap())?;

    Ok(())
}
