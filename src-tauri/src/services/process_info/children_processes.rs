use sysinfo::{Pid, System};

#[derive(Debug)]
pub struct ChildProcess {
    pub pid: u32,
    pub name: String,
}

pub fn get_children_processes(pid: Pid) -> Result<Vec<ChildProcess>, String> {
    let mut system = System::new_all();
    system.refresh_all();

    let mut children: Vec<ChildProcess> = Vec::new();
    for (child_pid, process) in system.processes() {
        if process.parent() == Some(pid) {
            children.push(ChildProcess {
                pid: child_pid.as_u32(),
                name: process.name().to_string_lossy().to_string(),
            });
        }
    }

    Ok(children)
}
