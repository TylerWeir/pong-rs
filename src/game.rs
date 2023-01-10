// Represents a game of pong played between two paddles 

pub trait Transitions {
    fn next(&self);
    fn back(&self);
}
