use sysinfo::{Pid, System};

pub fn get_cpu_usage(pid: Pid) -> Result<f32, String> {
    let mut system = System::new_all();
    system.refresh_all();

    if let Some(process) = system.process(pid) {
        let cpu_usage = process.cpu_usage();
        Ok(cpu_usage)
    } else {
        Err("Processo non trovato".to_string())
    }
}
