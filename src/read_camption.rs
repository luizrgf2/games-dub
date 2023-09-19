use winapi;
use winapi::ctypes::c_void;
use winapi::um::memoryapi::ReadProcessMemory;
pub struct ReactCaption{
    pub proccess: *mut c_void
}

impl ReactCaption {
    pub fn new(proccess: *mut c_void) -> ReactCaption {
        ReactCaption { proccess }
    }

    fn reverse_addr(addr: &str) -> String {
        let mut hex_final = String::new();
        let mut aux = String::new();
        for c in addr.chars().rev(){

            aux.push(c);

            if aux.len() == 2 {
                for c_aux in aux.chars().rev() {
                    hex_final = format!("{}{}",&hex_final,c_aux);
                }
                aux = String::new();
            }
            
        }
        hex_final
    }

    pub fn read_offset(self, offset1: &str, offset2: &str) -> String {
    
        let num1 = u64::from_str_radix(offset1, 16).unwrap();
        let num2 = u64::from_str_radix(offset2, 16).unwrap();
    
        let addr1 = u64::from_str_radix(&format!("{:X}", (num1 + num2)),16).unwrap() as *mut c_void;
    
        let mut buffer_addr1:Vec<u8> = vec![0; 8];
    
        unsafe {
            ReadProcessMemory(
                self.proccess.clone(), 
                addr1 as *mut c_void,
                buffer_addr1.as_mut_ptr() as  *mut c_void, 
                8, 
                std::ptr::null_mut())
        };
    
        let hex_string: String = buffer_addr1.iter()
            .map(|byte| format!("{:02X}", byte))
            .collect::<Vec<String>>()
            .join("")
            ;

        let hex_reversed = ReactCaption::reverse_addr(&hex_string);

        hex_reversed

    }

    pub fn read_text_from_addr(self,base_addr: &str, offset: &str) -> String {
    
        let num1 = u64::from_str_radix(base_addr, 16).unwrap();
        let num2 = u64::from_str_radix(offset, 16).unwrap();
    
        let mut addr1 = u64::from_str_radix(&format!("{:X}", (num1 + num2)),16).unwrap();
        
        let mut buffer_addr1:Vec<u8> = Vec::new();
        
        loop {
            let mut temp_buffer: Vec<u8> = vec![0,1];

            let read_memory = unsafe {
                ReadProcessMemory(
                    self.proccess.clone(), 
                    addr1 as *mut c_void,
                    temp_buffer.as_mut_ptr() as  *mut c_void, 
                    1, 
                    std::ptr::null_mut())
            };

            if read_memory == 0 {
                break;
            }

            if temp_buffer[0] == 0 {
                break;
            }

            addr1+=1;

            buffer_addr1.push(temp_buffer[0])
        }

         let result_string = match std::str::from_utf8(&buffer_addr1) {
            Ok(s) => s,
            Err(err) => {
                println!("Erro ao converter para string UTF-8. {}", err);
                ""
            }
        };

        result_string.to_owned()
    }

}
