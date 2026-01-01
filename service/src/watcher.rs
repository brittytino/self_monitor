use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowTextW,
    GetWindowThreadProcessId,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;
use windows::Win32::Foundation::{CloseHandle, MAX_PATH};
use std::mem::size_of;

pub struct SystemWatcher;

impl SystemWatcher {
    /// Returns (App Name, Window Title)
    pub fn get_active_window_info() -> (String, String) {
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.0 == std::ptr::null_mut() {
                return ("Unknown".to_string(), "No Active Window".to_string());
            }

            // 1. Get Window Title
            let mut title_buf = [0u16; 512];
            let len = GetWindowTextW(hwnd, &mut title_buf);
            let title = if len > 0 {
                String::from_utf16_lossy(&title_buf[..len as usize])
            } else {
                "Unknown".to_string()
            };

            // 2. Get Process Name
            let mut process_id = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut process_id));
            
            let process_handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id);
            let app_name = if let Ok(handle) = process_handle {
                let mut name_buf = [0u16; MAX_PATH as usize];
                let name_len = GetModuleFileNameExW(Some(handle), None, &mut name_buf);
                let _ = CloseHandle(handle); // Ensure we close the handle

                if name_len > 0 {
                    let full_path = String::from_utf16_lossy(&name_buf[..name_len as usize]);
                    // Extract just the executable name (e.g., "code.exe")
                    std::path::Path::new(&full_path)
                        .file_name()
                        .map(|f| f.to_string_lossy().into_owned())
                        .unwrap_or(full_path)
                } else {
                    "Unknown".to_string()
                }
            } else {
                "Unknown".to_string()
            };

            (app_name, title)
        }
    }

    pub fn get_idle_seconds() -> u64 {
        unsafe {
            let mut plii = LASTINPUTINFO {
                cbSize: size_of::<LASTINPUTINFO>() as u32,
                dwTime: 0,
            };
            
            if GetLastInputInfo(&mut plii).as_bool() {
                let tick_count = windows::Win32::System::SystemInformation::GetTickCount();
                // tick_count wraps every 49.7 days, strictly speaking handling wrap is good but for simple difference it works if consistent types
                // dwTime is also DWORD (u32), GetTickCount returns u32.
                // Idle = Now - LastInput
                if tick_count >= plii.dwTime {
                    (tick_count - plii.dwTime) as u64 / 1000
                } else {
                    0 // Overflow handling or system restart oddity
                }
            } else {
                0
            }
        }
    }
}
