use crate::command::Moveable;
use crate::utils::sprite::Sprite;
use crate::actor_utils::Point;
use crate::actor_utils::Messages;
use crate::actor_utils::Actor;

pub struct Paddle {
    x: i16,
    y: i16,
    sprite: Sprite,
    broker: crossbeam::channel::Sender<Messages>,
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
    fn poll (&mut self, r: crossbeam::channel::Receiver<Messages>) {
        println!("paddle is polling for messages...");

        loop {
            match r.recv() {
                Ok(msg) => self.handle_message(msg),
                Err(_err) => println!("paddle experiencing errors"),
            }
        }
    }
}

impl Paddle {
    // Creates a new paddle with zero valued fields
    pub fn new(s: crossbeam::channel::Sender<Messages>) -> Paddle {
        Paddle {
            x: 5,
            y: 5,
            sprite: Sprite::new("##\n##\n##\n##\n##\n##t"),
            broker: s
        }
    }

    fn tick(&self) {
        match self.broker.try_send(Messages::Draw(Point::new(self.x, self.y), self.sprite)) {
            Ok(_) => println!("draw send"),
            Err(_err) => println!("draw not send"),
        }
    }

    fn handle_message(&mut self, msg:Messages) {
        match msg {
            Messages::Tick => self.tick(),
            _ => return, 
        }
    }
}

