use crate::command::Cmd;

// Represents a paddle in the game pong. Note that a paddle 
// can be either a human player or a computer player. Paddles
// accept input the form of commands.
pub struct Paddle {
    x: i16,
    y: i16,
    height: u16,
    width: u16,
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
    pub fn do_cmd(&self, cmd: impl Cmd) -> &str {
        cmd.execute();
        "hello"
    }
}

