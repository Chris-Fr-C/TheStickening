mod config;
mod eventloop;
mod mouse;
mod setupapp;
mod setupinterface;

#[cfg(target_os = "windows")]
mod setupwin;

#[cfg(target_os = "linux")]
mod setuplinux;

use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load_from_file(None)?;
    println!("Configuration loaded.");
    eventloop::run_event_loop(config)?;
    Ok(())
}
