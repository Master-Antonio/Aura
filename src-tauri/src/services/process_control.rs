use crate::shared::system::get_system;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use sysinfo::Pid;
use thiserror::Error;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
};
use windows::Win32::System::SystemInformation::{GetSystemInfo, SYSTEM_INFO};
use windows::Win32::System::Threading::{
    OpenProcess, SetPriorityClass, SetProcessAffinityMask, HIGH_PRIORITY_CLASS,
    PROCESS_QUERY_INFORMATION, PROCESS_SET_INFORMATION, PROCESS_SUSPEND_RESUME,
};
use windows::Win32::System::Threading::{
    OpenThread, ResumeThread, SuspendThread, THREAD_SUSPEND_RESUME,
};

// Static cache for CPU usage calculation
#[cfg(target_os = "windows")]
static CPU_USAGE_CACHE: once_cell::sync::Lazy<Arc<Mutex<HashMap<u32, (u64, u64, SystemTime)>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

// External Windows API declarations
extern "C" {
    fn NtQuerySystemInformation(
        system_information_class: u32,
        system_information: *mut std::ffi::c_void,
        system_information_length: u32,
        return_length: *mut u32,
    ) -> i32;
}

// Constants for NtQuerySystemInformation
const SYSTEM_PROCESSES_AND_THREADS_INFORMATION: u32 = 5;

// NT Status codes
const STATUS_SUCCESS: i32 = 0x00000000;
const STATUS_INFO_LENGTH_MISMATCH: i32 = 0xC0000004u32 as i32;

// Thread states from Windows NT
const THREAD_STATE_WAIT: u32 = 5;
const THREAD_WAIT_REASON_SUSPENDED: u32 = 5;

// Structures for NtQuerySystemInformation
#[repr(C)]
#[derive(Debug)]
struct SystemProcessInformation {
    next_entry_offset: u32,
    number_of_threads: u32,
    reserved1: [u64; 3],
    create_time: i64,
    user_time: i64,
    kernel_time: i64,
    image_name: UnicodeString,
    base_priority: i32,
    unique_process_id: usize,
    inherited_from_unique_process_id: usize,
    handle_count: u32,
    session_id: u32,
    unique_process_key: usize,
    peak_virtual_size: usize,
    virtual_size: usize,
    page_fault_count: u32,
    peak_working_set_size: usize,
    working_set_size: usize,
    quota_peak_paged_pool_usage: usize,
    quota_paged_pool_usage: usize,
    quota_peak_non_paged_pool_usage: usize,
    quota_non_paged_pool_usage: usize,
    pagefile_usage: usize,
    peak_pagefile_usage: usize,
    private_page_count: usize,
    read_operation_count: i64,
    write_operation_count: i64,
    other_operation_count: i64,
    read_transfer_count: i64,
    write_transfer_count: i64,
    other_transfer_count: i64,
}

#[repr(C)]
#[derive(Debug)]
struct SystemThreadInformation {
    kernel_time: i64,
    user_time: i64,
    create_time: i64,
    wait_time: u32,
    start_address: *mut std::ffi::c_void,
    client_id: ClientId,
    priority: i32,
    base_priority: i32,
    context_switches: u32,
    thread_state: u32,
    wait_reason: u32,
}

#[repr(C)]
#[derive(Debug)]
struct ClientId {
    unique_process: *mut std::ffi::c_void,
    unique_thread: *mut std::ffi::c_void,
}

#[repr(C)]
#[derive(Debug)]
struct UnicodeString {
    length: u16,
    maximum_length: u16,
    buffer: *mut u16,
}

#[derive(Error, Debug)]
pub enum ProcessControlError {
    #[error("Failed to open process: {0}")]
    OpenError(String),

    #[error("Failed to set process affinity: {0}")]
    AffinityError(String),

    #[error("Operation not supported on this platform")]
    UnsupportedPlatform,

    #[error("Process not found: {0}")]
    NotFound(u32),
}

type Result<T> = std::result::Result<T, ProcessControlError>;

pub fn set_process_affinity(pid: u32) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            let process_handle = OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_SET_INFORMATION,
                false,
                pid,
            )
                .map_err(|e| ProcessControlError::OpenError(e.to_string()))?;

            let mut system_info = SYSTEM_INFO::default();
            GetSystemInfo(&mut system_info);

            // IMPROVED: Gaming boost should prioritize P-cores (first cores) for maximum performance
            // For modern CPUs, first cores are typically the most performant P-cores
            let core_count = system_info.dwNumberOfProcessors;

            // Use first 75% of cores for gaming performance (prioritizes P-cores)
            let gaming_cores = std::cmp::max(2, (core_count as f32 * 0.75) as u32);
            let mut affinity_mask = 0usize;

            // Set affinity to first N cores (most performant ones)
            for i in 0..gaming_cores {
                affinity_mask |= 1 << i;
            }

            SetProcessAffinityMask(process_handle, affinity_mask)
                .map_err(|e| ProcessControlError::AffinityError(e.to_string()))?;

            SetPriorityClass(process_handle, HIGH_PRIORITY_CLASS)
                .map_err(|e| ProcessControlError::AffinityError(e.to_string()))?;

            Ok(())
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err(ProcessControlError::UnsupportedPlatform)
    }
}

/// Enhanced gaming boost that optimizes for maximum gaming performance
/// - Prioritizes P-cores (first cores) which are most performant on modern CPUs
/// - Sets high priority for better CPU scheduling
/// - Uses optimal core allocation for gaming workloads
pub fn boost_process_for_gaming(pid: u32) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            let process_handle = OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_SET_INFORMATION,
                false,
                pid,
            )
                .map_err(|e| ProcessControlError::OpenError(e.to_string()))?;

            let mut system_info = SYSTEM_INFO::default();
            GetSystemInfo(&mut system_info);

            let core_count = system_info.dwNumberOfProcessors;

            // For gaming optimization:
            // - On hybrid CPUs (P+E cores): Use first 4-6 P-cores for best performance
            // - On traditional CPUs: Use first 50-75% of cores
            let gaming_cores = if core_count >= 8 {
                // Likely hybrid CPU, use first 4-6 cores (P-cores)
                std::cmp::min(6, core_count / 2)
            } else {
                // Traditional CPU, use first 75% of cores
                std::cmp::max(2, (core_count as f32 * 0.75) as u32)
            };

            let mut affinity_mask = 0usize;

            // Set affinity to first N cores (P-cores on modern systems)
            for i in 0..gaming_cores {
                affinity_mask |= 1 << i;
            }

            SetProcessAffinityMask(process_handle, affinity_mask)
                .map_err(|e| ProcessControlError::AffinityError(e.to_string()))?;

            // Set high priority for gaming performance
            SetPriorityClass(process_handle, HIGH_PRIORITY_CLASS)
                .map_err(|e| ProcessControlError::AffinityError(e.to_string()))?;

            Ok(())
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err(ProcessControlError::UnsupportedPlatform)
    }
}

pub fn set_process_affinity_cores(pid: u32, cores: Vec<u32>) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        if cores.is_empty() {
            return Err(ProcessControlError::AffinityError(
                "At least one core must be specified".to_string(),
            ));
        }

        unsafe {
            let process_handle = OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_SET_INFORMATION,
                false,
                pid,
            )
                .map_err(|e| ProcessControlError::OpenError(e.to_string()))?;

            let mut system_info = SYSTEM_INFO::default();
            GetSystemInfo(&mut system_info);

            // Create affinity mask from selected cores
            let mut affinity_mask: usize = 0;
            for core in cores {
                if core < system_info.dwNumberOfProcessors {
                    affinity_mask |= 1 << core;
                }
            }

            if affinity_mask == 0 {
                return Err(ProcessControlError::AffinityError(
                    "No valid cores specified".to_string(),
                ));
            }

            SetProcessAffinityMask(process_handle, affinity_mask)
                .map_err(|e| ProcessControlError::AffinityError(e.to_string()))?;

            SetPriorityClass(process_handle, HIGH_PRIORITY_CLASS)
                .map_err(|e| ProcessControlError::AffinityError(e.to_string()))?;

            CloseHandle(process_handle);

            Ok(())
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err(ProcessControlError::UnsupportedPlatform)
    }
}

pub fn get_process_affinity(pid: u32) -> Result<Vec<u32>> {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::System::Threading::GetProcessAffinityMask;

        unsafe {
            let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION, false, pid)
                .map_err(|e| ProcessControlError::OpenError(e.to_string()))?;

            let mut process_affinity_mask: usize = 0;
            let mut system_affinity_mask: usize = 0;

            GetProcessAffinityMask(
                process_handle,
                &mut process_affinity_mask,
                &mut system_affinity_mask,
            )
                .map_err(|e| ProcessControlError::AffinityError(e.to_string()))?;

            CloseHandle(process_handle);

            // Convert mask to core list
            let mut cores = Vec::new();
            for i in 0..64 {
                // Check up to 64 cores
                if process_affinity_mask & (1 << i) != 0 {
                    cores.push(i);
                }
            }

            Ok(cores)
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err(ProcessControlError::UnsupportedPlatform)
    }
}

pub fn kill_process(pid: u32) -> Result<()> {
    let mut system = get_system()
        .lock()
        .map_err(|e| ProcessControlError::OpenError(e.to_string()))?;
    system.refresh_all();

    // Find all child processes first
    let mut processes_to_kill = Vec::new();
    processes_to_kill.push(pid);

    // Recursively find all child processes
    fn find_children(system: &sysinfo::System, parent_pid: u32, processes_to_kill: &mut Vec<u32>) {
        for (child_pid, process) in system.processes() {
            if let Some(process_parent_pid) = process.parent() {
                if process_parent_pid.as_u32() == parent_pid {
                    let child_pid_u32 = child_pid.as_u32();
                    if !processes_to_kill.contains(&child_pid_u32) {
                        processes_to_kill.push(child_pid_u32);
                        // Recursively find grandchildren
                        find_children(system, child_pid_u32, processes_to_kill);
                    }
                }
            }
        }
    }

    find_children(&*system, pid, &mut processes_to_kill);

    // Kill all processes (children first, then parent)
    processes_to_kill.reverse(); // Kill children before parent

    let mut success_count = 0;
    let mut errors = Vec::new();

    for &process_pid in &processes_to_kill {
        if let Some(process) = system.process(Pid::from(process_pid as usize)) {
            if process.kill() {
                success_count += 1;
            } else {
                errors.push(format!("Failed to kill process {}", process_pid));
            }
        } else {
            errors.push(format!("Process {} not found", process_pid));
        }
    }

    if success_count > 0 {
        Ok(()) // At least some processes were killed successfully
    } else if !errors.is_empty() {
        Err(ProcessControlError::OpenError(errors.join("; ")))
    } else {
        Err(ProcessControlError::NotFound(pid))
    }
}

pub fn suspend_process(pid: u32) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        suspend_process_threads(pid)
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let output = Command::new("kill")
            .args(["-STOP", &pid.to_string()])
            .output()
            .map_err(|e| {
                ProcessControlError::OpenError(format!(
                    "Failed to send SIGSTOP to process {}: {}",
                    pid, e
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ProcessControlError::OpenError(format!(
                "Failed to suspend process {}: {}",
                pid, stderr
            )));
        }
        Ok(())
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        Err(ProcessControlError::UnsupportedPlatform)
    }
}

pub fn resume_process(pid: u32) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        resume_process_threads(pid)
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let output = Command::new("kill")
            .args(["-CONT", &pid.to_string()])
            .output()
            .map_err(|e| {
                ProcessControlError::OpenError(format!(
                    "Failed to send SIGCONT to process {}: {}",
                    pid, e
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ProcessControlError::OpenError(format!(
                "Failed to resume process {}: {}",
                pid, stderr
            )));
        }
        Ok(())
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        Err(ProcessControlError::UnsupportedPlatform)
    }
}

#[cfg(target_os = "windows")]
fn suspend_process_threads(pid: u32) -> Result<()> {
    unsafe {
        // First check if the process exists and is accessible
        let process_handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_SUSPEND_RESUME,
            false,
            pid,
        )
            .map_err(|e| {
                ProcessControlError::OpenError(format!("Failed to open process {}: {}", pid, e))
            })?;

        let _ = CloseHandle(process_handle); // Close immediately, we just needed to verify access

        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0).map_err(|e| {
            ProcessControlError::OpenError(format!("Failed to create thread snapshot: {}", e))
        })?;

        if snapshot == INVALID_HANDLE_VALUE {
            return Err(ProcessControlError::OpenError(
                "Invalid thread snapshot handle".to_string(),
            ));
        }

        let mut thread_entry = THREADENTRY32 {
            dwSize: std::mem::size_of::<THREADENTRY32>() as u32,
            cntUsage: 0,
            th32ThreadID: 0,
            th32OwnerProcessID: 0,
            tpBasePri: 0,
            tpDeltaPri: 0,
            dwFlags: 0,
        };

        let mut success_count = 0;
        let mut total_threads = 0;
        let mut errors = Vec::new();

        if Thread32First(snapshot, &mut thread_entry).is_ok() {
            loop {
                if thread_entry.th32OwnerProcessID == pid {
                    total_threads += 1;

                    match OpenThread(THREAD_SUSPEND_RESUME, false, thread_entry.th32ThreadID) {
                        Ok(handle) => {
                            let suspend_result = SuspendThread(handle);
                            if suspend_result != u32::MAX {
                                success_count += 1;
                            } else {
                                errors.push(format!(
                                    "Failed to suspend thread {}",
                                    thread_entry.th32ThreadID
                                ));
                            }
                            let _ = CloseHandle(handle);
                        }
                        Err(e) => {
                            errors.push(format!(
                                "Failed to open thread {}: {}",
                                thread_entry.th32ThreadID, e
                            ));
                        }
                    }
                }

                if Thread32Next(snapshot, &mut thread_entry).is_err() {
                    break;
                }
            }
        }
        let _ = CloseHandle(snapshot);

        if total_threads == 0 {
            return Err(ProcessControlError::NotFound(pid));
        }

        if success_count > 0 {
            Ok(())
        } else {
            Err(ProcessControlError::OpenError(format!(
                "Failed to suspend any of {} threads for process {} - Errors: {}",
                total_threads,
                pid,
                errors.join("; ")
            )))
        }
    }
}

#[cfg(target_os = "windows")]
fn resume_process_threads(pid: u32) -> Result<()> {
    unsafe {
        // First check if the process exists and is accessible
        let process_handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_SUSPEND_RESUME,
            false,
            pid,
        )
            .map_err(|e| {
                ProcessControlError::OpenError(format!("Failed to open process {}: {}", pid, e))
            })?;

        let _ = CloseHandle(process_handle); // Close immediately, we just needed to verify access

        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0).map_err(|e| {
            ProcessControlError::OpenError(format!("Failed to create thread snapshot: {}", e))
        })?;

        if snapshot == INVALID_HANDLE_VALUE {
            return Err(ProcessControlError::OpenError(
                "Invalid thread snapshot handle".to_string(),
            ));
        }

        let mut thread_entry = THREADENTRY32 {
            dwSize: std::mem::size_of::<THREADENTRY32>() as u32,
            cntUsage: 0,
            th32ThreadID: 0,
            th32OwnerProcessID: 0,
            tpBasePri: 0,
            tpDeltaPri: 0,
            dwFlags: 0,
        };

        let mut success_count = 0;
        let mut total_threads = 0;
        let mut errors = Vec::new();

        if Thread32First(snapshot, &mut thread_entry).is_ok() {
            loop {
                if thread_entry.th32OwnerProcessID == pid {
                    total_threads += 1;

                    match OpenThread(THREAD_SUSPEND_RESUME, false, thread_entry.th32ThreadID) {
                        Ok(handle) => {
                            let resume_result = ResumeThread(handle);
                            if resume_result != u32::MAX {
                                success_count += 1;
                            } else {
                                errors.push(format!(
                                    "Failed to resume thread {}",
                                    thread_entry.th32ThreadID
                                ));
                            }
                            let _ = CloseHandle(handle);
                        }
                        Err(e) => {
                            errors.push(format!(
                                "Failed to open thread {}: {}",
                                thread_entry.th32ThreadID, e
                            ));
                        }
                    }
                }

                if Thread32Next(snapshot, &mut thread_entry).is_err() {
                    break;
                }
            }
        }
        let _ = CloseHandle(snapshot);

        if total_threads == 0 {
            return Err(ProcessControlError::NotFound(pid));
        }

        if success_count > 0 {
            Ok(())
        } else {
            Err(ProcessControlError::OpenError(format!(
                "Failed to resume any of {} threads for process {} - Errors: {}",
                total_threads,
                pid,
                errors.join("; ")
            )))
        }
    }
}

fn execute_process_command(pid: u32, command: &str) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("powershell")
            .args(["-Command", &format!("{}-Process -Id {}", command, pid)])
            .output()
            .map_err(|e| ProcessControlError::OpenError(e.to_string()))?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let signal = if command == "suspend" { "STOP" } else { "CONT" };
        Command::new("kill")
            .args([&format!("-{}", signal), &pid.to_string()])
            .output()
            .map_err(|e| ProcessControlError::OpenError(e.to_string()))?;
        Ok(())
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        Err(ProcessControlError::UnsupportedPlatform)
    }
}

#[cfg(target_os = "windows")]
pub fn is_process_suspended(pid: u32) -> Result<bool> {
    unsafe {
        // First try to get the required buffer size
        let mut buffer_size: u32 = 0;
        let mut status = NtQuerySystemInformation(
            SYSTEM_PROCESSES_AND_THREADS_INFORMATION,
            std::ptr::null_mut(),
            0,
            &mut buffer_size,
        );

        if status != STATUS_INFO_LENGTH_MISMATCH {
            return Err(ProcessControlError::OpenError(
                "Failed to get buffer size for system information".to_string(),
            ));
        }

        // Allocate buffer with some extra space for potential growth
        buffer_size += 32768; // Add 32KB extra buffer
        let mut buffer = vec![0u8; buffer_size as usize];

        // Get the actual system information
        status = NtQuerySystemInformation(
            SYSTEM_PROCESSES_AND_THREADS_INFORMATION,
            buffer.as_mut_ptr() as *mut std::ffi::c_void,
            buffer_size,
            &mut buffer_size,
        );
        if status != STATUS_SUCCESS {
            return Err(ProcessControlError::OpenError(format!(
                "NtQuerySystemInformation failed with status: {:x}",
                status
            )));
        }

        // Parse the buffer to find our process
        let mut offset = 0usize;
        loop {
            if offset >= buffer.len() {
                break;
            }

            let process_info = &*(buffer.as_ptr().add(offset) as *const SystemProcessInformation);

            // Check if this is our target process
            if process_info.unique_process_id == pid as usize {
                // Found our process, now check its threads
                let threads_start = offset + std::mem::size_of::<SystemProcessInformation>();
                let mut suspended_threads = 0;
                let total_threads = process_info.number_of_threads;

                for i in 0..total_threads {
                    let thread_offset = threads_start
                        + (i as usize * std::mem::size_of::<SystemThreadInformation>());
                    if thread_offset + std::mem::size_of::<SystemThreadInformation>()
                        <= buffer.len()
                    {
                        let thread_info = &*(buffer.as_ptr().add(thread_offset)
                            as *const SystemThreadInformation);

                        // Check if thread is in wait state and wait reason is suspended
                        if thread_info.thread_state == THREAD_STATE_WAIT
                            && thread_info.wait_reason == THREAD_WAIT_REASON_SUSPENDED
                        {
                            suspended_threads += 1;
                        }
                    }
                }

                // Process is considered suspended if any of its threads are suspended
                return Ok(suspended_threads > 0);
            }

            // Move to next process entry
            if process_info.next_entry_offset == 0 {
                break;
            }
            offset += process_info.next_entry_offset as usize;
        }

        // Process not found
        Err(ProcessControlError::NotFound(pid))
    }
}

#[cfg(target_os = "linux")]
pub fn is_process_suspended(pid: u32) -> Result<bool> {
    use std::fs;

    // Read process state from /proc/[pid]/stat
    let stat_path = format!("/proc/{}/stat", pid);
    let stat_content =
        fs::read_to_string(&stat_path).map_err(|e| ProcessControlError::NotFound(pid))?;

    // The process state is the 3rd field in /proc/[pid]/stat
    // State can be: R (running), S (sleeping), D (disk sleep), T (stopped), etc.
    let fields: Vec<&str> = stat_content.split_whitespace().collect();
    if fields.len() < 3 {
        return Err(ProcessControlError::OpenError(
            "Invalid /proc/[pid]/stat format".to_string(),
        ));
    }

    let state = fields[2];
    // 'T' indicates stopped/suspended process
    Ok(state == "T")
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn is_process_suspended(_pid: u32) -> Result<bool> {
    Ok(false) // Always return false for unsupported platforms
}

// Complete process information structure
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub parent_pid: u32,
    pub name: String,
    pub exe_path: String,
    pub cpu_time_user: u64,
    pub cpu_time_kernel: u64,
    pub cpu_usage_percent: f64,
    pub memory_working_set: u64,
    pub memory_private: u64,
    pub memory_virtual: u64,
    pub memory_pagefile: u64,
    pub handle_count: u32,
    pub thread_count: u32,
    pub is_suspended: bool,
    pub create_time: i64,
    pub session_id: u32,
    pub io_read_bytes: u64,
    pub io_write_bytes: u64,
    pub io_read_operations: u64,
    pub io_write_operations: u64,
}

#[cfg(target_os = "windows")]
pub fn get_all_processes_info() -> Result<Vec<ProcessInfo>> {
    unsafe {
        // First try to get the required buffer size
        let mut buffer_size: u32 = 0;
        let mut status = NtQuerySystemInformation(
            SYSTEM_PROCESSES_AND_THREADS_INFORMATION,
            std::ptr::null_mut(),
            0,
            &mut buffer_size,
        );

        if status != STATUS_INFO_LENGTH_MISMATCH {
            return Err(ProcessControlError::OpenError(
                "Failed to get buffer size for system information".to_string(),
            ));
        }

        // Allocate buffer with extra space for potential growth
        buffer_size += 65536; // Add 64KB extra buffer
        let mut buffer = vec![0u8; buffer_size as usize];

        // Get the actual system information
        status = NtQuerySystemInformation(
            SYSTEM_PROCESSES_AND_THREADS_INFORMATION,
            buffer.as_mut_ptr() as *mut std::ffi::c_void,
            buffer_size,
            &mut buffer_size,
        );

        if status != STATUS_SUCCESS {
            return Err(ProcessControlError::OpenError(format!(
                "NtQuerySystemInformation failed with status: {:x}",
                status
            )));
        }

        let mut processes = Vec::new();
        let mut offset = 0usize;

        loop {
            if offset >= buffer.len() {
                break;
            }

            let process_info = &*(buffer.as_ptr().add(offset) as *const SystemProcessInformation);

            // Skip system idle process (PID 0)
            if process_info.unique_process_id != 0 {
                let pid = process_info.unique_process_id as u32;

                // Extract process name from Unicode string
                let process_name = if process_info.image_name.buffer.is_null()
                    || process_info.image_name.length == 0
                {
                    "System".to_string()
                } else {
                    let name_slice = std::slice::from_raw_parts(
                        process_info.image_name.buffer,
                        (process_info.image_name.length as usize) / 2,
                    );
                    String::from_utf16_lossy(name_slice)
                };

                // Get executable path
                let exe_path =
                    get_process_executable_path(pid).unwrap_or_else(|| "N/A".to_string());

                // Calculate CPU usage
                let cpu_usage = calculate_cpu_usage(
                    pid,
                    process_info.user_time as u64,
                    process_info.kernel_time as u64,
                );

                // Check if any thread is suspended
                let threads_start = offset + std::mem::size_of::<SystemProcessInformation>();
                let mut suspended_threads = 0;
                let total_threads = process_info.number_of_threads;

                for i in 0..total_threads {
                    let thread_offset = threads_start
                        + (i as usize * std::mem::size_of::<SystemThreadInformation>());
                    if thread_offset + std::mem::size_of::<SystemThreadInformation>()
                        <= buffer.len()
                    {
                        let thread_info = &*(buffer.as_ptr().add(thread_offset)
                            as *const SystemThreadInformation);

                        // Check if thread is in wait state and wait reason is suspended
                        if thread_info.thread_state == THREAD_STATE_WAIT
                            && thread_info.wait_reason == THREAD_WAIT_REASON_SUSPENDED
                        {
                            suspended_threads += 1;
                        }
                    }
                }

                let proc_info = ProcessInfo {
                    pid,
                    parent_pid: process_info.inherited_from_unique_process_id as u32,
                    name: process_name,
                    exe_path,
                    cpu_time_user: process_info.user_time as u64,
                    cpu_time_kernel: process_info.kernel_time as u64,
                    cpu_usage_percent: cpu_usage,
                    memory_working_set: process_info.working_set_size as u64,
                    memory_private: process_info.private_page_count as u64,
                    memory_virtual: process_info.virtual_size as u64,
                    memory_pagefile: process_info.pagefile_usage as u64,
                    handle_count: process_info.handle_count,
                    thread_count: process_info.number_of_threads,
                    is_suspended: suspended_threads > 0,
                    create_time: process_info.create_time,
                    session_id: process_info.session_id,
                    io_read_bytes: process_info.read_transfer_count as u64,
                    io_write_bytes: process_info.write_transfer_count as u64,
                    io_read_operations: process_info.read_operation_count as u64,
                    io_write_operations: process_info.write_operation_count as u64,
                };

                processes.push(proc_info);
            }

            // Move to next process entry
            if process_info.next_entry_offset == 0 {
                break;
            }
            offset += process_info.next_entry_offset as usize;
        }

        Ok(processes)
    }
}

#[cfg(target_os = "windows")]
fn get_process_executable_path(pid: u32) -> Option<String> {
    use windows::Win32::System::ProcessStatus::{GetModuleFileNameExW, GetProcessImageFileNameW};
    use windows::Win32::System::Threading::PROCESS_QUERY_LIMITED_INFORMATION;

    unsafe {
        match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) => {
                // First try GetModuleFileNameExW for the main module (should give full path)
                let mut path_buffer = [0u16; 1024];
                let result = GetModuleFileNameExW(Some(handle), None, &mut path_buffer);

                if result > 0 {
                    let path = String::from_utf16_lossy(&path_buffer[..result as usize]);
                    let _ = CloseHandle(handle);
                    return Some(path);
                }

                // Fallback to GetProcessImageFileNameW if the above fails
                let result = GetProcessImageFileNameW(handle, &mut path_buffer);
                let _ = CloseHandle(handle);

                if result > 0 {
                    let path = String::from_utf16_lossy(&path_buffer[..result as usize]);
                    // Convert device path to drive letter path
                    if let Some(converted_path) = convert_device_path_to_drive_path(&path) {
                        Some(converted_path)
                    } else {
                        Some(path)
                    }
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}

#[cfg(target_os = "windows")]
fn convert_device_path_to_drive_path(device_path: &str) -> Option<String> {
    // Convert paths like \Device\HarddiskVolume1\Windows\System32\notepad.exe
    // to C:\Windows\System32\notepad.exe
    use windows::Win32::Storage::FileSystem::{GetLogicalDriveStringsW, QueryDosDeviceW};

    unsafe {
        let mut drives_buffer = [0u16; 256];
        let drives_len = GetLogicalDriveStringsW(Some(&mut drives_buffer));

        if drives_len == 0 {
            return None;
        }

        let drives_str = String::from_utf16_lossy(&drives_buffer[..drives_len as usize]);
        let drives: Vec<&str> = drives_str.split('\0').filter(|s| !s.is_empty()).collect();

        for drive in drives {
            if drive.len() >= 2 {
                let drive_letter = &drive[..2]; // e.g., "C:"
                let mut device_buffer = [0u16; 256];
                let drive_letter_wide: Vec<u16> = drive_letter
                    .encode_utf16()
                    .chain(std::iter::once(0))
                    .collect();
                let device_len = QueryDosDeviceW(
                    PCWSTR::from_raw(drive_letter_wide.as_ptr()),
                    Some(&mut device_buffer),
                );

                if device_len > 0 {
                    let device_name =
                        String::from_utf16_lossy(&device_buffer[..device_len as usize]);
                    let device_name = device_name.trim_end_matches('\0');

                    if device_path.starts_with(device_name) {
                        let relative_path = &device_path[device_name.len()..];
                        return Some(format!("{}{}", drive_letter, relative_path));
                    }
                }
            }
        }
    }

    None
}

#[cfg(target_os = "windows")]
fn calculate_cpu_usage(pid: u32, user_time: u64, kernel_time: u64) -> f64 {
    let current_time = SystemTime::now();
    let current_total_time = user_time + kernel_time;

    let mut cache = CPU_USAGE_CACHE.lock().unwrap();

    if let Some((last_total_time, _last_user_time, last_timestamp)) = cache.get(&pid) {
        let time_delta = current_time
            .duration_since(*last_timestamp)
            .unwrap_or_default()
            .as_secs_f64();
        let cpu_time_delta = current_total_time.saturating_sub(*last_total_time);

        // Convert FILETIME (100ns units) to seconds and calculate percentage
        let cpu_seconds = (cpu_time_delta as f64) / 10_000_000.0;
        let cpu_percentage = if time_delta > 0.0 {
            (cpu_seconds / time_delta) * 100.0
        } else {
            0.0
        };

        // Update cache
        cache.insert(pid, (current_total_time, user_time, current_time));

        cpu_percentage.min(100.0) // Cap at 100%
    } else {
        // First time seeing this process, store data and return 0
        cache.insert(pid, (current_total_time, user_time, current_time));
        0.0
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_all_processes_info() -> Result<Vec<ProcessInfo>> {
    Err(ProcessControlError::UnsupportedPlatform)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_process_control() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let pid = std::process::id();

            // Test suspend/resume
            let suspend_result = suspend_process(pid);
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            assert!(suspend_result.is_ok());

            let resume_result = resume_process(pid);
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            assert!(resume_result.is_ok());

            // Test affinity
            let affinity_result = set_process_affinity(pid);
            #[cfg(target_os = "windows")]
            assert!(affinity_result.is_ok());
            #[cfg(not(target_os = "windows"))]
            assert!(matches!(
                affinity_result,
                Err(ProcessControlError::UnsupportedPlatform)
            ));
        });
    }

    #[test]
    fn test_invalid_process() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let result = kill_process(0);
            assert!(matches!(result, Err(ProcessControlError::NotFound(0))));
        });
    }
}

// Function to get detailed information for a single process
#[cfg(target_os = "windows")]
pub fn get_process_detailed_info(pid: u32) -> Result<ProcessInfo> {
    let all_processes = get_all_processes_info()?;

    all_processes
        .into_iter()
        .find(|p| p.pid == pid)
        .ok_or(ProcessControlError::NotFound(pid))
}

#[cfg(not(target_os = "windows"))]
pub fn get_process_detailed_info(pid: u32) -> Result<ProcessInfo> {
    // Fallback implementation using sysinfo
    use crate::shared::system::get_system;

    let mut system = get_system()
        .lock()
        .map_err(|e| ProcessControlError::OpenError(e.to_string()))?;
    system.refresh_all();

    let process = system
        .process(Pid::from(pid as usize))
        .ok_or(ProcessControlError::NotFound(pid))?;

    let is_suspended = is_process_suspended(pid).unwrap_or(false);

    Ok(ProcessInfo {
        pid,
        parent_pid: process.parent().map(|p| p.as_u32()).unwrap_or(0),
        name: process.name().to_string_lossy().into_owned(),
        exe_path: process
            .exe()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| "N/A".to_string()),
        cpu_time_user: 0,   // Not available through sysinfo
        cpu_time_kernel: 0, // Not available through sysinfo
        cpu_usage_percent: process.cpu_usage() as f64,
        memory_working_set: process.memory(),
        memory_private: 0, // Not available through sysinfo
        memory_virtual: process.virtual_memory(),
        memory_pagefile: 0, // Not available through sysinfo
        handle_count: 0,    // Not available through sysinfo
        thread_count: 0,    // Not available through sysinfo
        is_suspended,
        create_time: 0,         // Not easily available through sysinfo
        session_id: 0,          // Not available through sysinfo
        io_read_bytes: 0,       // Not available through sysinfo
        io_write_bytes: 0,      // Not available through sysinfo
        io_read_operations: 0,  // Not available through sysinfo
        io_write_operations: 0, // Not available through sysinfo
    })
}

// Function to get child processes of a given PID
pub fn get_child_processes(parent_pid: u32) -> Result<Vec<ProcessInfo>> {
    #[cfg(target_os = "windows")]
    {
        let all_processes = get_all_processes_info()?;
        let children = all_processes
            .into_iter()
            .filter(|p| p.parent_pid == parent_pid)
            .collect();
        Ok(children)
    }

    #[cfg(not(target_os = "windows"))]
    {
        use crate::shared::system::get_system;

        let mut system = get_system()
            .lock()
            .map_err(|e| ProcessControlError::OpenError(e.to_string()))?;
        system.refresh_all();

        let mut children = Vec::new();

        for (pid, process) in system.processes() {
            if let Some(process_parent_pid) = process.parent() {
                if process_parent_pid.as_u32() == parent_pid {
                    let child_pid = pid.as_u32();
                    let is_suspended = is_process_suspended(child_pid).unwrap_or(false);

                    let child_info = ProcessInfo {
                        pid: child_pid,
                        parent_pid,
                        name: process.name().to_string_lossy().into_owned(),
                        exe_path: process
                            .exe()
                            .map(|p| p.to_string_lossy().into_owned())
                            .unwrap_or_else(|| "N/A".to_string()),
                        cpu_time_user: 0,
                        cpu_time_kernel: 0,
                        cpu_usage_percent: process.cpu_usage() as f64,
                        memory_working_set: process.memory(),
                        memory_private: 0,
                        memory_virtual: process.virtual_memory(),
                        memory_pagefile: 0,
                        handle_count: 0,
                        thread_count: 0,
                        is_suspended,
                        create_time: 0,
                        session_id: 0,
                        io_read_bytes: 0,
                        io_write_bytes: 0,
                        io_read_operations: 0,
                        io_write_operations: 0,
                    };

                    children.push(child_info);
                }
            }
        }

        Ok(children)
    }
}
