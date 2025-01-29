use sysinfo::{Pid, System};

pub fn get_disk_io(pid: Pid) -> Result<String, String> {
    let mut system = System::new_all();
    system.refresh_all();
    if let Some(process) = system.process(pid) {
        let disk_io = process.disk_usage();
        Ok(format!(
            "{} KB / {} KB",
            disk_io.read_bytes / 1024,
            disk_io.written_bytes / 1024
        ))
    } else {
        Err("Processo non trovato".to_string())
    }
}
