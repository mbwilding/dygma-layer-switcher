#[cfg(target_os = "windows")]
use crate::collection;

#[cfg(target_os = "windows")]
use std::sync::atomic::AtomicU32;

#[cfg(target_os = "windows")]
use std::sync::atomic::Ordering;

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HWND;

#[cfg(target_os = "windows")]
use windows::Win32::UI::Accessibility::HWINEVENTHOOK;

#[cfg(target_os = "windows")]
static DEBOUNCER: AtomicU32 = AtomicU32::new(0);

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

    if dwms_event_time == DEBOUNCER.load(Ordering::SeqCst) {
        return;
    }

    DEBOUNCER.store(dwms_event_time, Ordering::SeqCst);

    let app_details = collection::hydrate(window_handle);
    common::layer::process(&app_details);
}
