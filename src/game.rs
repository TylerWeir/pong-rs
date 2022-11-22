// Represents a game of pong played between two paddles 

pub trait Transitions {
    fn next(&self);
    fn back(&self);
}

pub struct start {}

pub struct game {}

pub struct paused {}

pub struct end {}
