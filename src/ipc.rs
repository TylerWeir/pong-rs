// Enum defining all message types for ipc between actors
#[derive(Clone)]
pub enum Messages {
    TickMsg,
    DrawMsg,
    BoardSizeMsg(i16, i16)
}

pub trait Actor {
    fn poll (&self, r: crossbeam::channel::Receiver<Messages>);
}
