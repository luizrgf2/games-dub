use process_list::for_each_process;
use std::os::windows::prelude::OsStringExt;
use std::path::Path;
use std::ptr;
use std::ffi::OsString;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{DWORD, HMODULE};
use winapi::um::psapi::{EnumProcessModules, GetModuleBaseNameW};

pub struct Process {

}

impl Process {
    fn get_module_base_address(name_module: &str, process_handle: *mut c_void) -> Option<usize> {
        unsafe {
         
    
            // Enumerar os módulos do processo
            let mut module_handles: [HMODULE; 1024] = [ptr::null_mut(); 1024];
            let mut bytes_needed: DWORD = 0;
    
            if EnumProcessModules(
                process_handle,
                module_handles.as_mut_ptr(),
                std::mem::size_of_val(&module_handles) as DWORD,
                &mut bytes_needed,
            ) == 0
            {
                return None;
            }
    
            let num_modules = (bytes_needed / std::mem::size_of::<HMODULE>() as DWORD) as usize;
    
            for i in 0..num_modules {
                let mut module_name_buf: Vec<u16> = vec![0; 512];
    
                if GetModuleBaseNameW(
                    process_handle,
                    module_handles[i],
                    module_name_buf.as_mut_ptr(),
                    module_name_buf.len() as DWORD,
                ) == 0
                {
                    continue;
                }
    
                let module_name_osstr = OsString::from_wide(&module_name_buf);
                let module_name_string = module_name_osstr.to_string_lossy();
    
                if module_name_string.contains(name_module) {
                    return Some(module_handles[i] as usize);
                }
            }
        }
    
        None
    }

    fn find_pid_by_name(name_process: &str) -> i32 {

        let mut id_proccess:i32 = 0;
        for_each_process(|id: u32, name: &Path| {
            let name_of_proccess = &name.display().to_string()[..];
            if name_of_proccess.contains(name_process) {
                id_proccess = id as i32;
            }
        }).unwrap();
        id_proccess
    }
    
    pub fn open_process_by_name(name_procces: &str) -> Result<*mut c_void,String>{
        let pid = Process::find_pid_by_name(name_procces);

        if pid == 0 {
           return Err("O pid do processo não existe!".to_string());
        }

        let process_handle = unsafe {
            OpenProcess(PROCESS_ALL_ACCESS,0,pid as u32)
        };

        if process_handle.is_null(){
            return Err(format!("Erro para abrir o processo com o código {}",unsafe { winapi::um::errhandlingapi::GetLastError() }));
        };

        Ok(process_handle)
    }

    pub fn find_base_addr_from_module_name(name_module: &str, process_handle: *mut c_void) -> Result<String,String> {
        let base_addr = Process::get_module_base_address(name_module, process_handle);
     
        if let Some(value) = base_addr {
            Ok(format!("{:X}", value))
        }else{
            Err("Erro para abrir o modulo base da aplicação!".to_string())
        }
    }

}