use nvml_wrapper::enums::device::UsedGpuMemory;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::Nvml;
use std::error::Error;
use sysinfo::Pid;

pub fn get_gpu_usage() -> Result<(), Box<dyn Error>> {
    // Placeholder implementation - GPU monitoring will be implemented later
    //println!("GPU monitoring not yet implemented");
    Ok(())
}

pub fn get_gpu_usage_by_pid(pid: Pid) -> Result<Option<UsedGpuMemory>, Box<dyn Error>> {
    let nvml = Nvml::init()?;
    let device = nvml.device_by_index(0)?;

    // Recupera i processi in esecuzione sulla GPU
    match device.running_graphics_processes() {
        Ok(processes) => {
            for process in processes {
                if process.pid == pid.as_u32() {
                    return Ok(Some(process.used_gpu_memory));
                }
            }
            //println!("[INFO] Processo con PID {} non trovato sulla GPU.", pid);
            Ok(None)
        }
        Err(NvmlError::NotSupported) => {
            /*println!(
                "[WARN] La raccolta delle statistiche di utilizzo del processo non \
                      è supportata dal driver NVML corrente."
            );*/
            Err(Box::new(NvmlError::NotSupported))
        }
        Err(e) => {
            /*println!(
                "[ERROR] Errore durante la raccolta delle statistiche: {}",
                e
            );*/
            Err(Box::new(e))
        }
    }
}
