use crate::setupinterface::{SetupComponents, SetupInterface};
use tray_icon::{menu::MenuItem, TrayIcon};

/// Platform-specific app setup
pub fn setup_app() -> Result<AppSetup, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        let components = crate::setupwin::WindowsSetup::setup()?;
        Ok(AppSetup {
            tray_icon: components.tray_icon,
            quit_menu_item: components.quit_menu_item,
        })
    }
    #[cfg(target_os = "linux")]
    {
        let components = crate::setuplinux::LinuxSetup::setup()?;
        Ok(AppSetup {
            tray_icon: components.tray_icon,
            quit_menu_item: components.quit_menu_item,
        })
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        Err("Unsupported platform".into())
    }
}

/// Holds the application setup components
pub struct AppSetup {
    pub tray_icon: TrayIcon,
    pub quit_menu_item: MenuItem,
}
