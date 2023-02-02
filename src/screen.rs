extern crate ncurses;

use ncurses::*;

use crate::actor_utils::Messages;
use crate::actor_utils::Actor;
use crate::actor_utils::Point;
use crate::utils::sprite::Sprite;

pub struct Screen {
    max_x: i32,
    max_y: i32,
    s: crossbeam::channel::Sender<Messages>,
}

impl Actor for Screen {
    fn poll (&mut self, r: crossbeam::channel::Receiver<Messages>) {
        loop {
            match r.recv() {
                Ok(msg) => self.handle_msg(msg.clone()) ,
                Err(_err) => panic!("Screen failed to receive a message"),
            }
        }
    }
}

impl Screen {

    pub fn handle_msg(&mut self, msg:Messages) {
        match msg { 
            Messages::Tick => self.tick(),
            Messages::Draw(pos, sprite) => self.draw(pos, sprite),
            _ => return,
        }
    }

    // Creates a new screen with zero valued fields
    pub fn new(broker: crossbeam::channel::Sender<Messages>) -> Screen {

        /* Setup ncurses */
        ncurses::initscr();

        ncurses::clear();
        ncurses::mvaddstr(10, 10, "hello there");
        ncurses::refresh();

        let mut x: i32 = 0;
        let mut y: i32 = 0;
        ncurses::getmaxyx(ncurses::curscr(), &mut y, &mut x);
        let screen = Screen {max_x: x, max_y: y, s:broker};

        screen.tell_size();    // TODO How to detect resizeing??

        screen
    }

    pub fn tick(&mut self) {
        if self.check_resized() {
            // Tell everyone the new size and throw out old draw requests
            self.update_size();
            self.tell_size();
            ncurses::clear();
        } else {
            self.paint();
        }
    }

    pub fn paint(&mut self) {
        refresh();
        ncurses::clear();
    }

    pub fn draw(&self, p:Point, sprite:Sprite) {
        sprite.draw(p, ncurses::curscr());
    }

    pub fn check_resized(&self) -> bool {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        ncurses::getmaxyx(ncurses::curscr(), &mut y,&mut x);

        self.max_x != x || self.max_y != y
    }

    pub fn update_size(&mut self) {
        ncurses::getmaxyx(ncurses::curscr(), &mut self.max_y,&mut self.max_x);
    }

    pub fn tell_size(&self) {
        match self.s.try_send(Messages::ScreenSize(self.max_x, self.max_y)) {
            Ok(_) => (), 
            Err(_) => (),
        }
    }
}

