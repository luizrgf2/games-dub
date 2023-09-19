

pub trait Game {
    fn start(&mut self);
    fn open_process(&mut self);
    fn get_base_addr(&mut self);
    fn read_caption_of_conversation(&self) -> String ;
}

pub trait GameInstance<T> {
    fn new() ->T;
}