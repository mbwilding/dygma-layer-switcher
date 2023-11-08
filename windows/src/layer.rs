use crate::window_data::{get_exe_name, get_window_title};
use common::app::App;
use windows::Win32::Foundation::HWND;

pub unsafe fn process(window_handle: HWND) {
    let app = App {
        window_title: get_window_title(window_handle),
        exe_name: get_exe_name(window_handle),
    };

    common::serial::process(&app);
}
