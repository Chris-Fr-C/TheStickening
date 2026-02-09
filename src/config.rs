use gilrs::{Axis, Button};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Configuration struct for gamepad to mouse mapping
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Mapping between gamepad buttons and their corresponding actions
    pub button_mapping: HashMap<String, ButtonAction>,
    // button_mapping is not int32 because not valid toml key type.
    /// Which joystick to use for mouse movement (left or right)
    pub mouse_joystick: Joystick,
    /// Default sensitivity for mouse movement
    pub mouse_sensitivity: f32,
    /// Button mapped for accurate aiming mode (default: left trigger)
    pub aim_button: Button,
    /// Sensitivity decrease factor when aiming. Ratio between 0 and 1. (or more if you want it to
    /// get faster somehow). The total sensitivity will be mutiplied at maximum at that value.
    /// For instance a trigger fully pressed with aim_sensitivity_factor = 0.5 means we will divide
    /// by two the speed of the cursor.
    /// Just make sure you have mouse_sensitivity / aim_sensitivity_factor >= 1 because it has to
    /// end up to pixels.
    pub aim_sensitivity_factor: f32,
    /// Joystick deadzone for detecting movement
    pub joystick_deadzone: f32,

    /// Not rendering every frame as we could not slow down enough the mouse.
    /// Specify it in hertz.
    pub frequency: f32,
}

/// Represents available joysticks
/// If you have more than two joystick, I do not support general grievous playing style :(
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Joystick {
    Left,
    Right,
}
impl Joystick {
    pub fn y_axis(&self) -> Axis {
        match self {
            Self::Left => Axis::LeftStickY,
            Self::Right => Axis::RightStickY,
        }
    }
    pub fn x_axis(&self) -> Axis {
        match self {
            Self::Left => Axis::LeftStickX,
            Self::Right => Axis::RightStickX,
        }
    }
}

/// Represents actions that can be triggered by buttons
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonAction {
    MouseLeft,
    MouseRight,
    MouseMiddle,
    None,
}

impl Default for Config {
    fn default() -> Self {
        let mut button_mapping = HashMap::new();
        button_mapping.insert("0".to_string(), ButtonAction::MouseLeft); // A button
        button_mapping.insert("1".to_string(), ButtonAction::MouseRight); // B button

        Self {
            button_mapping,
            mouse_joystick: Joystick::Left,
            mouse_sensitivity: 5.0,
            aim_button: Button::LeftTrigger2, // Left trigger
            aim_sensitivity_factor: 3.,       // Decreases up to if <1, increase up to if >1
            // The min mouse sensitivity is to avoid that we press aim and the mouse stops moving.
            // The joystick deadzone is to avoid mouse movement when the joystick is at rest.
            joystick_deadzone: 0.005,
            frequency: 50.,
        }
    }
}

impl Config {
    /// Creates a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the mouse joystick
    pub fn set_mouse_joystick(&mut self, joystick: Joystick) {
        self.mouse_joystick = joystick;
    }

    /// Sets the mouse sensitivity
    pub fn set_mouse_sensitivity(&mut self, sensitivity: f32) -> Result<(), &'static str> {
        if sensitivity <= 0.0 {
            return Err("Sensitivity must be positive");
        }
        self.mouse_sensitivity = sensitivity;
        Ok(())
    }
    /// Adds or updates a button mapping
    pub fn set_button_mapping(&mut self, button: u32, action: ButtonAction) {
        self.button_mapping.insert(button.to_string(), action);
    }

    /// Gets the action for a specific button
    pub fn get_button_action(&self, button: u32) -> Option<&ButtonAction> {
        self.button_mapping.get(&button.to_string())
    }

    /// Gets the default config file path
    pub fn get_default_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let mut path = dirs::config_dir().ok_or("Failed to get config directory")?;
        path.push("thestickening");
        fs::create_dir_all(&path)?;
        path.push("config.toml");
        Ok(path)
    }

    /// Saves the configuration to a TOML file
    pub fn save_to_file(&self, path: Option<&PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
        let save_path = match path {
            Some(p) => p.clone(),
            None => Self::get_default_path()?,
        };

        let toml_string = toml::to_string_pretty(self)?;
        println!("Trying to save configuration file into {}", toml_string);
        fs::write(save_path, toml_string)?;
        Ok(())
    }

    /// Loads configuration from a TOML file
    pub fn load_from_file(path: Option<&PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let load_path = match path {
            Some(p) => p.clone(),
            None => Self::get_default_path()?,
        };

        if !load_path.exists() {
            let config = Self::default();
            // Its ok to panic, if we cant save the file it means we cant start.
            config.save_to_file(Some(&load_path))?;
            return Ok(config);
        }

        let content = fs::read_to_string(load_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
