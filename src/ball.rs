use crate::command::Moveable;
use crate::physics::SolidBody;
use crate::ipc::Messages;

pub struct Ball {
    x: i16,
    y: i16,
    vx: i16,
    vy: i16,
}

impl SolidBody for Ball {
    fn update(&mut self) {
        self.x = self.x + self.vx;
        self.y = self.y + self.vy;
    }
}

impl Moveable for Ball {
    fn add_x(&mut self, value: i16) {
        self.x = self.x + value;
    }

    fn add_y(&mut self, value: i16) {
        self.y = self.y + value;
    }
}

impl Ball {

    pub fn new() -> Ball {
        Ball {
         x: 0,
         y: 0,
         vx: 1,
         vy: 1,
        }
    }

    fn handle_message(&mut self, msg: Messages) {
        match msg {
            Messages::TickMsg => self.update(),
        }
    }
    
    pub fn poll(&mut self, r: crossbeam::channel::Receiver<Messages>) {
        println!("ball is polling for messages...");

        loop {
            match r.recv() {
                Ok(msg) => self.handle_message(msg),
                Err(_err) => println!("experiencing errors!"),
            }
        }               
    }
}


#[cfg(test)]
mod tests {
    use crate::ball::Ball;
    use crate::ipc::Messages;

    #[test]
    fn test_new() {
        let ball = Ball::new();
        assert!(ball.x == 0);
        assert!(ball.y == 0);
        assert!(ball.vx == 1);
        assert!(ball.vx == 1);
    }

    #[test]
    fn test_cmds() {
        let mut ball = Ball::new();
        ball.handle_message(Messages::TickMsg);

        assert!(ball.x == 0 + ball.vx);
        assert!(ball.y == 0 + ball.vy);

    }
}


