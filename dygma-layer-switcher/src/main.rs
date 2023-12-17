// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::DygmaLayerSwitcher;
use anyhow::Result;
use eframe::egui::ViewportBuilder;
use eframe::*;
use std::sync::Arc;
use std::{cell::RefCell, rc::Rc};
use tray_icon::menu::{Menu, MenuItem};
use tray_icon::TrayIconBuilder;

mod app;
mod helpers;
mod icon;
mod layer;
mod log;
mod single;
mod structs;
mod templates;
mod verbiage;
mod windows;

pub const ICON: &[u8] = include_bytes!("../../assets/icons/icon.ico");

pub fn main() -> Result<()> {
    single::check()?;

    let icon = icon::load_tray_icon(ICON)?;
    let mut _tray_icon = Rc::new(RefCell::new(None));
    let tray_rc = _tray_icon.clone();

    let tray_menu = Menu::new();
    tray_menu
        .append(&MenuItem::new("Quit", true, None))
        .expect("Failed to append menu item");

    run_native(
        verbiage::APP_NAME,
        NativeOptions {
            default_theme: Theme::Dark,
            // follow_system_theme: true,
            persist_window: true,
            centered: false,
            vsync: true,
            viewport: ViewportBuilder::default()
                .with_inner_size((400.0, 320.0))
                .with_resizable(true)
                .with_close_button(true)
                .with_minimize_button(true)
                .with_maximize_button(true)
                .with_icon(Arc::new(icon::load_icon(ICON))),
            ..Default::default()
        },
        Box::new(move |cc| {
            tray_rc.borrow_mut().replace(
                TrayIconBuilder::new()
                    .with_menu(Box::new(tray_menu))
                    .with_tooltip(verbiage::APP_NAME)
                    .with_icon(icon)
                    .build()
                    .unwrap(),
            );
            let app = DygmaLayerSwitcher::new(cc);
            log::init(app.logging);
            windows::start();
            Box::new(app)
        }),
    )?;

    Ok(())
}
