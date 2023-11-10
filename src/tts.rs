use tts_rust::{ tts::GTTSClient, languages::Languages };


pub struct TTs {    
    queue_of_speaks: Vec<String>,
    current_caption: String
}

impl TTs {
    pub fn new()-> TTs {
        TTs { queue_of_speaks: Vec::new(),current_caption: String::new() }
    }

    pub fn set_caption(&mut self, caption:&str) {
        self.current_caption = caption.to_owned();
    }

    pub fn check_if_caption_alreary_exists(&self, caption:&str) -> bool {
        self.queue_of_speaks.contains(&caption.to_string())
    }

    pub fn send_caption_to_queue(&mut self, caption:&str) {
        self.queue_of_speaks.push(caption.to_owned());
    }

    pub fn speak_captions(&mut self,) {
            let current_caption = self.queue_of_speaks.pop();
    
            if let Some(value) = current_caption {
                self.speak(&value);
            }
    }
    fn speak(&self,text:&str) {
        let  narrator: GTTSClient = GTTSClient {
            tld: "com",
            volume: 1.0, 
            language: Languages::Portuguese, // use the Languages enum
        };
        narrator.speak(text).unwrap();
    }
}
