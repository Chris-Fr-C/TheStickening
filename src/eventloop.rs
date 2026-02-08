use crate::config::Config;
use crate::mouse::{MouseMovementInput, movement_control};
use crate::setupapp::{AppSetup, setup_app};
use gilrs::{Axis, Button, EventType, Gilrs};
use std::sync::{Arc, Mutex};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};

struct AppState {
    gilrs: Gilrs,
    config: Arc<Mutex<Config>>,
    app_setup: AppSetup,
}

impl ApplicationHandler for AppState {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let mut movement_vector = [0.0f32, 0.0f32];
        let mut sensitivity_factor = 1.0f32;

        // Process gamepad events
        while let Some(gil_event) = self.gilrs.next_event() {
            match gil_event.event {
                EventType::AxisChanged(axis, value, _) => {
                    println!("Axis changed");
                    let config_guard = self.config.lock().unwrap();
                    let joystick = config_guard.mouse_joystick;
                    drop(config_guard);

                    match joystick {
                        crate::config::Joystick::Left => match axis {
                            Axis::LeftStickX => movement_vector[0] = value,
                            Axis::LeftStickY => movement_vector[1] = value,
                            _ => {}
                        },
                        crate::config::Joystick::Right => match axis {
                            Axis::RightStickX => movement_vector[0] = value,
                            Axis::RightStickY => movement_vector[1] = value,
                            _ => {}
                        },
                    }
                }
                EventType::ButtonChanged(button, value, _) => {
                    println!("Button pressed");
                    let config_guard = self.config.lock().unwrap();
                    if let Button::Unknown = button {
                        // Button::Unknown means we need to compare the raw button code
                        // For now, we'll assume the aim_button is stored as the raw code we want
                        let normalized_value = value.clamp(0.0, 1.0);
                        sensitivity_factor =
                            normalized_value.min(config_guard.min_mouse_sensitivity);
                    }
                    drop(config_guard);
                }
                _ => {}
            }
        }

        // Handle mouse movement
        let config_guard = self.config.lock().unwrap();
        let deadzone = config_guard.joystick_deadzone;
        drop(config_guard);

        if movement_vector[0].abs() > deadzone || movement_vector[1].abs() > deadzone {
            let mouse_input = MouseMovementInput{
                movement_vector,
                sensitivity_factor,
                deadzone,
            };
            movement_control(mouse_input);
        }

        event_loop.set_control_flow(ControlFlow::Poll);
    }
}

/// Runs the main event loop
/// The box thingy is to handle any type of error.
pub fn run_event_loop(config: Arc<Mutex<Config>>) -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    println!("Starting up");
    let app_setup = setup_app()?;
    let gilrs = Gilrs::new()?;
    // Iterate over all connected gamepads
    println!("Looking for controlers...");
    for (_id, gamepad) in gilrs.gamepads() {
        println!("Following gamepad detected: {} is {:?}", gamepad.name(), gamepad.power_info());
    }
    let mut app_state = AppState {
        gilrs,
        config,
        app_setup,
    };
    println!("Starting event loop");
    event_loop.run_app(&mut app_state)?;

    Ok(())
}
