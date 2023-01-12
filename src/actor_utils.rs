// Enum defining all message types for actor_utils between actors
#[derive(Clone)]
pub enum Messages {
    TickMsg,
    DrawMsg,
    BoardSizeMsg(i16, i16)
}

pub trait Actor {
    fn poll (&self, r: crossbeam::channel::Receiver<Messages>);
}
