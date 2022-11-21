use crate::command::MoveCommand;
use crate::command::Moveable;

extern crate ncurses;
use ncurses::*;

// Represents a paddle in the game pong. Note that a paddle 
// can be either a human player or a computer player. Paddles
// accept input the form of commands.
pub struct Paddle {
    x: i16,
    y: i16,
    height: u16,
    width: u16,
}

impl Moveable for &mut Paddle {
    fn add_x(&mut self, value: i16) {
        self.x = self.x + value;
    }

    fn add_y(&mut self, value: i16) {
        self.y = self.y + value;
    }
}

impl Paddle {
    // Creates a new paddle with zero valued fields
    pub fn new() -> Paddle {
        Paddle {
            x: 0,
            y: 0,
            height: 0,
            width: 0,
        }
    }

    // Returns the x position of the paddle
    pub fn get_x(&self) -> i16 {
        self.x
    } 

    // Returns the y position of the paddle
    pub fn get_y(&self) -> i16 {
        self.y
    }

    // Returns the width of the paddle
    pub fn get_width(&self) -> u16 {
        self.width
    }

    // Returns the height of the paddle
    pub fn get_height(&self) -> u16 {
        self.height
    }

    // The paddle receives commands and executes them here.
    // pretty slick stuff if you ask me
    pub fn do_cmd(&mut self, cmd: impl MoveCommand) -> &str {
        cmd.execute(self);
        "hello"
    }

    pub fn draw(&self, y: i32, x: i32) {
        mvaddstr(y, x, "Paddle:");
        mvaddstr(y+1, x, format!("X: {}", self.x).as_ref());
        mvaddstr(y+2, x, format!("Y: {}", self.y).as_ref());
        mvaddstr(y+3, x, format!("Width: {}", self.width).as_ref());
        mvaddstr(y+4, x, format!("Height: {}", self.height).as_ref());
    }
}

#[cfg(test)]
mod tests {
    use crate::command;
    use crate::paddle::Paddle;

    #[test]
    fn test_new() {
        let my_paddle = Paddle::new();
        assert!(my_paddle.get_x() == 0);
        assert!(my_paddle.get_y() == 0);
        assert!(my_paddle.get_width() == 0);
        assert!(my_paddle.get_height() == 0);
    }

    #[test]
    fn test_cmds() {
        let mut my_paddle = Paddle::new();
        my_paddle.do_cmd(command::MoveUp{});
        my_paddle.do_cmd(command::MoveUp{});
        my_paddle.do_cmd(command::MoveRight{});

        assert!(my_paddle.get_x() == 1);
        assert!(my_paddle.get_y() == 2);
        assert!(my_paddle.get_width() == 0);
        assert!(my_paddle.get_height() == 0);
    }
}
