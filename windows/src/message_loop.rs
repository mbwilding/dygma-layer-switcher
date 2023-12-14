#[cfg(target_os = "windows")]
use crate::collection;

#[cfg(target_os = "windows")]
use std::sync::Mutex;

#[cfg(target_os = "windows")]
use lazy_static::lazy_static;

#[cfg(target_os = "windows")]
use std::thread;

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HWND;

#[cfg(target_os = "windows")]
use windows::Win32::UI::Accessibility::HWINEVENTHOOK;

#[cfg(target_os = "windows")]
lazy_static! {
    static ref DEBOUNCER: Mutex<u32> = Mutex::new(0);
}

/// # Safety
///
/// WinAPI.
#[cfg(target_os = "windows")]
pub unsafe extern "system" fn get_focused_window_details(
    _h_win_event_hook: HWINEVENTHOOK,
    _event: u32,
    window_handle: HWND,
    _id_object: i32,
    _id_child: i32,
    _id_event_thread: u32,
    dwms_event_time: u32,
) {
    if window_handle.0 == 0 {
        return;
    }

    thread::spawn(move || {
        let mut debouncer = DEBOUNCER.lock().unwrap();
        let app_details = collection::hydrate(window_handle);

        if dwms_event_time == *debouncer {
            return;
        }

        *debouncer = dwms_event_time;
        common::layer::process(&app_details);
    });
}
