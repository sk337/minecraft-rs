use std::env;

#[cfg(windows)]
pub fn get_process_title(pid: u32) -> Option<String> {
    // windows only Imports
    extern crate winapi;

    use std::ffi::CStr;
    use std::mem::MaybeUninit;
    use std::ptr::null_mut;
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::processthreadsapi::{GetProcessImageFileNameA, OpenProcess};
    use winapi::um::psapi::PROCESS_VM_READ;
    use winapi::um::winnt::PROCESS_QUERY_INFORMATION;

    unsafe {
        // Open the process with query information and VM read access
        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
        if process_handle.is_null() {
            return None;
        }

        // Create a buffer to hold the process name
        let mut buffer: [i8; 1024] = [0; 1024];
        let len =
            GetProcessImageFileNameA(process_handle, buffer.as_mut_ptr(), buffer.len() as u32);

        // Close the process handle
        CloseHandle(process_handle);

        if len == 0 {
            return None;
        }

        // Convert the C string to a Rust String
        let c_str = CStr::from_ptr(buffer.as_ptr());
        match c_str.to_str() {
            Ok(str) => Some(str.to_string()),
            Err(_) => None,
        }
    }
}

#[cfg(target_os = "macos")]
pub fn get_process_title(pid: u32) -> Option<String> {
    use procinfo::pid;

    match pid::name(pid) {
        Ok(name) => Some(name),
        Err(_) => None,
    }
}

fn detect_display_server() -> Option<String> {
    if let Ok(_) = env::var("WAYLAND_DISPLAY") {
        return Some("Wayland".to_string());
    } else {
        return Some("X11".to_string());
    }
}

#[cfg(target_os = "linux")]
pub fn get_process_title(pid: u32) -> Option<String> {
    // Linux implementation here
    let environ = detect_display_server().unwrap();
    if environ == "X11" {
        // X11 implementation here
        unimplemented!()
    } else if environ == "Wayland" {
        // Wayland implementation here
        unimplemented!()
    } else {
        panic!("Unknown display server")
    }
}
