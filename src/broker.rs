use crate::actor_utils::Actor;
use crate::actor_utils::Messages;

pub struct Broker {
   members: Vec<crossbeam::channel::Sender<Messages>>
}


impl Actor for Broker {
    fn poll(&self, r: crossbeam::channel::Receiver<Messages>) {
        println!("Broker is read to pass along messages...");

        loop {
            match r.recv() {
                Ok(msg) => self.pass_along(msg),
                Err(_err) => println!("Broker had error"),
            }
        }
    }
}

impl Broker {

    pub fn new(v: Vec<crossbeam::channel::Sender<Messages>>) -> Broker {
        Broker {
            members: v,
        }
    }

    fn pass_along(&self, msg: Messages) {
        for s in &self.members {
            match s.try_send(msg.clone()) {
                Ok(_) => println!("broker passed along a message"),
                Err(_err) => println!("broker had an error passing the message"),
            }
        }
    }
}
