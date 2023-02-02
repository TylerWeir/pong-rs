use crate::utils::sprite::Sprite;
//
// Enum defining all message types for actor_utils between actors
//
#[derive(Clone)]
pub enum Messages {
    Tick,
    BallPos(i32, i32),
    Draw(Point, Sprite),
    ScreenSize(i32, i32),
}

pub trait Actor {
    fn poll (&mut self, r: crossbeam::channel::Receiver<Messages>);
}

//
// Point struct for position representations 
//
#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }
}
