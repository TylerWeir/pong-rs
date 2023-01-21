// Enum defining all message types for actor_utils between actors
#[derive(Clone)]
pub enum Messages {
    Tick,
    BallPos(i16, i16),
}

pub trait Actor {
    fn poll (&mut self, r: crossbeam::channel::Receiver<Messages>);
}
