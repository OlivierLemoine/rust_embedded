pub struct Flags{
    is_finished: bool,
}

impl Flags{
    pub fn new() -> Flags{
        Flags{
            is_finished: false,
        }
    }

    pub fn has_finished(&self) -> bool{
        self.is_finished
    }

    pub fn finished(&mut self){
        self.is_finished = true;
    }
}