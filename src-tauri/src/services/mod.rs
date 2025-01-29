pub mod process_info;
pub mod process_control;
pub mod process_service;
pub mod gpu_service;
pub mod optimization_service;

// Re-export delle funzioni più utilizzate
pub use process_control::{
    kill_process,
    resume_process,
    set_process_affinity,
    suspend_process,
};

pub use process_info::{
    children_processes,
    cpu,
    disk_io,
    env_vars,
    memory,
    name,
    parent_pid,
    session_id,
    status,
    user,
};
