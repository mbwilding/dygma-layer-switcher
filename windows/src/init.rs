use crate::message_loop::get_focused_window_details;
use anyhow::Result;
use tracing::debug;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::HWND,
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            Accessibility::SetWinEventHook,
            WindowsAndMessaging::{
                DispatchMessageW, GetMessageW, TranslateMessage, EVENT_OBJECT_FOCUS, MSG,
                WINEVENT_OUTOFCONTEXT,
            },
        },
    },
};

pub fn start() -> Result<()> {
    unsafe {
        let h_instance = GetModuleHandleW(PCWSTR::null())?;

        let _event_hook = SetWinEventHook(
            EVENT_OBJECT_FOCUS,
            EVENT_OBJECT_FOCUS,
            h_instance,
            Some(get_focused_window_details),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        );

        debug!("Hooked");

        let mut msg = MSG::default();

        loop {
            // Will wait here for a message, so no sleep is needed in the loop
            while GetMessageW(&mut msg, HWND(0), 0, 0).as_bool() {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}
