use std::f32::consts::PI;

use crate::config::AccelerationProfile;

pub fn smooth_profile(value: f32, profile: &AccelerationProfile) -> Result<f32, &str> {
    let smoothing_function: fn(f32) -> f32 = match profile {
        AccelerationProfile::Linear => |x| x,
        AccelerationProfile::SmoothStep => |x| x * x * (3. - 2. * x),
        AccelerationProfile::SmootherStep => |x| x * x * x * (x * (6. * x - 15.) + 10.),
        AccelerationProfile::EaseIn => |x| x * x,
        AccelerationProfile::EaseInOut => |x: f32| -> f32 {
            if x < 0.5 {
                2.0 * x * x
            } else {
                1.0 - ((-2.0 * x + 2.0).powi(2)) / 2.0
            }
        },
        AccelerationProfile::EaseOut => |x| 1. - (1. - x) * (1. - x),
        AccelerationProfile::SinusoidalEasing => |x: f32| -> f32 { 0.5 - 0.5 * (PI * x).cos() },
        AccelerationProfile::EaseInOutExpo => |x: f32| -> f32 {
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
