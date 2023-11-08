use crate::app::App;
use crate::layer;
use crate::window_data::{get_exe_name, get_window_title};
use tracing::{debug, error, info};
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

    debug!("Window handle: {:?}", window_handle.0);

    let window_title = get_window_title(window_handle);

    let exe_name = match get_exe_name(window_handle) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to get the executable name: {:?}", e);
            String::new()
        }
    };

    let window_details = App {
        window_title,
        exe_name,
    };

    info!("Window details: {:#?}", window_details);

    layer::process(&window_details);
}
