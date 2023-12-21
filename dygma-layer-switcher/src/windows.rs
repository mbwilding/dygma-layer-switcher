use crate::layer;
use crate::structs::AppDetails;
use std::path::Path;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use tracing::{debug, error, trace};
use windows::core::PCWSTR;
use windows::Win32::{
    Foundation::{HWND, MAX_PATH},
    System::{
        LibraryLoader::GetModuleHandleW,
        ProcessStatus::K32GetModuleFileNameExW,
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
    UI::{
        Accessibility::{SetWinEventHook, HWINEVENTHOOK},
        WindowsAndMessaging::{
            GetMessageW, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId,
            EVENT_OBJECT_FOCUS, MSG, WINEVENT_OUTOFCONTEXT,
        },
    },
};

/// # Safety
///
/// WinAPI.
pub unsafe fn hydrate(window_handle: HWND) -> AppDetails {
    let app_details = AppDetails {
        window: get_window(window_handle),
        process: get_process(window_handle),
    };

    debug!("{:?}", app_details);

    app_details
}

/// # Safety
///
/// WinAPI.
unsafe fn get_process(window_handle: HWND) -> String {
    let mut process_id: u32 = 0;

    let thread_id = GetWindowThreadProcessId(window_handle, Some(&mut process_id as *mut u32));

    if thread_id == 0 {
        error!("Failed to retrieve process ID");
        return String::new();
    };

    let process_handle = match OpenProcess(
        PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
        false,
        process_id,
    ) {
        Ok(handle) => handle,
        Err(e) => {
            error!("Failed to open process: {:?}", e);
            return String::new();
        }
    };

    trace!("Process handle: {:?}", process_handle.0);

    let mut exe_path_bytes: Vec<u16> = vec![0; MAX_PATH as usize];
    let exe_path_length = K32GetModuleFileNameExW(process_handle, None, &mut exe_path_bytes);
    let exe_path = String::from_utf16_lossy(&exe_path_bytes[..exe_path_length as usize]);

    trace!("Process: {:?}", exe_path);

    let process = if let Some(file_name) = Path::new(&exe_path).file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            file_name_str.to_owned()
        } else {
            error!("Failed to convert the executable name to string");
            return String::new();
        }
    } else {
        error!("Failed to extract the executable name from the path");
        return String::new();
    };
    trace!("Executable name: {:?}", process);

    process
}

/// # Safety
///
/// WinAPI.
unsafe fn get_window(h_wnd: HWND) -> String {
    let title_length = GetWindowTextLengthW(h_wnd) + 1;
    let mut window: Vec<u16> = vec![0; title_length as usize];
    let _ = GetWindowTextW(h_wnd, window.as_mut_slice());
    let window = String::from_utf16_lossy(&window[..title_length as usize - 1]);
    trace!("Window: {:?}", window);

    window
}

static DEBOUNCER: AtomicU32 = AtomicU32::new(0);

/// # Safety
///
/// WinAPI.
pub unsafe extern "system" fn window_focused(
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

    trace!("window_handle: {:?}", window_handle);

    if dwms_event_time == DEBOUNCER.load(Ordering::SeqCst) {
        return;
    }

    DEBOUNCER.store(dwms_event_time, Ordering::SeqCst);

    let app_details = hydrate(window_handle);

    layer::process(&app_details);
}

pub fn start() {
    std::thread::spawn(|| unsafe {
        debug!("Hooking");

        let module_handle = GetModuleHandleW(PCWSTR::null()).unwrap_or_else(|e| {
            error!("Failed to get module handle: {:?}", e);
            std::process::exit(1);
        });

        let _event_hook = SetWinEventHook(
            EVENT_OBJECT_FOCUS,
            EVENT_OBJECT_FOCUS,
            module_handle,
            Some(window_focused),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        );

        debug!("Hooked");

        let mut msg = MSG::default();

        loop {
            // Will wait here for a message, so no sleep is needed in the loop
            // This is a blocking call on a thread, but is required for the hook to work
            GetMessageW(&mut msg, HWND(0), 0, 0);
        }
    });
}
