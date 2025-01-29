use sysinfo::{Pid, System};

pub fn get_session_id(pid: u32) -> u32 {
    let mut system = System::new_all();
    system.refresh_all();

    match system.process(Pid::from(pid as usize)) {
        Some(process) => match process.session_id() {
            Some(session_id) => session_id.as_u32(),
            None => 0
        },
        None => 0
    }
}
