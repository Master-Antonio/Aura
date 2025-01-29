use sysinfo::{Pid, System};

pub fn get_parent_pid(pid: Pid) -> Option<i32> {
    let mut system = System::new_all();
    system.refresh_all();

    system
        .process(pid)
        .and_then(|process| process.parent())
        .map(|parent_pid| parent_pid.as_u32() as i32)
}
