use crate::command::Moveable;
use crate::physics::SolidBody;
use crate::actor_utils::Messages;
use crate::actor_utils::Actor;

pub struct Ball {
    x: i16,
    y: i16,
    vx: i16,
    vy: i16,
    sender: crossbeam::channel::Sender<Messages>,
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
                Ok(msg) => self.handle_messages(msg),
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
         sender: s,
        }
    }

    pub fn handle_messages(&self, msg: Messages) {
        match msg {
            Messages::TickMsg => println!("ball recieved a tick message"),
            Messages::BoardSizeMsg(x, y) => println!("Ball received board size {} {}", x, y),
            Messages::DrawMsg => println!("ball received a draw message"),
        } 
    }
}
