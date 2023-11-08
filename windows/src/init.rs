use crate::message_loop::{get_focused_window_details, EventHook};
use anyhow::Result;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::flag;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::debug;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::HWND,
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            Accessibility::SetWinEventHook,
            WindowsAndMessaging::{
                DispatchMessageW, GetMessageW, PostQuitMessage, TranslateMessage,
                EVENT_OBJECT_FOCUS, MSG, WINEVENT_OUTOFCONTEXT,
            },
        },
    },
};

pub fn start() -> Result<()> {
    let is_terminating = Arc::new(AtomicBool::new(false));

    for &signal in TERM_SIGNALS {
        flag::register_conditional_shutdown(signal, 0, Arc::clone(&is_terminating))?;
        flag::register(signal, Arc::clone(&is_terminating))?;
    }

    unsafe {
        let h_instance = GetModuleHandleW(PCWSTR::null())?;

        let event_hook = SetWinEventHook(
            EVENT_OBJECT_FOCUS,
            EVENT_OBJECT_FOCUS,
            h_instance,
            Some(get_focused_window_details),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        );

        debug!("Hooked");

        let _event_hook_guard = EventHook(event_hook);

        let mut msg = MSG::default();

        loop {
            // Will wait here for a message, so no sleep is needed in the loop
            while GetMessageW(&mut msg, HWND(0), 0, 0).as_bool() {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            if is_terminating.load(Ordering::Relaxed) {
                PostQuitMessage(0);
            }

            if msg.message == windows::Win32::UI::WindowsAndMessaging::WM_QUIT {
                break;
            }
        }
    }

    Ok(())
}
