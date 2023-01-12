use crate::ipc::Messages;
use crate::ipc::Actor;

extern crate ncurses;
use ncurses::*;

pub struct Board {
    width: i16,
    height: i16,
    sender: crossbeam::channel::Sender<Messages>, 
}

impl Actor for Board {
    fn poll(&self, r: crossbeam::channel::Receiver<Messages>) {
        println!("Board is polling for messages...");

        loop {
            match r.recv() {
                Ok(_msg) => println!("board receiving a message!"),
                Err(_err) => println!("board experiencing errors"),
            }
        }
    }
}

impl Board {
    pub fn new(s: crossbeam::channel::Sender<Messages>) -> Board {

        /* TODO: this needs to move */
        /* Start ncurses */
        initscr();

        /* Get screen bounds */
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);

        /* Tell the world how big we are */
        match s.try_send(Messages::BoardSizeMsg(max_x as i16, max_y as i16)) {
            Ok(_) => println!("Board broadcasted its size"),
            Err(_err) => println!("Board error sending message"),
        }

        Board {
            width: max_x as i16,
            height: max_y as i16,
            sender: s,
        }
    }
}
