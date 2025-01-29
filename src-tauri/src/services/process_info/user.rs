use sysinfo::{Pid, System};

pub fn get_user(pid: Pid) -> Result<String, String> {
    let mut system = System::new_all();
    system.refresh_all();

    if let Some(process) = system.process(pid) {
        Ok(process
            .user_id()
            .map_or("Unknown".to_string(), |uid| uid.to_string()))
    } else {
        Err("Processo non trovato".to_string())
    }
}
