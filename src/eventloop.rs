use crate::config::Config;
use crate::menu::SetupComponents;
use crate::mouse::{MouseMovementInput, click_control, movement_control};
use crate::setupapp::setup;
use gilrs::{EventType, Gilrs};
use std::f32::consts::PI;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};

struct AppState {
    gilrs: Gilrs,
    config: Config,
    mouse_input: MouseMovementInput,
    app_setup: SetupComponents,
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
            match self.extract_event() {
                Ok(it) => it,
                Err(err) => println!("An error occured {}", err),
            };

            self.last_update = now;
        }

        let next_update_time = self.last_update + target_duration;
        event_loop.set_control_flow(ControlFlow::WaitUntil(next_update_time));
    }
}

impl AppState {
    fn extract_event(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Process all pending gamepad events
        self.process_event()?;

        // Handle mouse movement
        let deadzone = self.config.joystick_deadzone;
        if self.mouse_input.movement_vector[0].abs() > deadzone
            || self.mouse_input.movement_vector[1].abs() > deadzone
        {
            movement_control(&self.mouse_input);
        }
        Ok(())
    }

    fn smooth_axis_event_value(&self, axis: &gilrs::Axis, value: f32) -> Result<f32, &str> {
        let profile = self.axis_profile(axis)?;
        // Note we only handle 0 to 1. Sign has to be handled outside.
        smooth_profile(value, profile)
    }

    fn smooth_button_event_value(&self, btn: &gilrs::Button, value: f32) -> Result<f32, &str> {
        let profile = self.button_profile(btn)?;
        // Note we only handle 0 to 1. Sign has to be handled outside.
        smooth_profile(value, profile)
    }
    fn axis_profile(
        &self,
        axis: &gilrs::Axis,
    ) -> Result<&crate::config::AccelerationProfile, &str> {
        let profile = match axis {
            gilrs::Axis::LeftStickX | gilrs::Axis::LeftStickY => {
                Ok(&self.config.left_joystick_smoothing)
            }
            gilrs::Axis::RightStickX | gilrs::Axis::RightStickY => {
                Ok(&self.config.right_joystick_smoothing)
            }
            gilrs::Axis::LeftZ | gilrs::Axis::RightZ => Ok(&self.config.zaxis_smoothing),
            _ => Err("No profile defined for provided axis"),
        }?;
        Ok(profile)
    }

    fn button_profile(
        &self,
        btn: &gilrs::Button,
    ) -> Result<&crate::config::AccelerationProfile, &str> {
        let profile = if btn == &self.config.aim_button {
            &self.config.zaxis_smoothing
        } else {
            &crate::config::AccelerationProfile::Linear
        };
        Ok(profile)
    }

    fn process_event(&mut self) -> Result<(), String> {
        while let Some(gil_event) = self.gilrs.next_event() {
            match gil_event.event {
                EventType::AxisChanged(axis, value, _) => {
                    let smoothed_value = self.smooth_axis_event_value(&axis, value)?;
                    // We smooth out based on the joystick
                    if axis == self.config.mouse_joystick.x_axis() {
                        self.mouse_input.movement_vector[0] = smoothed_value;
                    } else if axis == self.config.mouse_joystick.y_axis() {
                        self.mouse_input.movement_vector[1] = smoothed_value;
                    }
                }
                EventType::ButtonChanged(id, value, _code) => {
                    if id == self.config.aim_button {
                        let smoothed_value =
                            self.smooth_button_event_value(&self.config.aim_button, value)?;
                        // Afine function that goes from aim factor to 1
                        let p = self.config.aim_sensitivity_factor;
                        let m = 1. - self.config.aim_sensitivity_factor;
                        // When value == 0 it means we are not touching and the sensitivity should
                        // be at its max possible.
                        let x = 1. - smoothed_value;
                        let modifier = m * x + p;

                        self.mouse_input.sensitivity_factor =
                            self.config.mouse_sensitivity * modifier
                    }
                }
                event @ EventType::ButtonPressed(_btn, code)
                | event @ EventType::ButtonReleased(_btn, code) => {
                    // It could be in changed event but i want it to be more readable and explicit.
                    let key = code.into_u32().to_string();
                    if self.config.button_mapping.contains_key(&key)
                    // to string directly would put
                    // Button(0) instead of just 0
                    {
                        self.handle_button(&self.config.button_mapping[&key], &event);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_button(&self, action: &crate::config::ButtonAction, event: &EventType) {
        match action {
            crate::config::ButtonAction::MouseLeft
            | crate::config::ButtonAction::MouseRight
            | crate::config::ButtonAction::MouseMiddle => {
                click_control(action, event);
            }
            _ => todo!("Other buttons not implemented yet"), // Here we will add keyboard
        }
    }
}

fn smooth_profile(value: f32, profile: &crate::config::AccelerationProfile) -> Result<f32, &str> {
    let smoothing_function: fn(f32) -> f32 = match profile {
        crate::config::AccelerationProfile::Linear => |x| x,
        crate::config::AccelerationProfile::SmoothStep => |x| x * x * (3. - 2. * x),
        crate::config::AccelerationProfile::SmootherStep => {
            |x| x * x * x * (x * (6. * x - 15.) + 10.)
        }

        crate::config::AccelerationProfile::EaseIn => |x| x * x,
        crate::config::AccelerationProfile::EaseInOut => |x: f32| -> f32 {
            if x < 0.5 {
                2.0 * x * x
            } else {
                1.0 - ((-2.0 * x + 2.0).powi(2)) / 2.0
            }
        },
        crate::config::AccelerationProfile::EaseOut => |x| 1. - (1. - x) * (1. - x),
        crate::config::AccelerationProfile::SinusoidalEasing => {
            |x: f32| -> f32 { 0.5 - 0.5 * (PI * x).cos() }
        }
        crate::config::AccelerationProfile::EaseInOutExpo => |x: f32| -> f32 {
            if x == 0.0 {
                0.0
            } else if x == 1.0 {
                1.0
            } else if x < 0.5 {
                (2.0_f32).powf(20.0 * x - 10.0) / 2.0
            } else {
                (2.0 - (2.0_f32).powf(-20.0 * x + 10.0)) / 2.0
            }
        },
    };
    Ok(value.signum() * smoothing_function(value.abs()))
}

/// Runs the main event loop
/// The box thingy is to handle any type of error.
pub fn run_event_loop(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    println!("Starting up");
    let app_setup = setup()?;
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
