use crate::actor_utils::Actor;
use crate::actor_utils::Messages;

pub struct Broker {
   members: Vec<crossbeam::channel::Sender<Messages>>
}


impl Actor for Broker {
    fn poll(&mut self, r: crossbeam::channel::Receiver<Messages>) {
        loop {
            match r.recv() {
                Ok(msg) => self.pass_along(msg),
                Err(_err) => panic!("Broker failed to receive a message"),
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
                Ok(_) => (),
                Err(_err) => panic!("Broker failed to send a message"),
            }
        }
    }
}
