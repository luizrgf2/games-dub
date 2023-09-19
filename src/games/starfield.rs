use crate::read_camption;
use winapi::ctypes::c_void;
use crate::process;

use super::game_interface::{Game, GameInstance};


pub struct StarField{
    pub process_handle: *mut c_void,
    pub base_addr_module: String
}

impl Game for StarField {

    fn start(&mut self) {
        self.open_process();
        self.get_base_addr();
    }

    fn read_caption_of_conversation(&self) -> String {

        let read_caption = read_camption::ReactCaption::new(self.process_handle);
        let addr1 = read_caption.read_offset(&self.base_addr_module,"059DCC38");
    
        let read_caption = read_camption::ReactCaption::new(self.process_handle);
        let addr2 = read_caption.read_offset(&addr1,"8");
    
        let read_caption = read_camption::ReactCaption::new(self.process_handle);
        let caption = read_caption.read_text_from_addr(&addr2, "18");
        
        caption
    }

    fn open_process(&mut self){
        let process_handle = process::Process::open_process_by_name("Starfield").unwrap();
        self.process_handle = process_handle;
    }

    fn get_base_addr(&mut self){
        let base_addr_module = process::Process::find_base_addr_from_module_name("Starfield.exe", self.process_handle).unwrap();
        self.base_addr_module = base_addr_module;
    }


}

impl GameInstance<StarField> for StarField{
    fn new() -> StarField {
        StarField {base_addr_module: String::new(), process_handle:0x0 as *mut c_void}
    }
}