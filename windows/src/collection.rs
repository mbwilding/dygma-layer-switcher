use common::config::AppDetails;
use std::path::Path;
use tracing::{error, trace};
use windows::Win32::{
    Foundation::{HWND, MAX_PATH},
    System::{
        ProcessStatus::K32GetModuleFileNameExW,
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
    UI::WindowsAndMessaging::{GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId},
};

/// # Safety
///
/// WinAPI.
pub unsafe fn hydrate(window_handle: HWND) -> AppDetails {
    AppDetails {
        window: get_window(window_handle),
        process: get_process(window_handle),
    }
}

/// # Safety
///
/// WinAPI.
unsafe fn get_process(window_handle: HWND) -> Option<String> {
    let mut process_id: u32 = 0;

    let thread_id = GetWindowThreadProcessId(window_handle, Some(&mut process_id as *mut u32));

    if thread_id == 0 {
        error!("Failed to retrieve process ID");
        return None;
    };

    let process_handle = match OpenProcess(
        PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
        false,
        process_id,
    ) {
        Ok(handle) => handle,
        Err(e) => {
            error!("Failed to open process: {:?}", e);
            return None;
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
            return None;
        }
    } else {
        error!("Failed to extract the executable name from the path");
        return None;
    };
    trace!("Executable name: {:?}", process);

    if process.is_empty() {
        return None;
    }

    Some(process)
}

/// # Safety
///
/// WinAPI.
unsafe fn get_window(h_wnd: HWND) -> Option<String> {
    let title_length = GetWindowTextLengthW(h_wnd) + 1;
    let mut window: Vec<u16> = vec![0; title_length as usize];
    let _ = GetWindowTextW(h_wnd, window.as_mut_slice());
    let window = String::from_utf16_lossy(&window[..title_length as usize - 1]);
    trace!("Window: {:?}", window);

    if window.is_empty() {
        return None;
    }

    Some(window)
}
