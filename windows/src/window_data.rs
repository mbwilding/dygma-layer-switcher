use std::path::Path;
use tracing::{debug, error};
use windows::Win32::System::Threading::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowTextW, GetWindowThreadProcessId};
use windows::Win32::{
    Foundation::{HWND, MAX_PATH},
    System::{ProcessStatus::K32GetModuleFileNameExW, Threading::OpenProcess},
    UI::WindowsAndMessaging::GetWindowTextLengthW,
};

pub unsafe fn get_exe_name(window_handle: HWND) -> Option<String> {
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

    debug!("Process handle: {:?}", process_handle.0);

    let mut exe_path_bytes: Vec<u16> = vec![0; MAX_PATH as usize];
    let exe_path_length = K32GetModuleFileNameExW(process_handle, None, &mut exe_path_bytes);
    let exe_path = String::from_utf16_lossy(&exe_path_bytes[..exe_path_length as usize]);

    debug!("Executable path: {:?}", exe_path);

    let exe_name = if let Some(file_name) = Path::new(&exe_path).file_name() {
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
    debug!("Executable name: {:?}", exe_name);

    if exe_name.is_empty() {
        return None;
    }

    Some(exe_name)
}

pub unsafe fn get_window_title(h_wnd: HWND) -> Option<String> {
    let title_length = GetWindowTextLengthW(h_wnd) + 1;
    let mut window_title: Vec<u16> = vec![0; title_length as usize];
    let _ = GetWindowTextW(h_wnd, window_title.as_mut_slice());
    let window_title = String::from_utf16_lossy(&window_title[..title_length as usize - 1]);
    debug!("Window title: {:?}", window_title);

    if window_title.is_empty() {
        return None;
    }

    Some(window_title)
}