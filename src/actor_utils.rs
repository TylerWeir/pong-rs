use crate::utils::sprite::Sprite;
//
// Enum defining all message types for actor_utils between actors
//
#[derive(Clone)]
pub enum Messages {
    Tick,
    BallPos(i32, i32),
    Draw(Point, Sprite),
    UpCmd,
    DownCmd,
    Terminate,
    ScreenSize(i32, i32),
}

pub trait Actor {
    fn poll (&mut self, r: crossbeam::channel::Receiver<Messages>);
}

//
// Point struct for position representations 
//
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }

    pub fn add_x(&mut self, x: i32) {
        self.x += x;
    }

    pub fn add_y(&mut self, y: i32) {
        self.y += y;
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_x() {
        let mut my_point = Point::new(0,0);        

        my_point.add_x(5);

        assert_eq!(my_point, Point::new(5,0));
    }

    #[test]
    fn test_add_y() {
        let mut my_point = Point::new(0,0);        

        my_point.add_y(5);

        assert_eq!(my_point, Point::new(0,5));
    }

}
