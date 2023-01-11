use crate::command::Moveable;
use crate::physics::SolidBody;
use crate::ipc::Messages;
use crate::ipc::Actor;

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

impl Actor for Ball {
    fn poll(&self, r: crossbeam::channel::Receiver<Messages>) {
        println!("ball is polling for messages...");

        loop {
            match r.recv() {
                Ok(_msg) => println!("ball receiving a message!"),
                Err(_err) => println!("ball experiencing errors!"),
            }
        }               
    }
}


impl Ball {
    pub fn new(_s: crossbeam::channel::Sender<Messages>) -> Ball {
        Ball {
         x: 0,
         y: 0,
         vx: 1,
         vy: 1,
        }
    }

}
