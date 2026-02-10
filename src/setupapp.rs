use crate::menu::SetupComponents;
use tray_icon::{
    BadIcon, Icon, TrayIconBuilder,
    menu::{Menu, MenuItem},
};

pub fn setup() -> Result<SetupComponents, Box<dyn std::error::Error>> {
    let quit_menu_item = MenuItem::new("Quit", true, None);
    let menu = Menu::new();
    menu.append(&quit_menu_item)?;
    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("TheStickening")
        .with_title("TheStickening")
        .with_icon(make_icon().unwrap()) // if icon is not good
        // we can crash the app at startup
        .build()?;

    Ok(SetupComponents {
        tray_icon,
        quit_menu_item,
    })
}

fn make_icon() -> Result<tray_icon::Icon, BadIcon> {
    // Duplicated atm but will change later
    let width = 8;
    let height = 8;
    // I am something of an ascii artist myself (im not)
    let b = 255;
    let alpha: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, //
        0, b, b, b, b, b, b, 0, //
        0, 0, 0, 0, b, 0, 0, 0, //
        0, 0, 0, 0, b, 0, 0, 0, //
        0, 0, b, 0, b, 0, 0, 0, //
        0, 0, b, 0, b, 0, 0, 0, //
        0, 0, 0, b, b, 0, 0, 0, //
        0, 0, 0, 0, 0, 0, 0, 0, //
    ];
    let mut rgba: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);
    for a in alpha {
        rgba.push(0); // r
        rgba.push(0); // g
        rgba.push(0); // b
        rgba.push(a); // a
    }
    let icon = Icon::from_rgba(rgba, width, height)?;
    Ok(icon)
}
