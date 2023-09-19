use std::fs::File;
use std::{fs,path};
use std::io::Write;
pub struct Cache{
    current_caption:i32,
    caption_to_analize:i32,
    caption: String
}

impl Cache {

    pub fn new() -> Cache {
        Cache{current_caption:1, caption: "".to_string(), caption_to_analize:1}
    }

    pub fn set_caption(&mut self,caption:&str) {
        self.caption = caption.to_string();
    }

    pub fn write_to_cache(&mut self,line: &str) {
        let curent_caption = self.current_caption;
        let mut file = File::create(format!("caption{}.txt",curent_caption)).unwrap();
        file.write_all(line.as_bytes()).unwrap();
        if curent_caption == 3 {
            self.current_caption =1;
            self.caption_to_analize = 3;
        }else{
            self.current_caption +=1;
            self.caption_to_analize = self.current_caption-1;
        }
    }

    pub fn check_if_caption_exists(&self) -> bool{
        let curent_caption = self.caption_to_analize;
        if !path::Path::new(&format!("caption{}.txt",curent_caption)).exists() {return  false;};

        let text = fs::read_to_string(format!("caption{}.txt",curent_caption)).unwrap();

        let result = if &text[..] == self.caption {
            true
        } else{
            false
        };
        result
    }
}