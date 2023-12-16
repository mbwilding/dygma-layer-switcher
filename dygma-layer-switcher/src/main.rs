// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use eframe::egui::ViewportBuilder;
use eframe::*;
use std::{cell::RefCell, rc::Rc};
use tray_icon::menu::{Menu, MenuItem};
use tray_icon::TrayIconBuilder;

mod app;
mod helpers;
mod layer;
mod log;
mod single;
mod structs;
mod templates;
mod tray;
mod windows;

pub const TITLE: &str = "Dygma Layer Switcher";
pub const ICON: &[u8] = include_bytes!("../../assets/icons/icon.ico");

pub fn main() -> Result<()> {
    single::check()?;

    let icon = tray::load_tray_icon(ICON)?;
    let mut _tray_icon = Rc::new(RefCell::new(None));
    let tray_rc = _tray_icon.clone();

    let tray_menu = Menu::new();
    tray_menu
        .append(&MenuItem::new("Quit", true, None))
        .expect("Failed to append menu item");

    let options = NativeOptions {
        default_theme: Theme::Dark,
        follow_system_theme: true,
        persist_window: true,
        centered: false,
        vsync: true,
        ..Default::default()
    };

    run_native(
        "Dygma Layer Switcher",
        options,
        Box::new(move |cc| {
            tray_rc.borrow_mut().replace(
                TrayIconBuilder::new()
                    .with_menu(Box::new(tray_menu))
                    .with_tooltip(TITLE)
                    .with_icon(icon)
                    .build()
                    .unwrap(),
            );
            let app = app::DygmaLayerSwitcher::new(cc);
            log::init(app.logging);
            windows::start();
            Box::new(app)
        }),
    )?;

    Ok(())
}
