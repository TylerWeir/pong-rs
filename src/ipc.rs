// Enum defining all message types for ipc between actors
pub enum Messages {
    TickMsg,
}

pub trait Actor {
    fn poll (&self, r: crossbeam::channel::Receiver<Messages>);
}
