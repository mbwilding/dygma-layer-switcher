use anyhow::{Context, Result};
use std::path::Path;
use tracing::trace;
use tray_icon::menu::{Menu, MenuEvent, MenuEventReceiver, MenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent, TrayIconEventReceiver};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};

const TITLE: &str = "Dygma Layer Switcher";

fn load_icon(path: &Path) -> Result<Icon> {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)?.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height)?;

    Ok(icon)
}

pub fn load() -> Result<()> {
    let icon_path = "assets/icons/icon.ico";
    let icon =
        load_icon(Path::new(&icon_path)).context(format!("Could not find icon: {}", &icon_path))?;

    let event_loop = EventLoopBuilder::new().build()?;
    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    let item_quit = MenuItem::new("Quit", true, None);

    #[cfg(target_os = "windows")]
    {
        let tray_menu = Menu::new();
        tray_menu
            .append(&item_quit)
            .context("Failed to append menu item")?;

        let _tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip(TITLE)
            .with_icon(icon)
            .build()?;

        tray_loop(event_loop, menu_channel, tray_channel, item_quit)?;
    }

    #[cfg(target_os = "linux")]
    std::thread::spawn(move || {
        gtk::init().unwrap();

        let _tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(Menu::new()))
            .with_tooltip(TITLE)
            .with_icon(icon)
            .build()
            .unwrap();

        gtk::main();

        tray_loop(event_loop, menu_channel, tray_channel, item_quit)?;
    });

    Ok(())
}

fn tray_loop(
    event_loop: EventLoop<()>,
    menu_channel: &MenuEventReceiver,
    tray_channel: &TrayIconEventReceiver,
    item_quit: MenuItem,
) -> Result<()> {
    event_loop.run(move |_event, event_loop| {
        event_loop.set_control_flow(ControlFlow::Wait);

        if let Ok(event) = tray_channel.try_recv() {
            trace!("{:#?}", event);
        }

        if let Ok(event) = menu_channel.try_recv() {
            trace!("{:#?}", event);

            if event.id == item_quit.id() {
                event_loop.exit();
            }
        }
    })?;

    Ok(())
}
