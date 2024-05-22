use libmem::*;

#[cfg(windows)]
pub fn inject(proc: lm_process_t, lib_path: String) {
    unimplemented!()
}

#[cfg(target_os = "linux")]
pub fn inject(proc: lm_process_t, lib_path: String) {
    let size = lib_path.as_bytes().len();
    let alloc = LM_AllocMemoryEx(&proc, size, LM_PROT_XRW);
    alloc.expect("Failed to allocate memory");
    let _write = LM_WriteMemoryEx(&proc, alloc.unwrap(), &lib_path);
}
