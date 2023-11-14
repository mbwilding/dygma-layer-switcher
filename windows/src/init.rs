#[cfg(target_os = "windows")]
use crate::message_loop::get_focused_window_details;

#[cfg(target_os = "windows")]
use tracing::{error, info};

#[cfg(target_os = "windows")]
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::HWND,
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            Accessibility::SetWinEventHook,
            WindowsAndMessaging::{GetMessageW, EVENT_OBJECT_FOCUS, MSG, WINEVENT_OUTOFCONTEXT},
        },
    },
};

#[cfg(target_os = "windows")]
pub fn start() {
    std::thread::spawn(|| {
        unsafe {
            info!("Attempting Hook");

            let module_handle = GetModuleHandleW(PCWSTR::null()).unwrap_or_else(|e| {
                error!("Failed to get module handle: {:?}", e);
                std::process::exit(1);
            });

            let _event_hook = SetWinEventHook(
                EVENT_OBJECT_FOCUS,
                EVENT_OBJECT_FOCUS,
                module_handle,
                Some(get_focused_window_details),
                0,
                0,
                WINEVENT_OUTOFCONTEXT,
            );

            info!("Hooked");

            let mut msg = MSG::default();

            loop {
                // Will wait here for a message, so no sleep is needed in the loop
                // This is a blocking call, but is required for the hook to work
                while GetMessageW(&mut msg, HWND(0), 0, 0).as_bool() {}
            }
        }
    });
}
