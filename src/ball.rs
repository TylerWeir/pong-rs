use crate::command::Moveable;
use crate::physics::SolidBody;

extern crate ncurses;
use ncurses::*;

pub struct Ball {
    x: i16,
    y: i16,
    vx: i16,
    vy: i16,
    size: i16,
    window: WINDOW,
}

impl SolidBody for Ball {
    fn update(&mut self, delta_ms: i16) {
        self.x = self.x + delta_ms/100*self.vx;
        self.y = self.y + delta_ms/100*self.vy;

        // Update x velocities
        if (self.x + self.size) as i32 > getmaxx(self.window) {
            self.x = getmaxx(self.window) as i16 - self.size;
            self.vx = self.vx*(-1);
        }

        if self.x < 0 {
            self.x = 0;
            self.vx = self.vx*(-1);
        }
        
        // Update y velocities
        if (self.y + self.size) as i32 > getmaxy(self.window) {
            self.vy = self.vy*(-1);
            self.y = getmaxy(self.window) as i16 - self.size;
        }

        if self.y < 0 {
            self.y = 0;
            self.vy = self.vy*(-1);
        }
    }
}

impl Moveable for Ball {
    fn add_x(&mut self, value: i16) {
        self.x = self.x + value;
    }

    fn add_y(&mut self, value: i16) {
        self.y = self.y + value;
    }
}

impl Ball {

    pub fn new(window: WINDOW) -> Ball {
        Ball {
         x: 0,
         y: 0,
         vx: 1,
         vy: 1,
         size: 2,
         window: window,
        }
    }

    pub fn draw(&self) {
        mvaddstr(self.y as i32, self.x as i32, "##");
        mvaddstr(self.y as i32 + 1, self.x as i32, "##");
    }
}
