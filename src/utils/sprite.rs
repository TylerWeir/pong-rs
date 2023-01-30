extern crate ncurses;
use crate::actor_utils::Point;

static NEW_LINE: &str = "\n";

#[derive(Copy)]
#[derive(Clone)]
pub struct Sprite {
    sprite: &'static str,
}

impl Sprite {
    pub fn draw(&self, pos:Point, _win: ncurses::WINDOW) {
        for (i, val) in self.sprite.split(NEW_LINE).enumerate()  {
            let y = pos.y + i as i16;
            ncurses::mvaddstr(y.into(), pos.x.into(), val);
        }
    }

    pub fn new (s:&'static str) -> Sprite {
        Sprite {sprite:s}
    }
}
