use crate::actor_utils::Messages;
use crate::actor_utils::Actor;

pub struct Input {
    s: crossbeam::channel::Sender<Messages>,
}

impl Actor for Input {
    fn poll (&mut self, _r: crossbeam::channel::Receiver<Messages>) {
        println!("input is polling for messages...");
        loop {

        }
    }
}

impl Input {
    // Creates a new paddle with zero valued fields
    pub fn new(s: crossbeam::channel::Sender<Messages>) -> Input {
        Input {s}
    }
}
