use crate::config::Config;
use crate::setupapp::{setup_app, AppSetup};
use std::sync::{Arc, Mutex};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

/// Runs the main event loop
/// The box thingy is to handle any type of error.
pub fn run_event_loop(config: Arc<Mutex<Config>>) -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("TheStickening")
        .with_visible(false)
        .build(&event_loop)?;

    let app_setup = setup_app()?;
    let tray_icon = app_setup.tray_icon;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            winit::event::Event::NewEvents(_) => {}
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            winit::event::Event::MenuEvent { menu_id, .. } => {
                if menu_id == app_setup.quit_menu_item.id() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            winit::event::Event::RedrawRequested(_) => {}
            winit::event::Event::RedrawEventsCleared => {}
            _ => {}
        }
    })?;

    Ok(())
}
