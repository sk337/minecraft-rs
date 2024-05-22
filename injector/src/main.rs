mod inject;
use inject::inject;

use libmem::*;

fn find_procs() -> Vec<lm_process_t> {
    let procs = LM_EnumProcesses().unwrap();
    let mut java_procs: Vec<lm_process_t> = Vec::new();
    for proc in procs {
        if proc.get_name() == "java" || proc.get_name() == "java.exe" {
            // let title = get_process_title(proc.get_pid()).unwrap_or("N/A".to_string());
            // println!("Found java process: {}", title);
            java_procs.push(proc);
        }
    }
    java_procs
}

fn main() {
    let procs = find_procs();
    if procs.is_empty() {
        println!("No java processes found");
        return;
    }

    let mut idx: u32 = 0;
    for proc in procs.clone() {
        println!("{}.) {}: {}", idx + 1, proc.get_name(), proc.get_pid());
        idx += 1;
    }
    let mut selection = String::new();

    println!("Select a process to inject into: ");
    selection.clear();
    std::io::stdin().read_line(&mut selection).unwrap();
    let selection = selection.trim().parse::<u32>().unwrap();
    if selection > procs.len() as u32 {
        println!("Invalid selection");
        return;
    }
    let inject_process = procs.get(selection as usize - 1).unwrap();
    println!("Injecting into: {}", inject_process.get_name());
    let mut ext = String::new();
    if cfg!(target_os = "windows") {
        ext = "dll".to_string();
    } else if cfg!(target_os = "linux") {
        ext = "so".to_string();
    } else if cfg!(target_os = "macos") {
        ext = "dylib".to_string();
    }
    let dll_path = std::env::current_dir()
        .unwrap()
        .join("..")
        .join("loader")
        .join("target")
        .join("release")
        .join(format!("libinjector.{}", ext));
    println!("Injecting: {:?}", dll_path);
    inject(
        inject_process.clone(),
        dll_path.to_str().unwrap().to_string(),
    )
}
