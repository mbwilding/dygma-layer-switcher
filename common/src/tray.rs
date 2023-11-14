use anyhow::{Context, Result};
use std::path::Path;
use tracing::trace;
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, TrayIconBuilder, TrayIconEvent};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

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
    let tray_menu = Menu::new();

    let item_quit = MenuItem::new("Quit", true, None);
    tray_menu
        .append(&item_quit)
        .context("Failed to append menu item")?;

    // Since winit doesn't use gtk on Linux, and we need gtk for
    // the tray icon to show up, we need to spawn a thread
    // where we initialize gtk and create the tray_icon
    #[cfg(target_os = "linux")]
    std::thread::spawn(|| {
        gtk::init()?;
        let _tray_icon = TrayIconBuilder::new()
            .with_tooltip(TITLE)
            .with_menu(Box::new(tray_menu))
            .with_icon(icon)
            .build()?;

        gtk::main();
    });

    #[cfg(not(target_os = "linux"))]
    let _tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip(TITLE)
            .with_icon(icon)
            .build()?,
    );

    let event_loop = EventLoopBuilder::new().build()?;

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

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
