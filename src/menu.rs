use tray_icon::{menu::MenuItem, TrayIcon};

/// Common setup components across platforms
pub struct SetupComponents {
    pub tray_icon: TrayIcon,
    pub quit_menu_item: MenuItem,
}
