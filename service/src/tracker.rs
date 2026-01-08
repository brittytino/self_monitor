use windows::Win32::Foundation::{HWND, FALSE};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowThreadProcessId, GetWindowTextW,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetLastInputInfo, LASTINPUTINFO,
};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::System::ProcessStatus::GetModuleBaseNameW;
use windows::Win32::System::SystemInformation::GetTickCount;

pub fn is_user_idle() -> bool {
    unsafe {
        let mut lii = LASTINPUTINFO::default();
        lii.cbSize = std::mem::size_of::<LASTINPUTINFO>() as u32;
        // 0.52: GetLastInputInfo returns BOOL
        if GetLastInputInfo(&mut lii).as_bool() {
            let tick_count = GetTickCount();
            // Simple subtraction, ignoring wrap-around edge case for KISS
            let elapsed = tick_count.wrapping_sub(lii.dwTime);
            return elapsed > 30_000;
        }
    }
    false
}

pub fn get_active_window_info() -> (String, String) {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0 == 0 {
            return ("Unknown".to_string(), "Unknown".to_string());
        }
        
        // Get Title
        let mut title_buf = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut title_buf);
        let title = if len > 0 {
             String::from_utf16_lossy(&title_buf[..len as usize])
        } else {
             "Unknown".to_string()
        };
        
        // Get Process Name
        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        
        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid);
        let app_name = if let Ok(handle) = process_handle {
             let mut name_buf = [0u16; 512];
             // HMODULE(0) instead of None
             let len = GetModuleBaseNameW(handle, windows::Win32::Foundation::HMODULE(0), &mut name_buf);
             if len > 0 {
                 String::from_utf16_lossy(&name_buf[..len as usize])
             } else {
                 "Unknown".to_string()
             }
        } else {
            "Unknown".to_string()
        };
        
        (title, app_name)
    }
}
