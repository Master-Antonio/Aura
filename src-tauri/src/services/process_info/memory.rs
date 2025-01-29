use sysinfo::{Pid, System};

pub fn get_memory_usage(pid: Pid) -> Result<u64, String> {
    let mut system = System::new_all();
    system.refresh_all();

    if let Some(process) = system.process(pid) {
        let memory_usage = process.memory();
        Ok(memory_usage / 1024 / 1024)
    } else {
        Err("Processo non trovato".to_string())
    }
}
