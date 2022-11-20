pub trait Cmd {
    fn execute(&self) -> &str {
        "default"
    }
}

pub struct MoveUp;
impl Cmd for MoveUp {
    fn execute(&self) -> &str {
        "moving up" 
    }
}


pub struct MoveDown;
impl Cmd for MoveDown {
    fn execute(&self) -> &str {
        "moving down"
    }
}
