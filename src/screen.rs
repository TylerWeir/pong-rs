extern crate ncurses;

use ncurses::*;

use crate::actor_utils::Messages;
use crate::actor_utils::Actor;
use crate::actor_utils::Point;
use crate::utils::sprite::Sprite;

pub struct Screen {}

impl Actor for Screen {
    fn poll (&mut self, r: crossbeam::channel::Receiver<Messages>) {
        println!("screen is polling for messages...");

        loop {
            match r.recv() {
                Ok(msg) => self.handle_msg(msg.clone()) ,
                Err(_err) => println!("Screen experiencing errors"),
            }
        }
    }
}

impl Screen {

    pub fn handle_msg(&mut self, msg:Messages) {
        match msg { 
            Messages::Tick => self.paint(),
            Messages::Draw(pos, sprite) => self.draw(pos, sprite),
            _ => return,
        }
    }

    // Creates a new screen with zero valued fields
    pub fn new(_s: crossbeam::channel::Sender<Messages>) -> Screen {

        /* Setup ncurses */
        ncurses::initscr();

        ncurses::clear();
        ncurses::mvaddstr(10, 10, "hello there");
        ncurses::refresh();

        Screen {}
    }

    pub fn paint(&mut self) {
        refresh();
        ncurses::clear();
    }

    pub fn draw(&self, p:Point, sprite:Sprite) {
        sprite.draw(p, ncurses::curscr());
    }
}

