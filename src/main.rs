mod paddle;
mod command;
mod ball;
mod game;
mod physics;
mod ipc;

use std::{thread};
use crossbeam::channel::unbounded;
use crate::ipc::Messages;

fn main() {
    let (s, r) = unbounded();

    // Make a ball and give ownership to the thread 
    let mut ball = ball::Ball::new();
    let _handler = thread::spawn(move || {
        ball.poll(r);
    });

    loop {
        match s.try_send(Messages::TickMsg) {
            Ok(_) => println!("tick message sent"),
            Err(_err) => println!("error sending message"),
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
