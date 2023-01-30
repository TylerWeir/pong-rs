extern crate ncurses;
use crate::actor_utils::Point;

#[derive(Copy)]
#[derive(Clone)]
pub struct Sprite {
    sprite: &'static str,
}

impl Sprite {
    pub fn draw(&self, pos:Point, _win: ncurses::WINDOW) {
        ncurses::mvaddstr(pos.y.into(), pos.x.into(), self.sprite);
    }

    pub fn new (s:&'static str) -> Sprite {
        Sprite {sprite:s}
    }
}
