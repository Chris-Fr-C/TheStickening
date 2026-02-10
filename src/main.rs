mod config;
mod eventloop;
mod menu;
mod mouse;
mod setupapp;
use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load_from_file(None)?;
    println!("Configuration loaded.");
    eventloop::run_event_loop(config)?;
    Ok(())
}
