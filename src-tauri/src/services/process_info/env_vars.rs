use ntapi::ntpsapi::{
    NtQueryInformationProcess, ProcessBasicInformation, PROCESS_BASIC_INFORMATION,
};
use ntapi::winapi::ctypes::c_void;
use std::collections::HashMap;
use std::io::{self, Error, ErrorKind};
use sysinfo::Pid;
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

#[cfg(target_os = "windows")]
pub fn get_env_vars(pid: Pid) -> io::Result<HashMap<String, String>> {
    unsafe {
        let handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            pid.as_u32(),
        )?;

        let handle_ptr = handle.0 as *mut c_void;

        let mut pbi: PROCESS_BASIC_INFORMATION = std::mem::zeroed();
        let status = NtQueryInformationProcess(
            handle_ptr,
            ProcessBasicInformation,
            &mut pbi as *mut _ as *mut _,
            std::mem::size_of::<PROCESS_BASIC_INFORMATION>() as u32,
            std::ptr::null_mut(),
        );

        if status != 0 {
            return Err(Error::new(
                ErrorKind::Other,
                "Impossibile leggere le informazioni del processo",
            ));
        }

        let mut env_vars = HashMap::new();
        let mut buffer = vec![0u16; 4096];
        let mut bytes_read: usize = 0;

        let result = ReadProcessMemory(
            handle,
            pbi.PebBaseAddress as *const _,
            buffer.as_mut_ptr() as *mut _,
            buffer.len(),
            Some(&mut bytes_read as *mut usize),
        );

        if result.is_ok() {
            // Parsing delle variabili d'ambiente dalla memoria
            let env_block = String::from_utf16_lossy(&buffer[..bytes_read / 2]);
            for line in env_block.split('\0') {
                if let Some((key, value)) = line.split_once('=') {
                    env_vars.insert(key.to_string(), value.to_string());
                }
            }
            Ok(env_vars)
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "Impossibile leggere la memoria del processo",
            ))
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_env_vars(pid: Pid) -> io::Result<HashMap<String, String>> {
    let output = if cfg!(target_os = "linux") {
        Command::new("cat")
            .arg(format!("/proc/{}/environ", pid.as_u32()))
            .output()?
    } else {
        Command::new("ps")
            .arg("eww")
            .arg(pid.as_u32().to_string())
            .output()?
    };

    if !output.status.success() {
        return Err(Error::new(
            ErrorKind::Other,
            "Impossibile ottenere le variabili di ambiente",
        ));
    }

    let contents = String::from_utf8_lossy(&output.stdout);
    let env_vars = contents
        .split('\0')
        .filter(|s| !s.is_empty())
        .filter_map(|entry| {
            let parts: Vec<&str> = entry.splitn(2, '=').collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].to_string()))
            } else {
                None
            }
        })
        .collect();

    Ok(env_vars)
}
