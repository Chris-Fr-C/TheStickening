use crate::config::Config;
use crate::mouse::{MouseMovementInput, movement_control};
use crate::setupapp::{AppSetup, setup_app};
use gilrs::{EventType, Gilrs};
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};

struct AppState {
    gilrs: Gilrs,
    config: Config,
    mouse_input: MouseMovementInput,
    app_setup: AppSetup,
    last_update: Instant,
}

impl ApplicationHandler for AppState {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if event == WindowEvent::CloseRequested {
            event_loop.exit();
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        let target_duration = Duration::from_secs_f32(1.0 / self.config.frequency);

        if now.duration_since(self.last_update) >= target_duration {
            // Process all pending gamepad events
            while let Some(gil_event) = self.gilrs.next_event() {
                match gil_event.event {
                    EventType::AxisChanged(axis, value, _) => {
                        if axis == self.config.mouse_joystick.x_axis() {
                            self.mouse_input.movement_vector[0] = value;
                        } else if axis == self.config.mouse_joystick.y_axis() {
                            self.mouse_input.movement_vector[1] = value;
                        }
                    }
                    EventType::ButtonChanged(id, value, _code) => {
                        if id == self.config.aim_button {
                            // Afine function that goes from aim factor to 1
                            let p = self.config.aim_sensitivity_factor;
                            let m = 1. - self.config.aim_sensitivity_factor;
                            // When value == 0 it means we are not touching and the sensitivity should
                            // be at its max possible.
                            let x = 1. - value.clamp(0., 1.);
                            let modifier = m * x + p;

                            self.mouse_input.sensitivity_factor =
                                self.config.mouse_sensitivity * modifier
                        }
                    }
                    _ => {}
                }
            }

            // Handle mouse movement
            let deadzone = self.config.joystick_deadzone;
            if self.mouse_input.movement_vector[0].abs() > deadzone
                || self.mouse_input.movement_vector[1].abs() > deadzone
            {
                movement_control(&self.mouse_input);
            }

            self.last_update = now;
        }

        let next_update_time = self.last_update + target_duration;
        event_loop.set_control_flow(ControlFlow::WaitUntil(next_update_time));
    }
}

/// Runs the main event loop
/// The box thingy is to handle any type of error.
pub fn run_event_loop(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    println!("Starting up");
    let app_setup = setup_app()?;
    let gilrs = Gilrs::new()?;
    // Iterate over all connected gamepads
    println!("Looking for controlers...");
    for (_id, gamepad) in gilrs.gamepads() {
        println!(
            "Following gamepad detected: {} is {:?}",
            gamepad.name(),
            gamepad.power_info()
        );
    }
    let empty_mvt = MouseMovementInput {
        movement_vector: [0 as f32, 0 as f32],
        sensitivity_factor: config.mouse_sensitivity,
        deadzone: config.joystick_deadzone, // realized its a bit duplicate but will fix later.
    };
    let mut app_state = AppState {
        gilrs,
        config,
        mouse_input: empty_mvt,
        app_setup,
        last_update: Instant::now(),
    };
    println!("Starting event loop");
    event_loop.run_app(&mut app_state)?;

    Ok(())
}
