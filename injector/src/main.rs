mod get_proc_title;

use libmem::*;
use get_proc_title::get_process_title;


fn find_procs() -> Vec<lm_process_t> {
    let procs = LM_EnumProcesses().unwrap();
    let mut java_procs: Vec<lm_process_t> = Vec::new();
    for proc in procs {
        if proc.get_name() == "java" || proc.get_name() == "java.exe" {
            let title = get_process_title(proc.get_pid()).unwrap_or("N/A".to_string());
            println!("Found java process: {}", title);
            java_procs.push(proc);
        }
    }
    java_procs
}

fn main() {
    find_procs();
}
