extern crate ncurses;
use crate::actor_utils::Point;

static NEW_LINE: &str = "\n";

#[derive(Copy)]
#[derive(Clone)]
pub struct Sprite {
    width: i32,
    height: i32,
    sprite: &'static str,
}

impl Sprite {
    pub fn draw(&self, pos:Point, _win: ncurses::WINDOW) {
        for (i, val) in self.sprite.split(NEW_LINE).enumerate()  {
            let y = pos.y + i as i32;
            ncurses::mvaddstr(y.into(), pos.x.into(), val);
        }
    }

    pub fn new (w:i32, h:i32, s:&'static str) -> Sprite {
        Sprite {width:w, height:h, sprite:s}
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }
}
