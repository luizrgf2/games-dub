use crate::read_camption;
use winapi::ctypes::c_void;
use crate::process;

use super::game_interface::{Game, GameInstance};


pub struct RedDead2{
    pub process_handle: *mut c_void,
    pub base_addr_module: String
}

impl Game for RedDead2 {

    fn start(&mut self) {
        self.open_process();
        self.get_base_addr();
    }

    fn read_caption_of_conversation(&self) -> String {

        let read_caption = read_camption::ReactCaption::new(self.process_handle);
        let addr1 = read_caption.read_offset(&self.base_addr_module,"05C048E8");
    
        let read_caption = read_camption::ReactCaption::new(self.process_handle);
        let addr2 = read_caption.read_offset(&addr1,"0");
        
        let read_caption = read_camption::ReactCaption::new(self.process_handle);
        let addr2 = read_caption.read_offset(&addr2,"0");

        let read_caption = read_camption::ReactCaption::new(self.process_handle);
        let caption = read_caption.read_text_from_addr(&addr2, "0");
        
        caption.replace("<i>", "").replace("</i>", "")
    }

    fn open_process(&mut self){
        let process_handle = process::Process::open_process_by_name("RDR2").unwrap();
        self.process_handle = process_handle;
    }

    fn get_base_addr(&mut self){
        let base_addr_module = process::Process::find_base_addr_from_module_name("RDR2.exe", self.process_handle).unwrap();
        self.base_addr_module = base_addr_module;
    }


}

impl GameInstance<RedDead2> for RedDead2{
    fn new() -> RedDead2 {
        RedDead2 {base_addr_module: String::new(), process_handle:0x0 as *mut c_void}
    }
}