use crate::setupinterface::{SetupComponents, SetupInterface};
use tray_icon::{
    menu::{Menu, MenuItem},
    TrayIconBuilder,
};

/// Linux-specific setup implementation
pub struct LinuxSetup;

impl SetupInterface for LinuxSetup {
    fn setup() -> Result<SetupComponents, Box<dyn std::error::Error>> {
        let quit_menu_item = MenuItem::new("Quit", true, None);
        let menu = Menu::new();
        menu.append(&quit_menu_item)?;

        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("TheStickening")
            .with_title("TheStickening")
            .build()?;

        Ok(SetupComponents {
            tray_icon,
            quit_menu_item,
        })
    }
}
