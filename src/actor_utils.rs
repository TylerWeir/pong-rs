use crate::utils::sprite::Sprite;
//
// Enum defining all message types for actor_utils between actors
//
#[derive(Clone)]
pub enum Messages {
    Tick,
    BallPos(i16, i16),
    Draw(Point, Sprite),
}

pub trait Actor {
    fn poll (&mut self, r: crossbeam::channel::Receiver<Messages>);
}

//
// Point struct for position representations 
//
#[derive(Clone)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Point {
        Point {x, y}
    }
}
