use libappindicator::{AppIndicator, AppIndicatorStatus};
use gtk::prelude::*;
use std::path::Path;


fn main() -> Result<(), String>{
    gtk::init().unwrap();
    let mut indicator = AppIndicator::new("Test", "");
    indicator.set_status(AppIndicatorStatus::Active);
    indicator.set_icon_full("rust-logo-64x64-blk", "icon");
    let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    indicator.set_icon_theme_path(icon_path.to_str().unwrap());

    let mut menu = gtk::Menu::new();
    let mi = gtk::MenuItem::with_label("Quit");

    mi.connect_activate(|_| {
        gtk::main_quit();
    });
    menu.append(&mi);
    indicator.set_menu(&mut menu);
    menu.show_all();
    gtk::main();
    return Ok(());
}

