mod config;
mod eventloop;
mod setupapp;
mod setupinterface;

#[cfg(target_os = "windows")]
mod setupwin;

#[cfg(target_os = "linux")]
mod setuplinux;

use config::Config;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Arc::new(Mutex::new(Config::load_from_file(None)?));
    eventloop::run_event_loop(config)?;
    Ok(())
}
