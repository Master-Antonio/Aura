use sysinfo::{Pid, System};

pub fn get_name(pid: Pid) -> Result<String, String> {
    let mut system = System::new_all();
    system.refresh_all();

    if let Some(process) = system.process(pid) {
        let name = process.name();
        Ok(name.to_string_lossy().to_string())
    } else {
        Err("Processo non trovato".to_string())
    }
}
