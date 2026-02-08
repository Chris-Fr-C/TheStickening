/// Interface for platform-specific app setup
use tray_icon::{
    menu::{Menu, MenuItem},
    TrayIcon,
};

/// Trait for platform-specific setup implementations
pub trait SetupInterface {
    fn setup() -> Result<SetupComponents, Box<dyn std::error::Error>>;
}

/// Common setup components across platforms
pub struct SetupComponents {
    pub tray_icon: TrayIcon,
    pub quit_menu_item: MenuItem,
}
