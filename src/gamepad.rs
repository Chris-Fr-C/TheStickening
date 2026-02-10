use gilrs::{EventType, Gilrs};

use crate::config::{AccelerationProfile, Config};
use crate::mouse::{click_control, movement_control, MouseMovementInput};
use crate::smoothing::smooth_profile;

pub struct GamepadHandler {
    gilrs: Gilrs,
    pub config: Config,
    mouse_input: MouseMovementInput,
}

impl GamepadHandler {
    pub fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let gilrs = Gilrs::new()?;

        println!("Looking for controllers...");
        for (_id, gamepad) in gilrs.gamepads() {
            println!(
                "Following gamepad detected: {} is {:?}",
                gamepad.name(),
                gamepad.power_info()
            );
        }

        let mouse_input = MouseMovementInput {
            movement_vector: [0.0, 0.0],
            sensitivity_factor: config.mouse_sensitivity,
            deadzone: config.joystick_deadzone,
        };

        Ok(Self {
            gilrs,
            config,
            mouse_input,
        })
    }

    pub fn process_frame(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.process_events()?;

        let deadzone = self.config.joystick_deadzone;
        if self.mouse_input.movement_vector[0].abs() > deadzone
            || self.mouse_input.movement_vector[1].abs() > deadzone
        {
            movement_control(&self.mouse_input);
        }

        Ok(())
    }

    fn process_events(&mut self) -> Result<(), String> {
        while let Some(gil_event) = self.gilrs.next_event() {
            match gil_event.event {
                EventType::AxisChanged(axis, value, _) => {
                    let smoothed_value = self.smooth_axis_event_value(&axis, value)?;
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
                        let p = self.config.aim_sensitivity_factor;
                        let m = 1. - self.config.aim_sensitivity_factor;
                        let x = 1. - smoothed_value;
                        let modifier = m * x + p;

                        self.mouse_input.sensitivity_factor =
                            self.config.mouse_sensitivity * modifier;
                    }
                }
                event @ EventType::ButtonPressed(_btn, code)
                | event @ EventType::ButtonReleased(_btn, code) => {
                    let key = code.into_u32().to_string();
                    if self.config.button_mapping.contains_key(&key) {
                        self.handle_button(&self.config.button_mapping[&key], &event);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn smooth_axis_event_value(&self, axis: &gilrs::Axis, value: f32) -> Result<f32, &str> {
        let profile = self.axis_profile(axis)?;
        smooth_profile(value, profile)
    }

    fn smooth_button_event_value(&self, btn: &gilrs::Button, value: f32) -> Result<f32, &str> {
        let profile = self.button_profile(btn)?;
        smooth_profile(value, profile)
    }

    fn axis_profile(&self, axis: &gilrs::Axis) -> Result<&AccelerationProfile, &str> {
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

    fn button_profile(&self, btn: &gilrs::Button) -> Result<&AccelerationProfile, &str> {
        let profile = if btn == &self.config.aim_button {
            &self.config.zaxis_smoothing
        } else {
            &AccelerationProfile::Linear
        };
        Ok(profile)
    }

    fn handle_button(&self, action: &crate::config::ButtonAction, event: &EventType) {
        match action {
            crate::config::ButtonAction::MouseLeft
            | crate::config::ButtonAction::MouseRight
            | crate::config::ButtonAction::MouseMiddle => {
                click_control(action, event);
            }
            _ => {}
        }
    }
}
