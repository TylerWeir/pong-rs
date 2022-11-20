// This is a command pattern type used to capture movement commands
// which may be applied to players in the game. Actors capable of receiving 
// MoveCommands have the trait Moveable.
pub trait Moveable {
    fn add_x(&mut self, value: i16);
    fn add_y(&mut self, value: i16);
}

pub trait MoveCommand {
    fn execute(&self, target: impl Moveable) -> &str;
}

pub struct MoveUp;
impl MoveCommand for MoveUp {
    fn execute(&self, mut target: impl Moveable) -> &str {
        target.add_y(1);
        "moving up" 
    }
}

pub struct MoveDown;
impl MoveCommand for MoveDown {
    fn execute(&self, mut target: impl Moveable) -> &str {
        target.add_y(-1);
        "moving down"
    }
}

pub struct MoveLeft;
impl MoveCommand for MoveLeft {
    fn execute(&self, mut target: impl Moveable) -> &str {
        target.add_x(-1);
        "moving left"
    }
}

pub struct MoveRight;
impl MoveCommand for MoveRight {
    fn execute(&self, mut target: impl Moveable) -> &str {
        target.add_x(1);
        "moving right"
    }
}
