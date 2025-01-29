use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Diagnostics::ToolHelp::CreateToolhelp32Snapshot;
use windows::Win32::System::Diagnostics::ToolHelp::Module32First;
use windows::Win32::System::Diagnostics::ToolHelp::Module32Next;
use windows::Win32::System::Diagnostics::ToolHelp::MODULEENTRY32;
use windows::Win32::System::Diagnostics::ToolHelp::TH32CS_SNAPMODULE;

pub fn get_loaded_modules(pid: u32) -> Vec<String> {
    let mut modules = Vec::new();
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid);
        if let Ok(handle) = snapshot {
            let mut module_entry = MODULEENTRY32 {
                dwSize: std::mem::size_of::<MODULEENTRY32>() as u32,
                ..Default::default()
            };

            if Module32First(handle, &mut module_entry).is_ok() {
                loop {
                    let module_name =
                        String::from_utf8_lossy(&module_entry.szModule.map(|c| c as u8)[..])
                            .trim_end_matches('\0')
                            .to_lowercase();

                    modules.push(module_name);

                    if Module32Next(handle, &mut module_entry).is_err() {
                        break;
                    }
                }
            }
            let _ = CloseHandle(handle);
        }
    }
    modules
}
