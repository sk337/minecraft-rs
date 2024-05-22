use libmem::*;
use std::ffi::CString;

#[cfg(windows)]
pub fn inject(proc: lm_process_t, lib_path: String) {
    unimplemented!()
}

#[cfg(target_os = "linux")]
pub fn inject(proc: lm_process_t, lib_path: String) {
    let lib_path = CString::new(lib_path).expect("CString::new failed");
    let size = lib_path.as_bytes().len();
    let alloc = LM_AllocMemoryEx(&proc, size, LM_PROT_XRW);
    alloc.expect("Failed to allocate memory");
    let _write = LM_WriteMemoryEx(&proc, alloc.unwrap(), &lib_path);

    let dlopen_address = find_dlopen_address().expect("Failed to find dlopen address");
    for modu in LM_EnumModulesEx(&proc).unwrap() {
        if modu.get_name().contains("libc.so") {
            println!("Found libc.so: {}", modu.get_name());
            println!("Base: 0x{:X}", modu.get_base());
            println!("Size: {}", modu.get_size());
            println!("Path: {}", modu.get_path());
        }
    }
}

fn find_dlopen_address() -> Result<usize, String> {
    // Implementation to find dlopen address
    // You can use a library like goblin to parse ELF headers and find the address of dlopen
    // For simplicity, this example returns a hardcoded address
    Ok(0x12345678)
}
