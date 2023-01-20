extern crate ncurses;

use crate::command::Moveable;
use crate::physics::SolidBody;
use crate::ipc::Messages;
use crate::ipc::Actor;

pub struct Ball {
    x: i16,
    y: i16,
    vx: i16,
    vy: i16,
    broker: crossbeam::channel::Sender<Messages>,
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

impl Actor for Ball {
    fn poll(&mut self, r: crossbeam::channel::Receiver<Messages>) {
        println!("ball is polling for messages...");

        loop {
            match r.recv() {
                Ok(msg) => self.handle_msg(msg.clone()),
                Err(_err) => println!("ball experiencing errors!"),
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
        }
    }

    pub fn draw(x:i16, y:i16) {
        ncurses::mvaddstr(y as i32, x as i32, "##");
        ncurses::mvaddstr(y as i32 + 1, x as i32, "##");
    }

    fn handle_msg(&mut self, msg:Messages) {
        match msg {
            Messages::Tick => self.tick(),
            _ => return, 
        }
    }

    fn tick(&mut self) {
        self.x += 1;
        self.y += 1;

        match self.broker.try_send(Messages::BallPos(self.x, self.y)) {
            Ok(_) => println!("ball pos message sent"),
            Err(_err) => println!("ball error sending pos message"),
        }
    }
}
