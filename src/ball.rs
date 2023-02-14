extern crate ncurses;

use crate::physics::SolidBody;
use crate::actor_utils::Messages;
use crate::actor_utils::Actor;
use crate::actor_utils::Point;
use crate::utils::sprite::Sprite;

pub struct Ball {
    x: i16,
    y: i16,
    vx: i16,
    vy: i16,
    broker: crossbeam::channel::Sender<Messages>,
    sprite: Sprite
}

impl SolidBody for Ball {
    fn update(&mut self) {
        self.x = self.x + self.vx;
        self.y = self.y + self.vy;
    }
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
         broker: s,
         sprite: Sprite::new("@@\n@@"),
        }
    }

    fn handle_message(&mut self, msg:Messages) {
        match msg {
            Messages::Tick => self.tick(),
            _ => (), 
        }
    }

    fn tick(&mut self) {
        self.x += 1;
        self.y += 1;

        match self.broker.try_send(Messages::BallPos(self.x, self.y)) {
            Ok(_) => (), 
            Err(_err) => panic!("Ball failed to send position message"),
        }

        match self.broker.try_send(Messages::Draw(Point::new(self.x, self.y), self.sprite)) {
            Ok(_) => (),
            Err(_err) => println!("Ball failed to send draw message"),
        }
    }
}
