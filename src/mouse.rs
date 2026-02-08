/// Mouse movement input structure
pub struct MouseMovementInput {
    /// Horizontal and vertical movement vector
    pub movement_vector: [f32; 2],
    /// Sensitivity factor for mouse movement
    pub sensitivity_factor: f32,
}

/// Handles mouse movement based on input
pub fn movement_control(input: MouseMovementInput) {
    let sensitivity = input.sensitivity_factor;
    let [horizontal, vertical] = input.movement_vector;

    if horizontal.abs() < 0.1 && vertical.abs() < 0.1 {
        return;
    }

    let delta_x = (horizontal * sensitivity * 10.0) as i32;
    let delta_y = (vertical * sensitivity * 10.0) as i32;

    if delta_x != 0 || delta_y != 0 {
        #[cfg(target_os = "windows")]
        {
            use std::mem;
            use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
                INPUT, INPUT_0, INPUT_MOUSE, MOUSE_INPUT, MOUSEEVENTF_MOVE, SendInput, SetCursorPos,
            };

            let current_pos = get_cursor_pos_windows();
            let new_x = current_pos.0 + delta_x;
            let new_y = current_pos.1 + delta_y;

            unsafe {
                SetCursorPos(new_x, new_y);
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::process::Command;

            Command::new("xdotool")
                .args(&[
                    "mousemove_relative",
                    &delta_x.to_string(),
                    &delta_y.to_string(),
                ])
                .output()
                .ok();
        }
    }
}

#[cfg(target_os = "windows")]
fn get_cursor_pos_windows() -> (i32, i32) {
    use std::mem;
    use windows_sys::Win32::UI::WindowsAndMessaging::{GetCursorPos, POINT};

    let mut point = POINT { x: 0, y: 0 };
    unsafe {
        if GetCursorPos(&mut point) != 0 {
            (point.x, point.y)
        } else {
            (0, 0)
        }
    }
}
