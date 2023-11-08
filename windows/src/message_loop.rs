use std::thread;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Accessibility::HWINEVENTHOOK;
use crate::window_data;

pub unsafe extern "system" fn get_focused_window_details(
    _h_win_event_hook: HWINEVENTHOOK,
    _event: u32,
    window_handle: HWND,
    _id_object: i32,
    _id_child: i32,
    _id_event_thread: u32,
    _dwms_event_time: u32,
) {
    if window_handle.0 == 0 {
        return;
    }

    thread::spawn(move || {
        let app = window_data::hydrate(window_handle);
        common::serial::process(&app);
    });
}
