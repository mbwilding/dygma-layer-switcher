import { invoke } from '@tauri-apps/api/core';
import { Settings } from "./types";
import { Side } from "./types";
import { RGB } from "./types";
import { LedMode } from "./types";
import { RGBW } from "./types";
import { WirelessPowerMode } from "./types";

export async function find_all_devices() {
  return await invoke('find_all_devices', {});
}

export async function connect(port: any) {
  return await invoke('connect', { port });
}

export async function connect_first_available() {
  return await invoke('connect_first_available', {});
}

export async function disconnect() {
  return await invoke('disconnect', {});
}

export async function settings_get() {
  return await invoke('settings_get', {});
}

export async function settings_set(data: Settings) {
  return await invoke('settings_set', { data });
}

export async function version() {
  return await invoke('version', {});
}

export async function keymap_custom_get() {
  return await invoke('keymap_custom_get', {});
}

export async function keymap_custom_set(data: Array<number>) {
  return await invoke('keymap_custom_set', { data });
}

export async function keymap_default_get() {
  return await invoke('keymap_default_get', {});
}

export async function keymap_default_set(data: Array<number>) {
  return await invoke('keymap_default_set', { data });
}

export async function keymap_only_custom_get() {
  return await invoke('keymap_only_custom_get', {});
}

export async function keymap_only_custom_set(data: boolean) {
  return await invoke('keymap_only_custom_set', { data });
}

export async function settings_default_layer_get() {
  return await invoke('settings_default_layer_get', {});
}

export async function settings_default_layer_set(data: number) {
  return await invoke('settings_default_layer_set', { data });
}

export async function settings_valid() {
  return await invoke('settings_valid', {});
}

export async function settings_version_get() {
  return await invoke('settings_version_get', {});
}

export async function settings_version_set(data: string) {
  return await invoke('settings_version_set', { data });
}

export async function settings_crc() {
  return await invoke('settings_crc', {});
}

export async function eeprom_contents_get() {
  return await invoke('eeprom_contents_get', {});
}

export async function eeprom_contents_set(data: string) {
  return await invoke('eeprom_contents_set', { data });
}

export async function eeprom_free() {
  return await invoke('eeprom_free', {});
}

export async function upgrade_start() {
  return await invoke('upgrade_start', {});
}

export async function upgrade_is_ready() {
  return await invoke('upgrade_is_ready', {});
}

export async function upgrade_neuron() {
  return await invoke('upgrade_neuron', {});
}

export async function upgrade_end() {
  return await invoke('upgrade_end', {});
}

export async function upgrade_keyscanner_is_connected(data: Side) {
  return await invoke('upgrade_keyscanner_is_connected', { data });
}

export async function upgrade_keyscanner_is_bootloader(data: Side) {
  return await invoke('upgrade_keyscanner_is_bootloader', { data });
}

export async function upgrade_keyscanner_begin(data: Side) {
  return await invoke('upgrade_keyscanner_begin', { data });
}

export async function upgrade_keyscanner_is_ready() {
  return await invoke('upgrade_keyscanner_is_ready', {});
}

export async function upgrade_keyscanner_get_info() {
  return await invoke('upgrade_keyscanner_get_info', {});
}

export async function upgrade_keyscanner_send_write() {
  return await invoke('upgrade_keyscanner_send_write', {});
}

export async function upgrade_keyscanner_finish() {
  return await invoke('upgrade_keyscanner_finish', {});
}

export async function superkeys_map_get() {
  return await invoke('superkeys_map_get', {});
}

export async function superkeys_map_set(data: Array<number>) {
  return await invoke('superkeys_map_set', { data });
}

export async function superkeys_wait_for_get() {
  return await invoke('superkeys_wait_for_get', {});
}

export async function superkeys_wait_for_set(data: number) {
  return await invoke('superkeys_wait_for_set', { data });
}

export async function superkeys_timeout_get() {
  return await invoke('superkeys_timeout_get', {});
}

export async function superkeys_timeout_set(data: number) {
  return await invoke('superkeys_timeout_set', { data });
}

export async function superkeys_repeat_get() {
  return await invoke('superkeys_repeat_get', {});
}

export async function superkeys_repeat_set(data: number) {
  return await invoke('superkeys_repeat_set', { data });
}

export async function superkeys_hold_start_get() {
  return await invoke('superkeys_hold_start_get', {});
}

export async function superkeys_hold_start_set(data: number) {
  return await invoke('superkeys_hold_start_set', { data });
}

export async function superkeys_overlap_get() {
  return await invoke('superkeys_overlap_get', {});
}

export async function superkeys_overlap_set(data: number) {
  return await invoke('superkeys_overlap_set', { data });
}

export async function led_at_get(data: number) {
  return await invoke('led_at_get', { data });
}

export async function led_at_set(led: number, data: RGB) {
  return await invoke('led_at_set', { led, data });
}

export async function led_all(data: RGB) {
  return await invoke('led_all', { data });
}

export async function led_mode_get() {
  return await invoke('led_mode_get', {});
}

export async function led_mode_set(data: LedMode) {
  return await invoke('led_mode_set', { data });
}

export async function led_brightness_top_get() {
  return await invoke('led_brightness_top_get', {});
}

export async function led_brightness_top_set(data: number) {
  return await invoke('led_brightness_top_set', { data });
}

export async function led_brightness_underglow_wired_get() {
  return await invoke('led_brightness_underglow_wired_get', {});
}

export async function led_brightness_underglow_wired_set(data: number) {
  return await invoke('led_brightness_underglow_wired_set', { data });
}

export async function led_brightness_keys_wireless_get() {
  return await invoke('led_brightness_keys_wireless_get', {});
}

export async function led_brightness_keys_wireless_set(data: number) {
  return await invoke('led_brightness_keys_wireless_set', { data });
}

export async function led_brightness_underglow_wireless_get() {
  return await invoke('led_brightness_underglow_wireless_get', {});
}

export async function led_brightness_underglow_wireless_set(data: number) {
  return await invoke('led_brightness_underglow_wireless_set', { data });
}

export async function led_fade_get() {
  return await invoke('led_fade_get', {});
}

export async function led_fade_set(data: number) {
  return await invoke('led_fade_set', { data });
}

export async function led_theme_get() {
  return await invoke('led_theme_get', {});
}

export async function led_theme_set(data: Array<RGB>) {
  return await invoke('led_theme_set', { data });
}

export async function palette_rgb_get() {
  return await invoke('palette_rgb_get', {});
}

export async function palette_rgb_set(data: Array<RGB>) {
  return await invoke('palette_rgb_set', { data });
}

export async function palette_rgbw_get() {
  return await invoke('palette_rgbw_get', {});
}

export async function palette_rgbw_set(data: Array<RGBW>) {
  return await invoke('palette_rgbw_set', { data });
}

export async function color_map_get() {
  return await invoke('color_map_get', {});
}

export async function color_map_set(data: Array<number>) {
  return await invoke('color_map_set', { data });
}

export async function led_idle_true_sleep_get() {
  return await invoke('led_idle_true_sleep_get', {});
}

export async function led_idle_true_sleep_set(data: boolean) {
  return await invoke('led_idle_true_sleep_set', { data });
}

export async function led_idle_true_sleep_time_get() {
  return await invoke('led_idle_true_sleep_time_get', {});
}

export async function led_idle_true_sleep_time_set(data: number) {
  return await invoke('led_idle_true_sleep_time_set', { data });
}

export async function led_idle_time_limit_wired_get() {
  return await invoke('led_idle_time_limit_wired_get', {});
}

export async function led_idle_time_limit_wired_set(data: number) {
  return await invoke('led_idle_time_limit_wired_set', { data });
}

export async function led_idle_time_limit_wireless_get() {
  return await invoke('led_idle_time_limit_wireless_get', {});
}

export async function led_idle_time_limit_wireless_set(data: number) {
  return await invoke('led_idle_time_limit_wireless_set', { data });
}

export async function hardware_version_get() {
  return await invoke('hardware_version_get', {});
}

export async function hardware_version_set(data: string) {
  return await invoke('hardware_version_set', { data });
}

export async function qukeys_hold_timeout_get() {
  return await invoke('qukeys_hold_timeout_get', {});
}

export async function qukeys_hold_timeout_set(data: number) {
  return await invoke('qukeys_hold_timeout_set', { data });
}

export async function qukeys_overlap_threshold_get() {
  return await invoke('qukeys_overlap_threshold_get', {});
}

export async function qukeys_overlap_threshold_set(data: number) {
  return await invoke('qukeys_overlap_threshold_set', { data });
}

export async function macros_map_get() {
  return await invoke('macros_map_get', {});
}

export async function macros_map_set(data: Array<number>) {
  return await invoke('macros_map_set', { data });
}

export async function macros_trigger(data: number) {
  return await invoke('macros_trigger', { data });
}

export async function macros_memory() {
  return await invoke('macros_memory', {});
}

export async function help() {
  return await invoke('help', {});
}

export async function mouse_speed_get() {
  return await invoke('mouse_speed_get', {});
}

export async function mouse_speed_set(data: number) {
  return await invoke('mouse_speed_set', { data });
}

export async function mouse_delay_get() {
  return await invoke('mouse_delay_get', {});
}

export async function mouse_delay_set(data: number) {
  return await invoke('mouse_delay_set', { data });
}

export async function mouse_acceleration_speed_get() {
  return await invoke('mouse_acceleration_speed_get', {});
}

export async function mouse_acceleration_speed_set(data: number) {
  return await invoke('mouse_acceleration_speed_set', { data });
}

export async function mouse_acceleration_delay_get() {
  return await invoke('mouse_acceleration_delay_get', {});
}

export async function mouse_acceleration_delay_set(data: number) {
  return await invoke('mouse_acceleration_delay_set', { data });
}

export async function mouse_wheel_speed_get() {
  return await invoke('mouse_wheel_speed_get', {});
}

export async function mouse_wheel_speed_set(data: number) {
  return await invoke('mouse_wheel_speed_set', { data });
}

export async function mouse_wheel_delay_get() {
  return await invoke('mouse_wheel_delay_get', {});
}

export async function mouse_wheel_delay_set(data: number) {
  return await invoke('mouse_wheel_delay_set', { data });
}

export async function mouse_speed_limit_get() {
  return await invoke('mouse_speed_limit_get', {});
}

export async function mouse_speed_limit_set(data: number) {
  return await invoke('mouse_speed_limit_set', { data });
}

export async function layer_activate(data: number) {
  return await invoke('layer_activate', { data });
}

export async function layer_deactivate(data: number | undefined) {
  return await invoke('layer_deactivate', { data });
}

export async function layer_is_active(data: number) {
  return await invoke('layer_is_active', { data });
}

export async function layer_move_to(data: number) {
  return await invoke('layer_move_to', { data });
}

export async function layer_state() {
  return await invoke('layer_state', {});
}

export async function wireless_battery_level_left_get() {
  return await invoke('wireless_battery_level_left_get', {});
}

export async function wireless_battery_level_right_get() {
  return await invoke('wireless_battery_level_right_get', {});
}

export async function wireless_battery_status_left_get() {
  return await invoke('wireless_battery_status_left_get', {});
}

export async function wireless_battery_status_right_get() {
  return await invoke('wireless_battery_status_right_get', {});
}

export async function wireless_battery_saving_mode_get() {
  return await invoke('wireless_battery_saving_mode_get', {});
}

export async function wireless_battery_saving_mode_set(data: boolean) {
  return await invoke('wireless_battery_saving_mode_set', { data });
}

export async function wireless_battery_force_read() {
  return await invoke('wireless_battery_force_read', {});
}

export async function wireless_rf_power_level_get() {
  return await invoke('wireless_rf_power_level_get', {});
}

export async function wireless_rf_power_level_set(data: WirelessPowerMode) {
  return await invoke('wireless_rf_power_level_set', { data });
}

export async function wireless_rf_channel_hop_get() {
  return await invoke('wireless_rf_channel_hop_get', {});
}

export async function wireless_rf_channel_hop_set(data: boolean) {
  return await invoke('wireless_rf_channel_hop_set', { data });
}

export async function wireless_rf_sync_pairing() {
  return await invoke('wireless_rf_sync_pairing', {});
}