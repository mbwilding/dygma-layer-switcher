#![windows_subsystem = "windows"]
mod window_data;
mod message_loop;
mod window;

use serde_yaml;
use anyhow::Result;
use std::sync::Arc;
use signal_hook::consts::TERM_SIGNALS;
use std::sync::atomic::{AtomicBool, Ordering};
use signal_hook::flag;
use tracing::{debug};
use windows::Win32::UI::WindowsAndMessaging::{MESSAGE_RESOURCE_ENTRY, PM_REMOVE};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::HWND,
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            Accessibility::SetWinEventHook,
            WindowsAndMessaging::{
                DispatchMessageW, PeekMessageW, PostQuitMessage, TranslateMessage,
                EVENT_OBJECT_FOCUS, MSG, WINEVENT_OUTOFCONTEXT,
            },
        },
    },
};

use crate::message_loop::{EventHook, get_focused_window_details};

fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(if cfg!(debug_assertions) {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    })
    .with_ansi(true)
    .init();
    // atomic bool can be safely shared between threads, same memory representation as bool
    let is_terminating = Arc::new(AtomicBool::new(false));

    // & represents a reference without taking ownership of it, * would be the dereference operator
    //
    for &signal in TERM_SIGNALS {
        flag::register_conditional_shutdown(signal, 0, Arc::clone(&is_terminating))?;
        flag::register(signal, Arc::clone(&is_terminating))?;
    }
    // the question mark at the end of the line is error handling
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

        loop{
            while PeekMessageW(&mut msg, HWND(0), 0, 0, PM_REMOVE).into() {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            if is_terminating.load(Ordering::Relaxed) {
                PostQuitMessage(0);
            }

            if msg.message == windows::Win32::UI::WindowsAndMessaging::WM_QUIT {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    Ok(())
}
