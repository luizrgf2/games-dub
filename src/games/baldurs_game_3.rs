use crate::read_camption;
use winapi::ctypes::c_void;
use crate::process;

use super::game_interface::{Game, GameInstance};


pub struct BaldursGame3{
    pub process_handle: *mut c_void,
    pub base_addr_module: String
}

impl Game for BaldursGame3 {

    fn start(&mut self) {
        self.open_process();
        self.get_base_addr();
    }

    fn read_caption_of_conversation(&self) -> String {

        let read_caption = read_camption::ReactCaption::new(self.process_handle);
        let addr1 = read_caption.read_offset(&self.base_addr_module,"057BDF48");
    
        let read_caption = read_camption::ReactCaption::new(self.process_handle);
        let addr2 = read_caption.read_offset(&addr1,"390");
        
        let read_caption = read_camption::ReactCaption::new(self.process_handle);
        let addr2 = read_caption.read_offset(&addr2,"188");

        let read_caption = read_camption::ReactCaption::new(self.process_handle);
        let addr2 = read_caption.read_offset(&addr2,"38");

        let read_caption = read_camption::ReactCaption::new(self.process_handle);
        let addr2 = read_caption.read_offset(&addr2,"158");

        let read_caption = read_camption::ReactCaption::new(self.process_handle);
        let addr2 = read_caption.read_offset(&addr2,"340");

        let read_caption = read_camption::ReactCaption::new(self.process_handle);
        let addr2 = read_caption.read_offset(&addr2,"90");
    
        let read_caption = read_camption::ReactCaption::new(self.process_handle);
        let caption = read_caption.read_text_from_addr(&addr2, "0");
        
        caption.replace("<i>", "").replace("</i>", "")
    }

    fn open_process(&mut self){
        let process_handle = process::Process::open_process_by_name("bg3").unwrap();
        self.process_handle = process_handle;
    }

    fn get_base_addr(&mut self){
        let base_addr_module = process::Process::find_base_addr_from_module_name("bg3.exe", self.process_handle).unwrap();
        self.base_addr_module = base_addr_module;
    }


}

impl GameInstance<BaldursGame3> for BaldursGame3{
    fn new() -> BaldursGame3 {
        BaldursGame3 {base_addr_module: String::new(), process_handle:0x0 as *mut c_void}
    }
}