use crate::command::Moveable;
use crate::actor_utils::Messages;
use crate::actor_utils::Actor;

// Represents a paddle in the game pong. Note that a paddle 
// can be either a human player or a computer player. Paddles
// accept input the form of commands.
pub struct Paddle {
    x: i16,
    y: i16,
}

impl Moveable for &mut Paddle {
    fn add_x(&mut self, value: i16) {
        self.x = self.x + value;
    }

    fn add_y(&mut self, value: i16) {
        self.y = self.y + value;
    }
}

impl Actor for Paddle {
    fn poll (&self, r: crossbeam::channel::Receiver<Messages>) {
        println!("paddle is polling for messages...");

        loop {
            match r.recv() {
                Ok(_msg) => println!("paddle received a message!"),
                Err(_err) => println!("paddle experiencing errors"),
            }
        }
    }
}

impl Paddle {
    // Creates a new paddle with zero valued fields
    pub fn new(_s: crossbeam::channel::Sender<Messages>) -> Paddle {
        Paddle {
            x: 0,
            y: 0,
        }
    }
}

