use std::path::Path;
use windows::Win32::{
    Foundation::{HWND, MAX_PATH},
    System::{ProcessStatus::K32GetModuleFileNameExW, Threading::OpenProcess},
    UI::WindowsAndMessaging::GetWindowTextLengthW
};
use anyhow::Result;
use windows::core::factory;
use windows::Win32::System::Threading::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowTextW, GetWindowThreadProcessId};
use tracing::{debug, error};

pub unsafe fn get_exe_name(window_handle: HWND) -> Result<String> {
    /// fetches the exe name of the application corresponding to "window_handle"
    /// using the windows::Win32 crate
    // create mutable variable process_id as a 32 bit unsigned integer
    let mut process_id: u32 = 0;
    // call GetWindowThreadProcessId, "Some" is used as a way to circumvent the unsafe use of a null pointer (?)
    let thread_id = GetWindowThreadProcessId(window_handle, Some(&mut process_id
     as *mut u32));
    // if the thread_id is 0 return an error
    if thread_id == 0 {
        error!("Failed to retrieve process ID");
        return Ok(String::new());
    };
    // returns the process handle, no idea what "PROCESS_QUERY_INFORMATION | PROCESS_VM_READ" means
    let process_handle = OpenProcess(
        PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
        false,
        process_id
    )?;
    // return specific log
    debug!("Process handle: {:?}", process_handle.0);
    // create a vector with u16 elements. vec! is a macro provided by std to create vectors
    // with the same syntax as arrays
    let mut exe_path_bytes: Vec<u16> = vec![0; MAX_PATH as usize];
    // get file path length
    let exe_path_length = K32GetModuleFileNameExW(process_handle, None, &mut exe_path_bytes);
    // get exe_path, decode from utf16
    let exe_path = String::from_utf16_lossy(&exe_path_bytes[..exe_path_length as usize]);

    debug!("Executable path: {:?}", exe_path);

    let exe_name = if let Some(file_name) = Path::new(&exe_path).file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            file_name_str.to_owned()
        } else {
            error!("Failed to convert the execuable name to string");
            String::new() // return empty string
        }

    } else {
        error!("Failed to extract the executable name from the path");
        String::new()
    };
    debug!("Executable name: {:?}", exe_name);

    Ok(exe_name)
}

pub unsafe fn get_window_title(h_wnd: HWND) -> String {
    /// get the window title from the window handle using the windows::Win32 crate
    let title_length = GetWindowTextLengthW(h_wnd) + 1;
    let mut window_title: Vec<u16> = vec![0; title_length as usize];
    let _ = GetWindowTextW(h_wnd, window_title.as_mut_slice());
    let window_title = String::from_utf16_lossy(&window_title[..title_length as usize - 1]);
    debug!("Window title: {:?}", window_title);
    window_title
}