mod commands;
mod errors;
mod settings;
mod storage;

use crate::commands::focus::*;
use crate::settings::*;
use crate::storage::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let ctx = tauri::generate_context!();
    let mut builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    {
        let devtools = tauri_plugin_devtools::init();
        builder = builder.plugin(devtools);
    }

    #[cfg(not(debug_assertions))]
    {
        use tauri_plugin_log::fern::colors::ColoredLevelConfig;
        use tauri_plugin_log::{Builder, Target, TargetKind};

        let log_plugin = Builder::default()
            .targets([
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::LogDir { file_name: None }),
                Target::new(TargetKind::Webview),
            ])
            .with_colors(ColoredLevelConfig::default())
            .build();

        builder = builder.plugin(log_plugin);
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_os::init())
        .manage(Storage::default())
        .setup(|app| {
            settings(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            find_all_devices,
            connect,
            connect_first_available,
            disconnect,
            settings_get,
            settings_set,
            version,
            keymap_custom_get,
            keymap_custom_set,
            keymap_default_get,
            keymap_default_set,
            keymap_only_custom_get,
            keymap_only_custom_set,
            settings_default_layer_get,
            settings_default_layer_set,
            settings_valid,
            settings_version_get,
            settings_version_set,
            settings_crc,
            eeprom_contents_get,
            eeprom_contents_set,
            eeprom_free,
            upgrade_start,
            upgrade_is_ready,
            upgrade_neuron,
            upgrade_end,
            upgrade_keyscanner_is_connected,
            upgrade_keyscanner_is_bootloader,
            upgrade_keyscanner_begin,
            upgrade_keyscanner_is_ready,
            upgrade_keyscanner_get_info,
            upgrade_keyscanner_send_write,
            upgrade_keyscanner_finish,
            superkeys_map_get,
            superkeys_map_set,
            superkeys_wait_for_get,
            superkeys_wait_for_set,
            superkeys_timeout_get,
            superkeys_timeout_set,
            superkeys_repeat_get,
            superkeys_repeat_set,
            superkeys_hold_start_get,
            superkeys_hold_start_set,
            superkeys_overlap_get,
            superkeys_overlap_set,
            led_at_get,
            led_at_set,
            led_all,
            led_mode_get,
            led_mode_set,
            led_brightness_top_get,
            led_brightness_top_set,
            led_brightness_underglow_wired_get,
            led_brightness_underglow_wired_set,
            led_brightness_keys_wireless_get,
            led_brightness_keys_wireless_set,
            led_brightness_underglow_wireless_get,
            led_brightness_underglow_wireless_set,
            led_fade_get,
            led_fade_set,
            led_theme_get,
            led_theme_set,
            palette_rgb_get,
            palette_rgb_set,
            palette_rgbw_get,
            palette_rgbw_set,
            color_map_get,
            color_map_set,
            led_idle_true_sleep_get,
            led_idle_true_sleep_set,
            led_idle_true_sleep_time_get,
            led_idle_true_sleep_time_set,
            led_idle_time_limit_wired_get,
            led_idle_time_limit_wired_set,
            led_idle_time_limit_wireless_get,
            led_idle_time_limit_wireless_set,
            hardware_version_get,
            hardware_version_set,
            qukeys_hold_timeout_get,
            qukeys_hold_timeout_set,
            qukeys_overlap_threshold_get,
            qukeys_overlap_threshold_set,
            macros_map_get,
            macros_map_set,
            macros_trigger,
            macros_memory,
            help,
            mouse_speed_get,
            mouse_speed_set,
            mouse_delay_get,
            mouse_delay_set,
            mouse_acceleration_speed_get,
            mouse_acceleration_speed_set,
            mouse_acceleration_delay_get,
            mouse_acceleration_delay_set,
            mouse_wheel_speed_get,
            mouse_wheel_speed_set,
            mouse_wheel_delay_get,
            mouse_wheel_delay_set,
            mouse_speed_limit_get,
            mouse_speed_limit_set,
            layer_activate,
            layer_deactivate,
            layer_is_active,
            layer_move_to,
            layer_state,
            wireless_battery_level_left_get,
            wireless_battery_level_right_get,
            wireless_battery_status_left_get,
            wireless_battery_status_right_get,
            wireless_battery_saving_mode_get,
            wireless_battery_saving_mode_set,
            wireless_battery_force_read,
            wireless_rf_power_level_get,
            wireless_rf_power_level_set,
            wireless_rf_channel_hop_get,
            wireless_rf_channel_hop_set,
            wireless_rf_sync_pairing,
        ])
        .run(ctx)
        .expect("error while running tauri application");
}
