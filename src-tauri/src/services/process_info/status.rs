use sysinfo::{Pid, System};

pub fn get_status(pid: Pid) -> Result<String, String> {
    let mut system = System::new_all();
    system.refresh_all();

    if let Some(process) = system.process(pid) {
        let status = process.status();
        Ok(status.to_string())
    } else {
        Err("Processo non trovato".to_string())
    }
}
