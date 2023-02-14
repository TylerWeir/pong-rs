use crate::utils::sprite::Sprite;
//
// Enum defining all message types for actor_utils between actors
//
#[derive(Clone)]
pub enum Messages {
    Tick,
    BallPos(i16, i16),
    Draw(Point, Sprite),
    UpCmd,
    DownCmd,
    Terminate,
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
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Point {
        Point {x, y}
    }

    pub fn add_x(&mut self, x: i16) {
        self.x += x;
    }

    pub fn add_y(&mut self, y: i16) {
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
