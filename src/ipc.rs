// Enum defining all message types for ipc between actors
#[derive(Clone)]
pub enum Messages {
    Tick,
    BallPos(i16, i16),
}

pub trait Actor {
    fn poll (&mut self, r: crossbeam::channel::Receiver<Messages>);
}
