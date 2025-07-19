use log::debug;
use serde_json::json;
use std::collections::HashMap;
use tauri::App;
use tauri_plugin_store::StoreExt;

pub(crate) fn settings(app: &mut App) -> anyhow::Result<()> {
    // Create a new store or load the existing one
    // this also put the store in the app's resource table
    // so your following calls `store` calls (from both rust and js)
    // will reuse the same store
    //
    // Directory:
    //   macOS: ~/Library/Application Support/com.mbwilding.dygmalayerswitcher/settings.json
    //   windows: ~\AppData\Roaming\com.mbwilding.dygmalayerswitcher\settings.json
    let store = app.store("settings.json")?;

    let settings_defaults = HashMap::from([
        ("settings.version", json!(null)),
    ]);

    for (key, default_value) in &settings_defaults {
        if !store.has(key) {
            store.set(key.to_string(), default_value.clone());
        }
    }

    #[cfg(debug_assertions)]
    {
        let settings_keys: Vec<&str> = settings_defaults.keys().cloned().collect();

        debug!("Begin settings dump");
        for key in settings_keys {
            let value = store.get(key).unwrap_or_default();
            debug!("{}: {}", key, value);
        }
        debug!("End settings dump");
    }

    // Remove the store from the resource table
    // store.close_resource();

    Ok(())
}
