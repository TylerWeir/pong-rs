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
    fn update(&mut self, delta_ms: i16) {
        self.x = self.x + delta_ms/100*self.vx;
        self.y = self.y + delta_ms/100*self.vy;
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
    
    pub fn poll(&self, r: crossbeam::channel::Receiver<Messages>) {
        println!("ball is polling for messages...");

        loop {
            match r.recv() {
                Ok(_msg) => println!("receiving a message!"),
                Err(_err) => println!("experiencing errors!"),
            }
        }               
    }
}
