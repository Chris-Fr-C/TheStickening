use tray_icon::{TrayIcon, menu::MenuItem};

/// Common setup components across platforms
pub struct SetupComponents {
    pub tray_icon: TrayIcon,
    pub quit_menu_item: MenuItem,
}
