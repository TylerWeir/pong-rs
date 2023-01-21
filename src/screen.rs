extern crate ncurses;

use ncurses::*;

use crate::actor_utils::Messages;
use crate::actor_utils::Actor;
use crate::ball::Ball;

pub struct Screen {
    ball_x: i16,
    ball_y: i16,
}

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
            Messages::BallPos(x, y) => self.update_ball(x, y),
        }
    }

    // Creates a new screen with zero valued fields
    pub fn new(_s: crossbeam::channel::Sender<Messages>) -> Screen {

        /* Setup ncurses */
        ncurses::initscr();

        ncurses::clear();
        ncurses::mvaddstr(10, 10, "hello there");
        ncurses::refresh();

        Screen {
            ball_x: 0,
            ball_y: 0,
        }
    }

    pub fn update_ball(&mut self, x: i16, y: i16) {
        self.ball_x = x;
        self.ball_y = y;
    }

    pub fn paint(&mut self) {
        ncurses::clear();
        Ball::draw(self.ball_x, self.ball_y);
        refresh();
    }
}

