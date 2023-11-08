use crate::layer;
use std::thread;
use tracing::debug;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Accessibility::{UnhookWinEvent, HWINEVENTHOOK};

pub struct EventHook(pub HWINEVENTHOOK);

impl Drop for EventHook {
    fn drop(&mut self) {
        unsafe {
            UnhookWinEvent(self.0);
        }
        debug!("Unhooked");
    }
}

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
        let app = layer::collect_info(window_handle);
        common::serial::process(&app);
    });
}
