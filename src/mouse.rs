/// Mouse movement input structure
pub struct MouseMovementInput {
    /// Horizontal and vertical movement vector
    pub movement_vector: [f32; 2],
    /// Sensitivity factor for mouse movement
    pub sensitivity_factor: f32,
    /// We will ignore moves that dont reach that threshold.
    pub deadzone: f32,
}

/// Handles mouse movement based on input
pub fn movement_control(input: MouseMovementInput) {
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
    let delta_x = (horizontal * sensitivity) as i32;
    let delta_y = (vertical * sensitivity) as i32;
    println!("Moving delta {} {}", delta_x, delta_y);
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
