use crate::errors::*;
use crate::Storage;
use anyhow::{anyhow, Context, Error};
use dygma_focus::prelude::*;
use dygma_focus::{errors::FocusError, Focus};
use tauri::{command, Result, State};

fn with_focus<T, F>(storage: State<Storage>, func: F) -> Result<T>
where
    F: FnOnce(&mut Focus) -> std::result::Result<T, FocusError>,
{
    let mut focus_guard = storage.focus.lock().expect("Failed to lock Focus mutex");
    let focus = focus_guard
        .as_mut()
        .ok_or(BazecorError::NotConnectedError)
        .map_err(Error::new)?;
    match func(focus) {
        Ok(result) => Ok(result),
        Err(err) => {
            *focus_guard = None;
            Err(anyhow!(err).into())
        }
    }
}

#[command]
pub(crate) fn find_all_devices() -> Result<Vec<Device>> {
    Ok(Focus::find_all_devices()
        .context("Could not find any Dygma devices")?
        .into_iter()
        // For macOS as it will return 2 serial devices per dygma device
        .filter(|device| !device.serial_port.starts_with("/dev/cu."))
        .collect())
}

#[command]
pub(crate) fn connect(port: &str, storage: State<Storage>) -> Result<()> {
    *storage.focus.lock().expect("Failed to lock Focus mutex") =
        Some(Focus::new_via_port(port).context("Failed to create Focus via port")?);
    Ok(())
}

#[command]
pub(crate) fn connect_first_available(storage: State<Storage>) -> Result<()> {
    *storage.focus.lock().expect("Failed to lock Focus mutex") =
        Some(Focus::new_first_available().context("Failed to create Focus via first available")?);
    Ok(())
}

#[command]
pub(crate) fn disconnect(storage: State<Storage>) -> Result<()> {
    *storage.focus.lock().expect("Failed to lock Focus mutex") = None;
    Ok(())
}

#[command]
pub(crate) fn settings_get(storage: State<Storage>) -> Result<Settings> {
    with_focus(storage, |focus| focus.settings_get())
}

#[command]
pub(crate) fn settings_set(data: Settings, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.settings_set(&data))
}

#[command]
pub(crate) fn version(storage: State<Storage>) -> Result<String> {
    with_focus(storage, |focus| focus.version())
}

#[command]
pub(crate) fn keymap_custom_get(storage: State<Storage>) -> Result<Vec<u16>> {
    with_focus(storage, |focus| focus.keymap_custom_get())
}

#[command]
pub(crate) fn keymap_custom_set(data: Vec<u16>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.keymap_custom_set(&data))
}

#[command]
pub(crate) fn keymap_default_get(storage: State<Storage>) -> Result<Vec<u16>> {
    with_focus(storage, |focus| focus.keymap_default_get())
}

#[command]
pub(crate) fn keymap_default_set(data: Vec<u16>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.keymap_default_set(&data))
}

#[command]
pub(crate) fn keymap_only_custom_get(storage: State<Storage>) -> Result<bool> {
    with_focus(storage, |focus| focus.keymap_only_custom_get())
}

#[command]
pub(crate) fn keymap_only_custom_set(data: bool, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.keymap_only_custom_set(data))
}

#[command]
pub(crate) fn settings_default_layer_get(storage: State<Storage>) -> Result<u8> {
    with_focus(storage, |focus| focus.settings_default_layer_get())
}

#[command]
pub(crate) fn settings_default_layer_set(data: u8, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.settings_default_layer_set(data))
}

#[command]
pub(crate) fn settings_valid(storage: State<Storage>) -> Result<bool> {
    with_focus(storage, |focus| focus.settings_valid())
}

#[command]
pub(crate) fn settings_version_get(storage: State<Storage>) -> Result<String> {
    with_focus(storage, |focus| focus.settings_version_get())
}

#[command]
pub(crate) fn settings_version_set(data: String, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.settings_version_set(&data))
}

#[command]
pub(crate) fn settings_crc(storage: State<Storage>) -> Result<String> {
    with_focus(storage, |focus| focus.settings_crc())
}

#[command]
pub(crate) fn eeprom_contents_get(storage: State<Storage>) -> Result<String> {
    with_focus(storage, |focus| focus.eeprom_contents_get())
}

#[command]
pub(crate) fn eeprom_contents_set(data: String, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.eeprom_contents_set(&data))
}

#[command]
pub(crate) fn eeprom_free(storage: State<Storage>) -> Result<String> {
    with_focus(storage, |focus| focus.eeprom_free())
}

#[command]
pub(crate) fn upgrade_start(storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.upgrade_start())
}

#[command]
pub(crate) fn upgrade_is_ready(storage: State<Storage>) -> Result<bool> {
    with_focus(storage, |focus| focus.upgrade_is_ready())
}

#[command]
pub(crate) fn upgrade_neuron(storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.upgrade_neuron())
}

#[command]
pub(crate) fn upgrade_end(storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.upgrade_end())
}

#[command]
pub(crate) fn upgrade_keyscanner_is_connected(data: Side, storage: State<Storage>) -> Result<bool> {
    with_focus(storage, |focus| focus.upgrade_keyscanner_is_connected(data))
}

#[command]
pub(crate) fn upgrade_keyscanner_is_bootloader(
    data: Side,
    storage: State<Storage>,
) -> Result<bool> {
    with_focus(storage, |focus| {
        focus.upgrade_keyscanner_is_bootloader(data)
    })
}

#[command]
pub(crate) fn upgrade_keyscanner_begin(data: Side, storage: State<Storage>) -> Result<bool> {
    with_focus(storage, |focus| focus.upgrade_keyscanner_begin(data))
}

#[command]
pub(crate) fn upgrade_keyscanner_is_ready(storage: State<Storage>) -> Result<bool> {
    with_focus(storage, |focus| focus.upgrade_keyscanner_is_ready())
}

#[command]
pub(crate) fn upgrade_keyscanner_get_info(storage: State<Storage>) -> Result<String> {
    with_focus(storage, |focus| focus.upgrade_keyscanner_get_info())
}

#[command]
pub(crate) fn upgrade_keyscanner_send_write(storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.upgrade_keyscanner_send_write())
}

#[command]
pub(crate) fn upgrade_keyscanner_finish(storage: State<Storage>) -> Result<String> {
    with_focus(storage, |focus| focus.upgrade_keyscanner_finish())
}

#[command]
pub(crate) fn superkeys_map_get(storage: State<Storage>) -> Result<Vec<u16>> {
    with_focus(storage, |focus| focus.superkeys_map_get())
}

#[command]
pub(crate) fn superkeys_map_set(data: Vec<u16>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.superkeys_map_set(&data))
}

#[command]
pub(crate) fn superkeys_wait_for_get(storage: State<Storage>) -> Result<u16> {
    with_focus(storage, |focus| focus.superkeys_wait_for_get())
}

#[command]
pub(crate) fn superkeys_wait_for_set(data: u16, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.superkeys_wait_for_set(data))
}

#[command]
pub(crate) fn superkeys_timeout_get(storage: State<Storage>) -> Result<u16> {
    with_focus(storage, |focus| focus.superkeys_timeout_get())
}

#[command]
pub(crate) fn superkeys_timeout_set(data: u16, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.superkeys_timeout_set(data))
}

#[command]
pub(crate) fn superkeys_repeat_get(storage: State<Storage>) -> Result<u16> {
    with_focus(storage, |focus| focus.superkeys_repeat_get())
}

#[command]
pub(crate) fn superkeys_repeat_set(data: u16, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.superkeys_repeat_set(data))
}

#[command]
pub(crate) fn superkeys_hold_start_get(storage: State<Storage>) -> Result<u16> {
    with_focus(storage, |focus| focus.superkeys_hold_start_get())
}

#[command]
pub(crate) fn superkeys_hold_start_set(data: u16, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.superkeys_hold_start_set(data))
}

#[command]
pub(crate) fn superkeys_overlap_get(storage: State<Storage>) -> Result<u8> {
    with_focus(storage, |focus| focus.superkeys_overlap_get())
}

#[command]
pub(crate) fn superkeys_overlap_set(data: u8, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.superkeys_overlap_set(data))
}

#[command]
pub(crate) fn led_at_get(data: u8, storage: State<Storage>) -> Result<RGB> {
    with_focus(storage, |focus| focus.led_at_get(data))
}

#[command]
pub(crate) fn led_at_set(led: u8, data: RGB, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.led_at_set(led, &data))
}

#[command]
pub(crate) fn led_all(data: RGB, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.led_all(&data))
}

#[command]
pub(crate) fn led_mode_get(storage: State<Storage>) -> Result<LedMode> {
    with_focus(storage, |focus| focus.led_mode_get())
}

#[command]
pub(crate) fn led_mode_set(data: LedMode, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.led_mode_set(data))
}

#[command]
pub(crate) fn led_brightness_top_get(storage: State<Storage>) -> Result<u8> {
    with_focus(storage, |focus| focus.led_brightness_top_get())
}

#[command]
pub(crate) fn led_brightness_top_set(data: u8, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.led_brightness_top_set(data))
}

#[command]
pub(crate) fn led_brightness_underglow_wired_get(storage: State<Storage>) -> Result<u8> {
    with_focus(storage, |focus| focus.led_brightness_underglow_wired_get())
}

#[command]
pub(crate) fn led_brightness_underglow_wired_set(data: u8, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| {
        focus.led_brightness_underglow_wired_set(data)
    })
}

#[command]
pub(crate) fn led_brightness_keys_wireless_get(storage: State<Storage>) -> Result<u8> {
    with_focus(storage, |focus| focus.led_brightness_keys_wireless_get())
}

#[command]
pub(crate) fn led_brightness_keys_wireless_set(data: u8, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| {
        focus.led_brightness_keys_wireless_set(data)
    })
}

#[command]
pub(crate) fn led_brightness_underglow_wireless_get(storage: State<Storage>) -> Result<u8> {
    with_focus(storage, |focus| {
        focus.led_brightness_underglow_wireless_get()
    })
}

#[command]
pub(crate) fn led_brightness_underglow_wireless_set(
    data: u8,
    storage: State<Storage>,
) -> Result<()> {
    with_focus(storage, |focus| {
        focus.led_brightness_underglow_wireless_set(data)
    })
}

#[command]
pub(crate) fn led_fade_get(storage: State<Storage>) -> Result<u16> {
    with_focus(storage, |focus| focus.led_fade_get())
}

#[command]
pub(crate) fn led_fade_set(data: u16, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.led_fade_set(data))
}

#[command]
pub(crate) fn led_theme_get(storage: State<Storage>) -> Result<Vec<RGB>> {
    with_focus(storage, |focus| focus.led_theme_get())
}

#[command]
pub(crate) fn led_theme_set(data: Vec<RGB>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.led_theme_set(&data))
}

#[command]
pub(crate) fn palette_rgb_get(storage: State<Storage>) -> Result<Vec<RGB>> {
    with_focus(storage, |focus| focus.palette_rgb_get())
}

#[command]
pub(crate) fn palette_rgb_set(data: Vec<RGB>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.palette_rgb_set(&data))
}

#[command]
pub(crate) fn palette_rgbw_get(storage: State<Storage>) -> Result<Vec<RGBW>> {
    with_focus(storage, |focus| focus.palette_rgbw_get())
}

#[command]
pub(crate) fn palette_rgbw_set(data: Vec<RGBW>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.palette_rgbw_set(&data))
}

#[command]
pub(crate) fn color_map_get(storage: State<Storage>) -> Result<Vec<u8>> {
    with_focus(storage, |focus| focus.color_map_get())
}

#[command]
pub(crate) fn color_map_set(data: Vec<u8>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.color_map_set(&data))
}

#[command]
pub(crate) fn led_idle_true_sleep_get(storage: State<Storage>) -> Result<bool> {
    with_focus(storage, |focus| focus.led_idle_true_sleep_get())
}

#[command]
pub(crate) fn led_idle_true_sleep_set(data: bool, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.led_idle_true_sleep_set(data))
}

#[command]
pub(crate) fn led_idle_true_sleep_time_get(storage: State<Storage>) -> Result<u16> {
    with_focus(storage, |focus| focus.led_idle_true_sleep_time_get())
}

#[command]
pub(crate) fn led_idle_true_sleep_time_set(data: u16, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.led_idle_true_sleep_time_set(data))
}

#[command]
pub(crate) fn led_idle_time_limit_wired_get(storage: State<Storage>) -> Result<u16> {
    with_focus(storage, |focus| focus.led_idle_time_limit_wired_get())
}

#[command]
pub(crate) fn led_idle_time_limit_wired_set(data: u16, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.led_idle_time_limit_wired_set(data))
}

#[command]
pub(crate) fn led_idle_time_limit_wireless_get(storage: State<Storage>) -> Result<u16> {
    with_focus(storage, |focus| focus.led_idle_time_limit_wireless_get())
}

#[command]
pub(crate) fn led_idle_time_limit_wireless_set(data: u16, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| {
        focus.led_idle_time_limit_wireless_set(data)
    })
}

#[command]
pub(crate) fn hardware_version_get(storage: State<Storage>) -> Result<String> {
    with_focus(storage, |focus| focus.hardware_version_get())
}

#[command]
pub(crate) fn hardware_version_set(data: String, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.hardware_version_set(&data))
}

#[command]
pub(crate) fn qukeys_hold_timeout_get(storage: State<Storage>) -> Result<u16> {
    with_focus(storage, |focus| focus.qukeys_hold_timeout_get())
}

#[command]
pub(crate) fn qukeys_hold_timeout_set(data: u16, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.qukeys_hold_timeout_set(data))
}

#[command]
pub(crate) fn qukeys_overlap_threshold_get(storage: State<Storage>) -> Result<u16> {
    with_focus(storage, |focus| focus.qukeys_overlap_threshold_get())
}

#[command]
pub(crate) fn qukeys_overlap_threshold_set(data: u16, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.qukeys_overlap_threshold_set(data))
}

#[command]
pub(crate) fn macros_map_get(storage: State<Storage>) -> Result<Vec<u8>> {
    with_focus(storage, |focus| focus.macros_map_get())
}

#[command]
pub(crate) fn macros_map_set(data: Vec<u8>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.macros_map_set(&data))
}

#[command]
pub(crate) fn macros_trigger(data: u8, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.macros_trigger(data))
}

#[command]
pub(crate) fn macros_memory(storage: State<Storage>) -> Result<u16> {
    with_focus(storage, |focus| focus.macros_memory())
}

#[command]
pub(crate) fn help(storage: State<Storage>) -> Result<Vec<String>> {
    with_focus(storage, |focus| focus.help())
}

#[command]
pub(crate) fn mouse_speed_get(storage: State<Storage>) -> Result<u8> {
    with_focus(storage, |focus| focus.mouse_speed_get())
}

#[command]
pub(crate) fn mouse_speed_set(data: u8, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.mouse_speed_set(data))
}

#[command]
pub(crate) fn mouse_delay_get(storage: State<Storage>) -> Result<u16> {
    with_focus(storage, |focus| focus.mouse_delay_get())
}

#[command]
pub(crate) fn mouse_delay_set(data: u16, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.mouse_delay_set(data))
}

#[command]
pub(crate) fn mouse_acceleration_speed_get(storage: State<Storage>) -> Result<u8> {
    with_focus(storage, |focus| focus.mouse_acceleration_speed_get())
}

#[command]
pub(crate) fn mouse_acceleration_speed_set(data: u8, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.mouse_acceleration_speed_set(data))
}

#[command]
pub(crate) fn mouse_acceleration_delay_get(storage: State<Storage>) -> Result<u16> {
    with_focus(storage, |focus| focus.mouse_acceleration_delay_get())
}

#[command]
pub(crate) fn mouse_acceleration_delay_set(data: u16, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.mouse_acceleration_delay_set(data))
}

#[command]
pub(crate) fn mouse_wheel_speed_get(storage: State<Storage>) -> Result<u8> {
    with_focus(storage, |focus| focus.mouse_wheel_speed_get())
}

#[command]
pub(crate) fn mouse_wheel_speed_set(data: u8, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.mouse_wheel_speed_set(data))
}

#[command]
pub(crate) fn mouse_wheel_delay_get(storage: State<Storage>) -> Result<u16> {
    with_focus(storage, |focus| focus.mouse_wheel_delay_get())
}

#[command]
pub(crate) fn mouse_wheel_delay_set(data: u16, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.mouse_wheel_delay_set(data))
}

#[command]
pub(crate) fn mouse_speed_limit_get(storage: State<Storage>) -> Result<u8> {
    with_focus(storage, |focus| focus.mouse_speed_limit_get())
}

#[command]
pub(crate) fn mouse_speed_limit_set(data: u8, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.mouse_speed_limit_set(data))
}

#[command]
pub(crate) fn layer_activate(data: u8, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.layer_activate(data))
}

#[command]
pub(crate) fn layer_deactivate(data: Option<u8>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.layer_deactivate(data))
}

#[command]
pub(crate) fn layer_is_active(data: u8, storage: State<Storage>) -> Result<bool> {
    with_focus(storage, |focus| focus.layer_is_active(data))
}

#[command]
pub(crate) fn layer_move_to(data: u8, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.layer_move_to(data))
}

#[command]
pub(crate) fn layer_state(storage: State<Storage>) -> Result<Vec<bool>> {
    with_focus(storage, |focus| focus.layer_state())
}

#[command]
pub(crate) fn wireless_battery_level_left_get(storage: State<Storage>) -> Result<u8> {
    with_focus(storage, |focus| focus.wireless_battery_level_left_get())
}

#[command]
pub(crate) fn wireless_battery_level_right_get(storage: State<Storage>) -> Result<u8> {
    with_focus(storage, |focus| focus.wireless_battery_level_right_get())
}

#[command]
pub(crate) fn wireless_battery_status_left_get(storage: State<Storage>) -> Result<u8> {
    with_focus(storage, |focus| focus.wireless_battery_status_left_get())
}

#[command]
pub(crate) fn wireless_battery_status_right_get(storage: State<Storage>) -> Result<u8> {
    with_focus(storage, |focus| focus.wireless_battery_status_right_get())
}

#[command]
pub(crate) fn wireless_battery_saving_mode_get(storage: State<Storage>) -> Result<bool> {
    with_focus(storage, |focus| focus.wireless_battery_saving_mode_get())
}

#[command]
pub(crate) fn wireless_battery_saving_mode_set(data: bool, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| {
        focus.wireless_battery_saving_mode_set(data)
    })
}

#[command]
pub(crate) fn wireless_battery_force_read(storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.wireless_battery_force_read())
}

#[command]
pub(crate) fn wireless_rf_power_level_get(storage: State<Storage>) -> Result<WirelessPowerMode> {
    with_focus(storage, |focus| focus.wireless_rf_power_level_get())
}

#[command]
pub(crate) fn wireless_rf_power_level_set(
    data: WirelessPowerMode,
    storage: State<Storage>,
) -> Result<()> {
    with_focus(storage, |focus| focus.wireless_rf_power_level_set(data))
}

#[command]
pub(crate) fn wireless_rf_channel_hop_get(storage: State<Storage>) -> Result<bool> {
    with_focus(storage, |focus| focus.wireless_rf_channel_hop_get())
}

#[command]
pub(crate) fn wireless_rf_channel_hop_set(data: bool, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.wireless_rf_channel_hop_set(data))
}

#[command]
pub(crate) fn wireless_rf_sync_pairing(storage: State<Storage>) -> Result<bool> {
    with_focus(storage, |focus| focus.wireless_rf_sync_pairing())
}
