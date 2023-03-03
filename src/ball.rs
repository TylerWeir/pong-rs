extern crate ncurses;

use crate::actor_utils::Messages;
use crate::actor_utils::Actor;
use crate::actor_utils::Point;
use crate::utils::sprite::Sprite;

pub struct Ball {
    // Ball
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,

    // TODO figure out how to hold charactertistics of other actors... 
    max_x: i32,
    max_y: i32,

    broker: crossbeam::channel::Sender<Messages>,
    sprite: Sprite
}

impl Actor for Ball {
    fn poll(&mut self, r: crossbeam::channel::Receiver<Messages>) {
        loop {
            match r.recv() {
                Ok(msg) => self.handle_message(msg),
                Err(_err) => panic!("Ball failed to receive a message"),
            }
        }               
    }
}

impl Ball {
    pub fn new(s: crossbeam::channel::Sender<Messages>) -> Ball {
        Ball {
         x: 0,
         y: 0,
         vx: 1,
         vy: 1,
         max_x: 0,
         max_y: 0,
         broker: s,
         sprite: Sprite::new(1, 1, "@"),
        }
    }

    fn handle_message(&mut self, msg:Messages) {
        match msg {
            Messages::Tick => self.tick(),
            Messages::ScreenSize(x, y) => self.update_bounds(x, y),
            _ => (), 
        }
    }

    // TODO this should live in a state machine... 
    // tick will vary depending on the state of the ball and game
    fn tick(&mut self) {
        self.update_ball_heading();
        self.tick_ball_pos();

        // TODO wrap these in a function in a state machine.
        // The messages sent at the end of a tick will vary between states
        match self.broker.try_send(Messages::BallPos(self.x, self.y)) {
            Ok(_) => (), 
            Err(_err) => panic!("Ball failed to send position message"),
        }

        match self.broker.try_send(Messages::Draw(Point::new(self.x, self.y), self.sprite)) {
            Ok(_) => (),
            Err(_err) => println!("Ball failed to send draw message"),
        }
    }

    fn update_bounds(&mut self, x:i32, y:i32) {
        self.max_x = x;
        self.max_y = y;
    }

    fn update_ball_heading(&mut self) {
        // update x heading 
        if self.x + self.sprite.get_width() > self.max_x {
            self.vx = -1;
        } else if self.x < 0 {
            self.vx = 1;
        }

        // update y heading
        if self.y + self.sprite.get_height() > self.max_y {
            self.vy = -1;
        } else if self.y < 0 {
            self.vy = 1;
        }
    }

    fn tick_ball_pos(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
}
