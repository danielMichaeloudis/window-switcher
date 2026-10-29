use std::ffi::c_void;

use windows::{
    Win32::{
        Foundation::HWND,
        UI::{
            Input::KeyboardAndMouse::SetActiveWindow,
            WindowsAndMessaging::{
                GetForegroundWindow, IsIconic, SW_RESTORE, SetForegroundWindow, ShowWindow,
            },
        },
    },
    core::HRESULT,
};
use x_win::WindowInfo;

pub struct OpenWindow {
    id: u32,
    title: String,
    path: String,
    exec_name: String,
}

impl From<WindowInfo> for OpenWindow {
    fn from(value: WindowInfo) -> Self {
        let info = value.info;
        Self {
            id: value.id,
            title: value.title,
            path: info.path,
            exec_name: info.exec_name,
        }
    }
}

pub fn get_open_windows() -> x_win::Result<Vec<OpenWindow>> {
    x_win::get_open_windows().map(|wins| wins.into_iter().map(Into::into).collect())
}

pub fn set_active_window(id: u32) -> bool {
    let hwnd = HWND(id as *mut c_void);
    unsafe {
        if IsIconic(hwnd).as_bool() {
            println!("is iconic");
            ShowWindow(hwnd, SW_RESTORE);
        }

        if !SetForegroundWindow(hwnd).as_bool() {
            eprintln!("SetForegroundWindow was rejected");
            return false;
        }

        let foreground = GetForegroundWindow();

        if foreground != hwnd {
            eprintln!("Target did not become the foreground window");
            return false;
        }

        true
    }
}
