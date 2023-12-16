use anyhow::Result;
use eframe::egui::IconData;
use tray_icon::Icon;

pub fn load_tray_icon(bytes: &[u8]) -> Result<Icon> {
    let icon = load_icon(bytes);
    let icon = Icon::from_rgba(icon.rgba, icon.width, icon.height)?;

    Ok(icon)
}

pub fn load_icon(bytes: &[u8]) -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(bytes)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
