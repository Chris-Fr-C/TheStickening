use rand::Rng;

/// Mouse movement input structure
#[derive(Debug)]
pub struct MouseMovementInput {
    /// Horizontal and vertical movement vector
    pub movement_vector: [f32; 2],
    /// Sensitivity factor for mouse movement
    pub sensitivity_factor: f32,
    /// We will ignore moves that dont reach that threshold.
    pub deadzone: f32,
}

/// Handles mouse movement based on input
pub fn movement_control(input: &MouseMovementInput) {
    let sensitivity = input.sensitivity_factor;
    let [mut horizontal, mut vertical] = input.movement_vector;
    if horizontal.abs() < input.deadzone && vertical.abs() < input.deadzone {
        return;
    }
    // We cleanup residual stuff
    horizontal = if horizontal.abs() < input.deadzone {
        0.0
    } else {
        horizontal
    };
    vertical = if vertical.abs() < input.deadzone {
        0.0
    } else {
        // Lol might be overkill to put the else statement but eh
        vertical
    };

    // For a big enough frequency the randomness will be smoothed out.
    // The goal is that since we cant move less than a pixel, when we want to be
    // slower than a pixel per period, we will just do one every N period, randomly but
    // generally converging to the expected speed.
    // the Y axis is inverted when sent to screen.
    let lucky_num: f32 = rand::random();
    let mut dxf = horizontal * sensitivity;
    if dxf.abs() < 1. && dxf.abs() < lucky_num {
        dxf = 0.;
    } else if dxf.abs() < 1. && dxf.abs() >= lucky_num {
        dxf = dxf.signum();
    }
    let delta_x = dxf as i32;
    // If you wanna buffer overflow its due to your config.
    let mut dyf = -vertical * sensitivity;
    if dyf.abs() < 1. && dyf.abs() < lucky_num {
        dyf = 0.;
    } else if dyf.abs() < 1. && dyf.abs() >= lucky_num {
        dyf = dyf.signum();
    }
    let delta_y = dyf as i32;

    print!(
        "x {} dx {} y {} dy {}  sensitivity {}  lucky_num {}              \r",
        horizontal, delta_x, vertical, delta_y, sensitivity, lucky_num
    );
    if delta_x != 0 || delta_y != 0 {
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::UI::Input::KeyboardAndMouse::{MOUSEEVENTF_MOVE, mouse_event};

            unsafe {
                // the 0 is cause we dont use the wheel
                mouse_event(MOUSEEVENTF_MOVE, delta_x, delta_y, 0, 0);
            }
        }
        // I dont want to support linux yet as i am manually testing for the moment on windows.
    }
}
